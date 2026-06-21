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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave97-live-heavy-gate-release-claim-anti-equivocation-seal-audit-security-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-root-only-records";
pub const SEAL_SUITE: &str =
    "wave97-wave96-release-readiness-root-to-anti-equivocation-release-claim-seal-v1";
pub const DEFAULT_WAVE: u64 = 97;
pub const DEFAULT_SOURCE_WAVE: u64 = 96;
pub const DEFAULT_HEIGHT: u64 = 4_282_097;
pub const DEFAULT_MIN_SLOT_COUNT: u64 = 6;
pub const DEFAULT_MAX_RAW_PAYLOAD_RECORDS: u64 = 0;
pub const DEFAULT_MAX_SEALED_LANES: u64 = 0;
pub const DEFAULT_MAX_RELEASED_CLAIMS: u64 = 0;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneSlotKind {
    AuditReview,
    AdversarialScenario,
    ThreatModel,
    PrivacyReview,
    ReviewerSignoff,
    OperatorSignoff,
}

impl LaneSlotKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AuditReview => "audit_review",
            Self::AdversarialScenario => "adversarial_scenario",
            Self::ThreatModel => "threat_model",
            Self::PrivacyReview => "privacy_review",
            Self::ReviewerSignoff => "reviewer_signoff",
            Self::OperatorSignoff => "operator_signoff",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::AuditReview,
            Self::AdversarialScenario,
            Self::ThreatModel,
            Self::PrivacyReview,
            Self::ReviewerSignoff,
            Self::OperatorSignoff,
        ]
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SealStatus {
    BlockedEmpty,
    BlockedAntiEquivocation,
    BlockedReleaseDenied,
    Sealed,
}

impl SealStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BlockedEmpty => "blocked_empty",
            Self::BlockedAntiEquivocation => "blocked_anti_equivocation",
            Self::BlockedReleaseDenied => "blocked_release_denied",
            Self::Sealed => "sealed",
        }
    }

    pub fn is_sealed(self) -> bool {
        self == Self::Sealed
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AntiEquivocationBlockerKind {
    ReleaseReadinessRootMissing,
    ReleaseClaimRootMissing,
    AuditCheckpointRootMissing,
    ReplayGuardActive,
    ForkGuardActive,
    DuplicateGuardActive,
    PriorSealRootMissing,
    ConflictingSealRootPresent,
    ReleaseClaimNotSealed,
    ReleaseDenied,
    SealedLaneBudgetZero,
    ProductionDenied,
    FailClosedDisarmed,
    RawPayloadPresent,
    HeavyGateRunClaimed,
}

impl AntiEquivocationBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseReadinessRootMissing => "release_readiness_root_missing",
            Self::ReleaseClaimRootMissing => "release_claim_root_missing",
            Self::AuditCheckpointRootMissing => "audit_checkpoint_root_missing",
            Self::ReplayGuardActive => "replay_guard_active",
            Self::ForkGuardActive => "fork_guard_active",
            Self::DuplicateGuardActive => "duplicate_guard_active",
            Self::PriorSealRootMissing => "prior_seal_root_missing",
            Self::ConflictingSealRootPresent => "conflicting_seal_root_present",
            Self::ReleaseClaimNotSealed => "release_claim_not_sealed",
            Self::ReleaseDenied => "release_denied",
            Self::SealedLaneBudgetZero => "sealed_lane_budget_zero",
            Self::ProductionDenied => "production_denied",
            Self::FailClosedDisarmed => "fail_closed_disarmed",
            Self::RawPayloadPresent => "raw_payload_present",
            Self::HeavyGateRunClaimed => "heavy_gate_run_claimed",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandHintKind {
    HoldRelease,
    KeepFailClosed,
    ImportWave96ReadinessRoot,
    AttachReleaseClaimRoot,
    AttachAuditCheckpointRoot,
    ReviewReplayGuard,
    ReviewForkGuard,
    ReviewDuplicateGuard,
    SealOnlyAfterBlockersClear,
}

impl CommandHintKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HoldRelease => "hold_release",
            Self::KeepFailClosed => "keep_fail_closed",
            Self::ImportWave96ReadinessRoot => "import_wave96_readiness_root",
            Self::AttachReleaseClaimRoot => "attach_release_claim_root",
            Self::AttachAuditCheckpointRoot => "attach_audit_checkpoint_root",
            Self::ReviewReplayGuard => "review_replay_guard",
            Self::ReviewForkGuard => "review_fork_guard",
            Self::ReviewDuplicateGuard => "review_duplicate_guard",
            Self::SealOnlyAfterBlockersClear => "seal_only_after_blockers_clear",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub seal_suite: String,
    pub wave: u64,
    pub source_wave: u64,
    pub current_height: u64,
    pub min_slot_count: u64,
    pub max_raw_payload_records: u64,
    pub max_sealed_lanes: u64,
    pub max_released_claims: u64,
    pub source_release_readiness_root: String,
    pub source_release_claim_root: String,
    pub source_quorum_blocker_root: String,
    pub anti_equivocation_anchor_root: String,
    pub fail_closed_armed: bool,
    pub anti_equivocation_blockers_active: bool,
    pub release_allowed: bool,
    pub production_allowed: bool,
    pub heavy_gates_ran: bool,
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
            seal_suite: SEAL_SUITE.to_string(),
            wave: DEFAULT_WAVE,
            source_wave: DEFAULT_SOURCE_WAVE,
            current_height: DEFAULT_HEIGHT,
            min_slot_count: DEFAULT_MIN_SLOT_COUNT,
            max_raw_payload_records: DEFAULT_MAX_RAW_PAYLOAD_RECORDS,
            max_sealed_lanes: DEFAULT_MAX_SEALED_LANES,
            max_released_claims: DEFAULT_MAX_RELEASED_CLAIMS,
            source_release_readiness_root: deterministic_root("wave96-release-readiness-root"),
            source_release_claim_root: deterministic_root("wave96-release-claim-root"),
            source_quorum_blocker_root: deterministic_root("wave96-quorum-blocker-root"),
            anti_equivocation_anchor_root: empty_root("wave97-anti-equivocation-anchor"),
            fail_closed_armed: true,
            anti_equivocation_blockers_active: true,
            release_allowed: false,
            production_allowed: false,
            heavy_gates_ran: false,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("chain_id", &self.chain_id)?;
        ensure_non_empty("protocol_version", &self.protocol_version)?;
        ensure_non_empty("hash_suite", &self.hash_suite)?;
        ensure_non_empty("seal_suite", &self.seal_suite)?;
        ensure_positive("wave", self.wave)?;
        ensure_positive("source_wave", self.source_wave)?;
        ensure_positive("current_height", self.current_height)?;
        ensure_positive("min_slot_count", self.min_slot_count)?;
        ensure_root(
            "source_release_readiness_root",
            &self.source_release_readiness_root,
        )?;
        ensure_root("source_release_claim_root", &self.source_release_claim_root)?;
        ensure_root(
            "source_quorum_blocker_root",
            &self.source_quorum_blocker_root,
        )?;
        ensure_root(
            "anti_equivocation_anchor_root",
            &self.anti_equivocation_anchor_root,
        )?;
        if !self.fail_closed_armed {
            return Err("wave97 seal lane fail closed is not armed".to_string());
        }
        if !self.anti_equivocation_blockers_active {
            return Err("wave97 anti equivocation blockers are not active".to_string());
        }
        if self.release_allowed {
            return Err("wave97 release claim seal denies release by default".to_string());
        }
        if self.production_allowed {
            return Err("wave97 release claim seal denies production by default".to_string());
        }
        if self.heavy_gates_ran {
            return Err("wave97 release claim seal cannot claim gate execution".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave97_release_claim_anti_equivocation_seal_config",
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "seal_suite": self.seal_suite,
            "wave": self.wave,
            "source_wave": self.source_wave,
            "current_height": self.current_height,
            "min_slot_count": self.min_slot_count,
            "max_raw_payload_records": self.max_raw_payload_records,
            "max_sealed_lanes": self.max_sealed_lanes,
            "max_released_claims": self.max_released_claims,
            "source_release_readiness_root": self.source_release_readiness_root,
            "source_release_claim_root": self.source_release_claim_root,
            "source_quorum_blocker_root": self.source_quorum_blocker_root,
            "anti_equivocation_anchor_root": self.anti_equivocation_anchor_root,
            "fail_closed_armed": self.fail_closed_armed,
            "anti_equivocation_blockers_active": self.anti_equivocation_blockers_active,
            "release_allowed": self.release_allowed,
            "production_allowed": self.production_allowed,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE97-CONFIG", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub source_readiness_roots: u64,
    pub release_claim_roots: u64,
    pub audit_checkpoint_roots: u64,
    pub replay_guard_roots: u64,
    pub fork_guard_roots: u64,
    pub duplicate_guard_roots: u64,
    pub blocker_count: u64,
    pub command_hint_count: u64,
    pub sealed_lanes: u64,
    pub released_claims: u64,
    pub raw_payload_records: u64,
}

impl Counters {
    pub fn public_record(&self) -> PublicRecord {
        json!({
            "source_readiness_roots": self.source_readiness_roots,
            "release_claim_roots": self.release_claim_roots,
            "audit_checkpoint_roots": self.audit_checkpoint_roots,
            "replay_guard_roots": self.replay_guard_roots,
            "fork_guard_roots": self.fork_guard_roots,
            "duplicate_guard_roots": self.duplicate_guard_roots,
            "blocker_count": self.blocker_count,
            "command_hint_count": self.command_hint_count,
            "sealed_lanes": self.sealed_lanes,
            "released_claims": self.released_claims,
            "raw_payload_records": self.raw_payload_records,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleaseClaimRoot {
    pub slot_kind: LaneSlotKind,
    pub source_readiness_root: String,
    pub release_claim_root: String,
    pub audit_checkpoint_root: String,
    pub seal_candidate_root: String,
    pub status: SealStatus,
}

impl ReleaseClaimRoot {
    pub fn blocked(slot_kind: LaneSlotKind, config: &Config) -> Self {
        let slot = slot_kind.as_str();
        let source_readiness_root = bind_root(
            "release-readiness-source",
            slot,
            &config.source_release_readiness_root,
        );
        let release_claim_root = bind_root(
            "release-claim-source",
            slot,
            &config.source_release_claim_root,
        );
        let audit_checkpoint_root = deterministic_root(&format!("{}-audit-checkpoint", slot));
        let seal_candidate_root = value_root(
            "WAVE97-SEAL-CANDIDATE",
            &json!({
                "slot_kind": slot,
                "source_readiness_root": source_readiness_root,
                "release_claim_root": release_claim_root,
                "audit_checkpoint_root": audit_checkpoint_root,
                "release_allowed": false,
            }),
        );
        Self {
            slot_kind,
            source_readiness_root,
            release_claim_root,
            audit_checkpoint_root,
            seal_candidate_root,
            status: SealStatus::BlockedAntiEquivocation,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root("source_readiness_root", &self.source_readiness_root)?;
        ensure_root("release_claim_root", &self.release_claim_root)?;
        ensure_root("audit_checkpoint_root", &self.audit_checkpoint_root)?;
        ensure_root("seal_candidate_root", &self.seal_candidate_root)?;
        if self.status.is_sealed() {
            return Err("devnet release claim root must not be sealed".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "slot_kind": self.slot_kind.as_str(),
            "source_readiness_root": self.source_readiness_root,
            "release_claim_root": self.release_claim_root,
            "audit_checkpoint_root": self.audit_checkpoint_root,
            "seal_candidate_root": self.seal_candidate_root,
            "status": self.status.as_str(),
            "sealed": false,
            "release_allowed": false,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AntiEquivocationGuard {
    pub slot_kind: LaneSlotKind,
    pub replay_guard_root: String,
    pub fork_guard_root: String,
    pub duplicate_guard_root: String,
    pub prior_seal_root: String,
    pub conflict_set_root: String,
    pub guard_active: bool,
}

impl AntiEquivocationGuard {
    pub fn active(slot_kind: LaneSlotKind, claim: &ReleaseClaimRoot, config: &Config) -> Self {
        let slot = slot_kind.as_str();
        let replay_guard_root = value_root(
            "WAVE97-REPLAY-GUARD",
            &json!({
                "slot_kind": slot,
                "source_readiness_root": claim.source_readiness_root,
                "release_claim_root": claim.release_claim_root,
                "anchor_root": config.anti_equivocation_anchor_root,
                "active": true,
            }),
        );
        let fork_guard_root = value_root(
            "WAVE97-FORK-GUARD",
            &json!({
                "slot_kind": slot,
                "source_quorum_blocker_root": config.source_quorum_blocker_root,
                "seal_candidate_root": claim.seal_candidate_root,
                "active": true,
            }),
        );
        let duplicate_guard_root = value_root(
            "WAVE97-DUPLICATE-GUARD",
            &json!({
                "slot_kind": slot,
                "release_claim_root": claim.release_claim_root,
                "audit_checkpoint_root": claim.audit_checkpoint_root,
                "active": true,
            }),
        );
        let prior_seal_root = empty_root(&format!("{}-prior-seal-root", slot));
        let conflict_set_root = value_root(
            "WAVE97-CONFLICT-SET",
            &json!({
                "slot_kind": slot,
                "prior_seal_root": prior_seal_root,
                "duplicate_guard_root": duplicate_guard_root,
                "conflict_detected": true,
            }),
        );
        Self {
            slot_kind,
            replay_guard_root,
            fork_guard_root,
            duplicate_guard_root,
            prior_seal_root,
            conflict_set_root,
            guard_active: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root("replay_guard_root", &self.replay_guard_root)?;
        ensure_root("fork_guard_root", &self.fork_guard_root)?;
        ensure_root("duplicate_guard_root", &self.duplicate_guard_root)?;
        ensure_root("prior_seal_root", &self.prior_seal_root)?;
        ensure_root("conflict_set_root", &self.conflict_set_root)?;
        if !self.guard_active {
            return Err("anti equivocation guard is not active".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "slot_kind": self.slot_kind.as_str(),
            "replay_guard_root": self.replay_guard_root,
            "fork_guard_root": self.fork_guard_root,
            "duplicate_guard_root": self.duplicate_guard_root,
            "prior_seal_root": self.prior_seal_root,
            "conflict_set_root": self.conflict_set_root,
            "guard_active": self.guard_active,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AntiEquivocationBlocker {
    pub kind: AntiEquivocationBlockerKind,
    pub slot_kind: LaneSlotKind,
    pub blocker_root: String,
}

impl AntiEquivocationBlocker {
    pub fn new(
        kind: AntiEquivocationBlockerKind,
        slot_kind: LaneSlotKind,
        evidence_root: &str,
    ) -> Self {
        Self {
            kind,
            slot_kind,
            blocker_root: value_root(
                "WAVE97-ANTI-EQUIVOCATION-BLOCKER",
                &json!({
                    "kind": kind.as_str(),
                    "slot_kind": slot_kind.as_str(),
                    "evidence_root": evidence_root,
                }),
            ),
        }
    }

    pub fn canonical(
        claims: &[ReleaseClaimRoot],
        guards: &[AntiEquivocationGuard],
        config: &Config,
    ) -> Vec<Self> {
        let mut blockers = Vec::new();
        for claim in claims {
            blockers.push(Self::new(
                AntiEquivocationBlockerKind::ReleaseDenied,
                claim.slot_kind,
                &claim.release_claim_root,
            ));
            blockers.push(Self::new(
                AntiEquivocationBlockerKind::ReleaseClaimNotSealed,
                claim.slot_kind,
                &claim.seal_candidate_root,
            ));
            blockers.push(Self::new(
                AntiEquivocationBlockerKind::SealedLaneBudgetZero,
                claim.slot_kind,
                &claim.audit_checkpoint_root,
            ));
        }
        for guard in guards {
            blockers.push(Self::new(
                AntiEquivocationBlockerKind::ReplayGuardActive,
                guard.slot_kind,
                &guard.replay_guard_root,
            ));
            blockers.push(Self::new(
                AntiEquivocationBlockerKind::ForkGuardActive,
                guard.slot_kind,
                &guard.fork_guard_root,
            ));
            blockers.push(Self::new(
                AntiEquivocationBlockerKind::DuplicateGuardActive,
                guard.slot_kind,
                &guard.duplicate_guard_root,
            ));
            blockers.push(Self::new(
                AntiEquivocationBlockerKind::ConflictingSealRootPresent,
                guard.slot_kind,
                &guard.conflict_set_root,
            ));
        }
        blockers.push(Self::new(
            AntiEquivocationBlockerKind::ProductionDenied,
            LaneSlotKind::OperatorSignoff,
            &config.source_quorum_blocker_root,
        ));
        blockers
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root("blocker_root", &self.blocker_root)
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": self.kind.as_str(),
            "slot_kind": self.slot_kind.as_str(),
            "blocker_root": self.blocker_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandHint {
    pub kind: CommandHintKind,
    pub hint_root: String,
}

impl CommandHint {
    pub fn new(kind: CommandHintKind, config: &Config) -> Self {
        Self {
            kind,
            hint_root: value_root(
                "WAVE97-COMMAND-HINT",
                &json!({
                    "kind": kind.as_str(),
                    "source_wave": config.source_wave,
                    "wave": config.wave,
                    "release_allowed": false,
                    "fail_closed_armed": config.fail_closed_armed,
                }),
            ),
        }
    }

    pub fn canonical(config: &Config) -> Vec<Self> {
        vec![
            Self::new(CommandHintKind::HoldRelease, config),
            Self::new(CommandHintKind::KeepFailClosed, config),
            Self::new(CommandHintKind::ImportWave96ReadinessRoot, config),
            Self::new(CommandHintKind::AttachReleaseClaimRoot, config),
            Self::new(CommandHintKind::AttachAuditCheckpointRoot, config),
            Self::new(CommandHintKind::ReviewReplayGuard, config),
            Self::new(CommandHintKind::ReviewForkGuard, config),
            Self::new(CommandHintKind::ReviewDuplicateGuard, config),
            Self::new(CommandHintKind::SealOnlyAfterBlockersClear, config),
        ]
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root("hint_root", &self.hint_root)
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": self.kind.as_str(),
            "hint_root": self.hint_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub release_claim_roots: Vec<ReleaseClaimRoot>,
    pub anti_equivocation_guards: Vec<AntiEquivocationGuard>,
    pub blockers: Vec<AntiEquivocationBlocker>,
    pub command_hints: Vec<CommandHint>,
    pub counters: Counters,
}

impl State {
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;
        let release_claim_roots = LaneSlotKind::all()
            .into_iter()
            .map(|slot_kind| ReleaseClaimRoot::blocked(slot_kind, &config))
            .collect::<Vec<_>>();
        let anti_equivocation_guards = release_claim_roots
            .iter()
            .map(|claim| AntiEquivocationGuard::active(claim.slot_kind, claim, &config))
            .collect::<Vec<_>>();
        let blockers = AntiEquivocationBlocker::canonical(
            &release_claim_roots,
            &anti_equivocation_guards,
            &config,
        );
        let command_hints = CommandHint::canonical(&config);
        let mut state = Self {
            config,
            release_claim_roots,
            anti_equivocation_guards,
            blockers,
            command_hints,
            counters: Counters::default(),
        };
        state.counters = state.compute_counters();
        state.validate()?;
        Ok(state)
    }

    pub fn compute_counters(&self) -> Counters {
        Counters {
            source_readiness_roots: self.release_claim_roots.len() as u64,
            release_claim_roots: self.release_claim_roots.len() as u64,
            audit_checkpoint_roots: self.release_claim_roots.len() as u64,
            replay_guard_roots: self.anti_equivocation_guards.len() as u64,
            fork_guard_roots: self.anti_equivocation_guards.len() as u64,
            duplicate_guard_roots: self.anti_equivocation_guards.len() as u64,
            blocker_count: self.blockers.len() as u64,
            command_hint_count: self.command_hints.len() as u64,
            sealed_lanes: self
                .release_claim_roots
                .iter()
                .filter(|claim| claim.status.is_sealed())
                .count() as u64,
            released_claims: 0,
            raw_payload_records: 0,
        }
    }

    pub fn release_claim_root(&self) -> String {
        collection_root(
            "WAVE97-RELEASE-CLAIM-ROOTS",
            self.release_claim_roots
                .iter()
                .map(ReleaseClaimRoot::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn audit_checkpoint_root(&self) -> String {
        collection_root(
            "WAVE97-AUDIT-CHECKPOINT-ROOTS",
            self.release_claim_roots
                .iter()
                .map(|claim| {
                    json!({
                        "slot_kind": claim.slot_kind.as_str(),
                        "audit_checkpoint_root": claim.audit_checkpoint_root,
                        "seal_candidate_root": claim.seal_candidate_root,
                    })
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn guard_root(&self) -> String {
        collection_root(
            "WAVE97-ANTI-EQUIVOCATION-GUARDS",
            self.anti_equivocation_guards
                .iter()
                .map(AntiEquivocationGuard::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn blocker_root(&self) -> String {
        collection_root(
            "WAVE97-ANTI-EQUIVOCATION-BLOCKERS",
            self.blockers
                .iter()
                .map(AntiEquivocationBlocker::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn command_hint_root(&self) -> String {
        collection_root(
            "WAVE97-COMMAND-HINTS",
            self.command_hints
                .iter()
                .map(CommandHint::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn seal_root(&self) -> String {
        value_root(
            "WAVE97-SEAL-ROOT",
            &json!({
                "release_claim_root": self.release_claim_root(),
                "audit_checkpoint_root": self.audit_checkpoint_root(),
                "guard_root": self.guard_root(),
                "blocker_root": self.blocker_root(),
                "sealed_lanes": 0,
                "release_allowed": false,
            }),
        )
    }

    pub fn state_material_root(&self) -> String {
        value_root(
            "WAVE97-STATE-MATERIAL",
            &json!({
                "config_root": self.config.state_root(),
                "release_claim_root": self.release_claim_root(),
                "audit_checkpoint_root": self.audit_checkpoint_root(),
                "guard_root": self.guard_root(),
                "blocker_root": self.blocker_root(),
                "command_hint_root": self.command_hint_root(),
                "seal_root": self.seal_root(),
                "counters": self.counters.public_record(),
                "release_allowed": false,
                "production_allowed": false,
                "heavy_gates_ran": false,
            }),
        )
    }

    pub fn state_root(&self) -> String {
        value_root(
            "WAVE97-STATE",
            &json!({
                "state_material_root": self.state_material_root(),
                "fail_closed_armed": self.config.fail_closed_armed,
                "anti_equivocation_blockers_active": self.config.anti_equivocation_blockers_active,
                "release_allowed": false,
                "sealed_lanes": 0,
                "released_claims": 0,
                "production_allowed": false,
                "heavy_gates_ran": false,
            }),
        )
    }

    pub fn release_denied(&self) -> bool {
        !self.config.release_allowed || !self.blockers.is_empty() || self.counters.sealed_lanes == 0
    }

    pub fn production_denied(&self) -> bool {
        !self.config.production_allowed || self.release_denied()
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        ensure_min_count(
            "slot count",
            self.release_claim_roots.len() as u64,
            self.config.min_slot_count,
        )?;
        for claim in &self.release_claim_roots {
            claim.validate()?;
        }
        for guard in &self.anti_equivocation_guards {
            guard.validate()?;
        }
        for blocker in &self.blockers {
            blocker.validate()?;
        }
        for command in &self.command_hints {
            command.validate()?;
        }
        if self.counters.raw_payload_records > self.config.max_raw_payload_records {
            return Err("wave97 seal lane contains raw payload records".to_string());
        }
        if self.counters.sealed_lanes > self.config.max_sealed_lanes {
            return Err("wave97 sealed lanes above configured limit".to_string());
        }
        if self.counters.released_claims > self.config.max_released_claims {
            return Err("wave97 released claims above configured limit".to_string());
        }
        if self.counters.sealed_lanes != 0 {
            return Err("devnet wave97 seal lane must not seal lanes".to_string());
        }
        if self.counters.released_claims != 0 {
            return Err("devnet wave97 seal lane must not release claims".to_string());
        }
        if self.compute_counters() != self.counters {
            return Err("wave97 seal counters do not match state".to_string());
        }
        if !self.release_denied() {
            return Err("wave97 release claim seal cannot allow release".to_string());
        }
        if !self.production_denied() {
            return Err("wave97 release claim seal cannot allow production".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave97_live_heavy_gate_release_claim_anti_equivocation_seal_audit_security_lane_state",
            "config": self.config.public_record(),
            "release_claim_root": self.release_claim_root(),
            "audit_checkpoint_root": self.audit_checkpoint_root(),
            "guard_root": self.guard_root(),
            "blocker_root": self.blocker_root(),
            "command_hint_root": self.command_hint_root(),
            "seal_root": self.seal_root(),
            "state_root": self.state_root(),
            "counters": self.counters.public_record(),
            "release_denied": self.release_denied(),
            "production_denied": self.production_denied(),
            "heavy_gates_ran": false,
            "release_claim_roots": self.release_claim_roots.iter().map(ReleaseClaimRoot::public_record).collect::<Vec<_>>(),
            "anti_equivocation_guards": self.anti_equivocation_guards.iter().map(AntiEquivocationGuard::public_record).collect::<Vec<_>>(),
            "blockers": self.blockers.iter().map(AntiEquivocationBlocker::public_record).collect::<Vec<_>>(),
            "command_hints": self.command_hints.iter().map(CommandHint::public_record).collect::<Vec<_>>(),
        })
    }
}

pub fn devnet() -> Runtime {
    match State::new(Config::devnet()) {
        Ok(runtime) => runtime,
        Err(error) => fallback_runtime(error),
    }
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn fallback_runtime(error: String) -> Runtime {
    let config = Config::devnet();
    let release_claim_roots = LaneSlotKind::all()
        .into_iter()
        .map(|slot_kind| ReleaseClaimRoot::blocked(slot_kind, &config))
        .collect::<Vec<_>>();
    let anti_equivocation_guards = release_claim_roots
        .iter()
        .map(|claim| AntiEquivocationGuard::active(claim.slot_kind, claim, &config))
        .collect::<Vec<_>>();
    let mut state = State {
        blockers: vec![AntiEquivocationBlocker::new(
            AntiEquivocationBlockerKind::ProductionDenied,
            LaneSlotKind::OperatorSignoff,
            &value_root(
                "WAVE97-FALLBACK-ERROR",
                &json!({"error_root": stable_id("fallback-error", &error)}),
            ),
        )],
        command_hints: CommandHint::canonical(&config),
        counters: Counters::default(),
        anti_equivocation_guards,
        release_claim_roots,
        config,
    };
    state.counters = state.compute_counters();
    state
}

fn ensure_non_empty(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(format!("{} is empty", field));
    }
    Ok(())
}

fn ensure_positive(field: &str, value: u64) -> Result<()> {
    if value == 0 {
        return Err(format!("{} must be positive", field));
    }
    Ok(())
}

fn ensure_min_count(field: &str, actual: u64, minimum: u64) -> Result<()> {
    if actual < minimum {
        return Err(format!("{} is below required minimum", field));
    }
    Ok(())
}

fn ensure_root(field: &str, value: &str) -> Result<()> {
    ensure_non_empty(field, value)?;
    if value.len() < 32 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(format!("{} is not a canonical root", field));
    }
    Ok(())
}

fn stable_id(domain: &str, label: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WAVE97-STABLE-ID",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(SEAL_SUITE),
            HashPart::Str(domain),
            HashPart::Str(label),
        ],
        32,
    )
}

fn deterministic_root(label: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WAVE97-DETERMINISTIC-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(SEAL_SUITE),
            HashPart::Str(label),
        ],
        32,
    )
}

fn empty_root(label: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WAVE97-EMPTY-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(SEAL_SUITE),
            HashPart::Str(label),
        ],
        32,
    )
}

fn bind_root(domain: &str, label: &str, source_root: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WAVE97-BIND-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(SEAL_SUITE),
            HashPart::Str(domain),
            HashPart::Str(label),
            HashPart::Str(source_root),
        ],
        32,
    )
}

fn value_root(domain: &str, value: &Value) -> String {
    domain_hash(
        domain,
        &[HashPart::Str(PROTOCOL_VERSION), HashPart::Json(value)],
        32,
    )
}

fn collection_root(domain: &str, values: Vec<Value>) -> String {
    merkle_root(domain, &values)
}
