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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave96-live-heavy-gate-receipt-release-readiness-quorum-audit-security-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-root-only-records";
pub const QUORUM_SUITE: &str =
    "wave96-wave95-promoted-slot-root-to-lane-release-readiness-quorum-v1";
pub const DEFAULT_WAVE: u64 = 96;
pub const DEFAULT_SOURCE_WAVE: u64 = 95;
pub const DEFAULT_HEIGHT: u64 = 4_281_904;
pub const DEFAULT_MIN_SLOT_COUNT: u64 = 6;
pub const DEFAULT_QUORUM_THRESHOLD: u64 = 6;
pub const DEFAULT_MAX_RAW_PAYLOAD_RECORDS: u64 = 0;
pub const DEFAULT_MAX_RELEASE_READY_LANES: u64 = 0;

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
pub enum ReadinessStatus {
    BlockedEmpty,
    PendingQuorum,
    ReleaseReady,
    Rejected,
}

impl ReadinessStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BlockedEmpty => "blocked_empty",
            Self::PendingQuorum => "pending_quorum",
            Self::ReleaseReady => "release_ready",
            Self::Rejected => "rejected",
        }
    }

    pub fn is_ready(self) -> bool {
        self == Self::ReleaseReady
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QuorumBlockerKind {
    PromotedSlotRootMissing,
    PromotedSlotRootNotBound,
    ReleaseClaimPlaceholder,
    ReviewerSignoffMissing,
    OperatorSignoffMissing,
    QuorumUnmet,
    ReleaseReadyBudgetZero,
    ProductionDenied,
    FailClosedDisarmed,
    RawPayloadPresent,
    HeavyGateRunClaimed,
}

impl QuorumBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PromotedSlotRootMissing => "promoted_slot_root_missing",
            Self::PromotedSlotRootNotBound => "promoted_slot_root_not_bound",
            Self::ReleaseClaimPlaceholder => "release_claim_placeholder",
            Self::ReviewerSignoffMissing => "reviewer_signoff_missing",
            Self::OperatorSignoffMissing => "operator_signoff_missing",
            Self::QuorumUnmet => "quorum_unmet",
            Self::ReleaseReadyBudgetZero => "release_ready_budget_zero",
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
    ImportWave95PromotedSlotRoot,
    BindPromotedSlotToLaneQuorum,
    ReviewQuorumBlockers,
    AttachReleaseClaimRoot,
    AttachReviewerSignoffRoot,
    AttachOperatorSignoffRoot,
    KeepFailClosed,
}

impl CommandHintKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HoldRelease => "hold_release",
            Self::ImportWave95PromotedSlotRoot => "import_wave95_promoted_slot_root",
            Self::BindPromotedSlotToLaneQuorum => "bind_promoted_slot_to_lane_quorum",
            Self::ReviewQuorumBlockers => "review_quorum_blockers",
            Self::AttachReleaseClaimRoot => "attach_release_claim_root",
            Self::AttachReviewerSignoffRoot => "attach_reviewer_signoff_root",
            Self::AttachOperatorSignoffRoot => "attach_operator_signoff_root",
            Self::KeepFailClosed => "keep_fail_closed",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub quorum_suite: String,
    pub wave: u64,
    pub source_wave: u64,
    pub current_height: u64,
    pub min_slot_count: u64,
    pub quorum_threshold: u64,
    pub max_raw_payload_records: u64,
    pub max_release_ready_lanes: u64,
    pub source_promotion_root: String,
    pub source_promoted_slot_root: String,
    pub source_promotion_blocker_root: String,
    pub release_claim_placeholder_root: String,
    pub fail_closed_armed: bool,
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
            quorum_suite: QUORUM_SUITE.to_string(),
            wave: DEFAULT_WAVE,
            source_wave: DEFAULT_SOURCE_WAVE,
            current_height: DEFAULT_HEIGHT,
            min_slot_count: DEFAULT_MIN_SLOT_COUNT,
            quorum_threshold: DEFAULT_QUORUM_THRESHOLD,
            max_raw_payload_records: DEFAULT_MAX_RAW_PAYLOAD_RECORDS,
            max_release_ready_lanes: DEFAULT_MAX_RELEASE_READY_LANES,
            source_promotion_root: deterministic_root("wave95-promotion-root"),
            source_promoted_slot_root: deterministic_root("wave95-promoted-slot-root"),
            source_promotion_blocker_root: deterministic_root("wave95-promotion-blocker-root"),
            release_claim_placeholder_root: empty_root("wave96-release-claim-placeholder"),
            fail_closed_armed: true,
            production_allowed: false,
            heavy_gates_ran: false,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("chain_id", &self.chain_id)?;
        ensure_non_empty("protocol_version", &self.protocol_version)?;
        ensure_non_empty("hash_suite", &self.hash_suite)?;
        ensure_non_empty("quorum_suite", &self.quorum_suite)?;
        ensure_positive("wave", self.wave)?;
        ensure_positive("source_wave", self.source_wave)?;
        ensure_positive("current_height", self.current_height)?;
        ensure_positive("min_slot_count", self.min_slot_count)?;
        ensure_positive("quorum_threshold", self.quorum_threshold)?;
        ensure_root("source_promotion_root", &self.source_promotion_root)?;
        ensure_root("source_promoted_slot_root", &self.source_promoted_slot_root)?;
        ensure_root(
            "source_promotion_blocker_root",
            &self.source_promotion_blocker_root,
        )?;
        ensure_root(
            "release_claim_placeholder_root",
            &self.release_claim_placeholder_root,
        )?;
        if !self.fail_closed_armed {
            return Err("release readiness quorum lane fail closed is not armed".to_string());
        }
        if self.production_allowed {
            return Err("wave96 release readiness lane denies production by default".to_string());
        }
        if self.heavy_gates_ran {
            return Err("wave96 release readiness lane cannot claim gate execution".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave96_release_readiness_quorum_config",
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "quorum_suite": self.quorum_suite,
            "wave": self.wave,
            "source_wave": self.source_wave,
            "current_height": self.current_height,
            "min_slot_count": self.min_slot_count,
            "quorum_threshold": self.quorum_threshold,
            "max_raw_payload_records": self.max_raw_payload_records,
            "max_release_ready_lanes": self.max_release_ready_lanes,
            "source_promotion_root": self.source_promotion_root,
            "source_promoted_slot_root": self.source_promoted_slot_root,
            "source_promotion_blocker_root": self.source_promotion_blocker_root,
            "release_claim_placeholder_root": self.release_claim_placeholder_root,
            "fail_closed_armed": self.fail_closed_armed,
            "production_allowed": self.production_allowed,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE96-CONFIG", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PromotedSlotRoot {
    pub slot_kind: LaneSlotKind,
    pub promoted_slot_root: String,
    pub source_promotion_root: String,
    pub binding_root: String,
}

impl PromotedSlotRoot {
    pub fn placeholder(slot_kind: LaneSlotKind, config: &Config) -> Self {
        let mut root = Self {
            slot_kind,
            promoted_slot_root: empty_root(&format!("{}-promoted-slot", slot_kind.as_str())),
            source_promotion_root: config.source_promotion_root.clone(),
            binding_root: String::new(),
        };
        root.binding_root = root.compute_root();
        root
    }

    pub fn from_root(
        slot_kind: LaneSlotKind,
        promoted_slot_root: &str,
        config: &Config,
    ) -> Result<Self> {
        ensure_root("promoted_slot_root", promoted_slot_root)?;
        let mut root = Self {
            slot_kind,
            promoted_slot_root: promoted_slot_root.to_string(),
            source_promotion_root: config.source_promotion_root.clone(),
            binding_root: String::new(),
        };
        root.binding_root = root.compute_root();
        Ok(root)
    }

    pub fn compute_root(&self) -> String {
        value_root(
            "WAVE96-PROMOTED-SLOT-ROOT",
            &json!({
                "slot_kind": self.slot_kind.as_str(),
                "promoted_slot_root": self.promoted_slot_root,
                "source_promotion_root": self.source_promotion_root,
            }),
        )
    }

    pub fn validate(&self, config: &Config) -> Result<()> {
        ensure_root("promoted_slot_root", &self.promoted_slot_root)?;
        ensure_root("source_promotion_root", &self.source_promotion_root)?;
        ensure_root("binding_root", &self.binding_root)?;
        if self.source_promotion_root != config.source_promotion_root {
            return Err("promoted slot root is not bound to configured wave95 root".to_string());
        }
        if self.binding_root != self.compute_root() {
            return Err("promoted slot root binding does not match root".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "slot_kind": self.slot_kind.as_str(),
            "promoted_slot_root": self.promoted_slot_root,
            "source_promotion_root": self.source_promotion_root,
            "binding_root": self.binding_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleaseClaim {
    pub claim_id: String,
    pub slot_kind: LaneSlotKind,
    pub status: ReadinessStatus,
    pub promoted_slot_binding_root: String,
    pub release_claim_root: String,
    pub reviewer_signoff_root: String,
    pub operator_signoff_root: String,
    pub quorum_claim_root: String,
}

impl ReleaseClaim {
    pub fn blocked(slot_kind: LaneSlotKind, promoted: &PromotedSlotRoot, config: &Config) -> Self {
        let mut claim = Self {
            claim_id: stable_id("wave96-release-claim", slot_kind.as_str()),
            slot_kind,
            status: ReadinessStatus::BlockedEmpty,
            promoted_slot_binding_root: promoted.binding_root.clone(),
            release_claim_root: config.release_claim_placeholder_root.clone(),
            reviewer_signoff_root: empty_root(&format!("{}-reviewer-signoff", slot_kind.as_str())),
            operator_signoff_root: empty_root(&format!("{}-operator-signoff", slot_kind.as_str())),
            quorum_claim_root: String::new(),
        };
        claim.quorum_claim_root = claim.compute_root();
        claim
    }

    pub fn pending(
        slot_kind: LaneSlotKind,
        promoted: &PromotedSlotRoot,
        release_claim_root: &str,
    ) -> Result<Self> {
        ensure_root("release_claim_root", release_claim_root)?;
        let mut claim = Self {
            claim_id: stable_id(
                "wave96-release-claim",
                &format!("{}:{}", slot_kind.as_str(), promoted.binding_root),
            ),
            slot_kind,
            status: ReadinessStatus::PendingQuorum,
            promoted_slot_binding_root: promoted.binding_root.clone(),
            release_claim_root: release_claim_root.to_string(),
            reviewer_signoff_root: empty_root(&format!("{}-reviewer-signoff", slot_kind.as_str())),
            operator_signoff_root: empty_root(&format!("{}-operator-signoff", slot_kind.as_str())),
            quorum_claim_root: String::new(),
        };
        claim.quorum_claim_root = claim.compute_root();
        Ok(claim)
    }

    pub fn compute_root(&self) -> String {
        value_root(
            "WAVE96-RELEASE-CLAIM",
            &json!({
                "claim_id": self.claim_id,
                "slot_kind": self.slot_kind.as_str(),
                "status": self.status.as_str(),
                "promoted_slot_binding_root": self.promoted_slot_binding_root,
                "release_claim_root": self.release_claim_root,
                "reviewer_signoff_root": self.reviewer_signoff_root,
                "operator_signoff_root": self.operator_signoff_root,
            }),
        )
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("claim_id", &self.claim_id)?;
        ensure_root(
            "promoted_slot_binding_root",
            &self.promoted_slot_binding_root,
        )?;
        ensure_root("release_claim_root", &self.release_claim_root)?;
        ensure_root("reviewer_signoff_root", &self.reviewer_signoff_root)?;
        ensure_root("operator_signoff_root", &self.operator_signoff_root)?;
        ensure_root("quorum_claim_root", &self.quorum_claim_root)?;
        if self.quorum_claim_root != self.compute_root() {
            return Err("release claim root does not match claim".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "claim_id": self.claim_id,
            "slot_kind": self.slot_kind.as_str(),
            "status": self.status.as_str(),
            "promoted_slot_binding_root": self.promoted_slot_binding_root,
            "release_claim_root": self.release_claim_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "operator_signoff_root": self.operator_signoff_root,
            "quorum_claim_root": self.quorum_claim_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct QuorumBlocker {
    pub blocker_id: String,
    pub kind: QuorumBlockerKind,
    pub slot_kind: LaneSlotKind,
    pub evidence_root: String,
    pub active: bool,
    pub blocker_root: String,
}

impl QuorumBlocker {
    pub fn new(kind: QuorumBlockerKind, slot_kind: LaneSlotKind, evidence_root: &str) -> Self {
        let blocker_id = stable_id(
            "wave96-quorum-blocker",
            &format!("{}:{}:{}", kind.as_str(), slot_kind.as_str(), evidence_root),
        );
        let mut blocker = Self {
            blocker_id,
            kind,
            slot_kind,
            evidence_root: evidence_root.to_string(),
            active: true,
            blocker_root: String::new(),
        };
        blocker.blocker_root = blocker.compute_root();
        blocker
    }

    pub fn compute_root(&self) -> String {
        value_root(
            "WAVE96-QUORUM-BLOCKER",
            &json!({
                "blocker_id": self.blocker_id,
                "kind": self.kind.as_str(),
                "slot_kind": self.slot_kind.as_str(),
                "evidence_root": self.evidence_root,
                "active": self.active,
            }),
        )
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("blocker_id", &self.blocker_id)?;
        ensure_root("evidence_root", &self.evidence_root)?;
        ensure_root("blocker_root", &self.blocker_root)?;
        if self.blocker_root != self.compute_root() {
            return Err("quorum blocker root does not match blocker".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "blocker_id": self.blocker_id,
            "kind": self.kind.as_str(),
            "slot_kind": self.slot_kind.as_str(),
            "evidence_root": self.evidence_root,
            "active": self.active,
            "blocker_root": self.blocker_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandHint {
    pub command_id: String,
    pub kind: CommandHintKind,
    pub target_root: String,
    pub fail_closed_preserving: bool,
    pub command_root: String,
}

impl CommandHint {
    pub fn new(kind: CommandHintKind, target_root: &str) -> Self {
        let command_id = stable_id(
            "wave96-command-hint",
            &format!("{}:{}", kind.as_str(), target_root),
        );
        let mut command = Self {
            command_id,
            kind,
            target_root: target_root.to_string(),
            fail_closed_preserving: true,
            command_root: String::new(),
        };
        command.command_root = command.compute_root();
        command
    }

    pub fn canonical(config: &Config) -> Vec<Self> {
        vec![
            Self::new(
                CommandHintKind::HoldRelease,
                &config.source_promotion_blocker_root,
            ),
            Self::new(
                CommandHintKind::ImportWave95PromotedSlotRoot,
                &config.source_promoted_slot_root,
            ),
            Self::new(
                CommandHintKind::BindPromotedSlotToLaneQuorum,
                &config.source_promotion_root,
            ),
            Self::new(
                CommandHintKind::ReviewQuorumBlockers,
                &config.source_promotion_blocker_root,
            ),
            Self::new(
                CommandHintKind::AttachReleaseClaimRoot,
                &config.release_claim_placeholder_root,
            ),
            Self::new(
                CommandHintKind::AttachReviewerSignoffRoot,
                &config.release_claim_placeholder_root,
            ),
            Self::new(
                CommandHintKind::AttachOperatorSignoffRoot,
                &config.release_claim_placeholder_root,
            ),
            Self::new(
                CommandHintKind::KeepFailClosed,
                &config.source_promotion_blocker_root,
            ),
        ]
    }

    pub fn compute_root(&self) -> String {
        value_root(
            "WAVE96-COMMAND-HINT",
            &json!({
                "command_id": self.command_id,
                "kind": self.kind.as_str(),
                "target_root": self.target_root,
                "fail_closed_preserving": self.fail_closed_preserving,
            }),
        )
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("command_id", &self.command_id)?;
        ensure_root("target_root", &self.target_root)?;
        ensure_root("command_root", &self.command_root)?;
        if self.command_root != self.compute_root() {
            return Err("command root does not match hint".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "command_id": self.command_id,
            "kind": self.kind.as_str(),
            "target_root": self.target_root,
            "fail_closed_preserving": self.fail_closed_preserving,
            "command_root": self.command_root,
        })
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub slot_count: u64,
    pub promoted_slot_roots: u64,
    pub release_claims: u64,
    pub release_ready_lanes: u64,
    pub blocked_claims: u64,
    pub rejected_claims: u64,
    pub quorum_blockers: u64,
    pub command_hints: u64,
    pub raw_payload_records: u64,
}

impl Counters {
    pub fn public_record(&self) -> PublicRecord {
        json!({
            "slot_count": self.slot_count,
            "promoted_slot_roots": self.promoted_slot_roots,
            "release_claims": self.release_claims,
            "release_ready_lanes": self.release_ready_lanes,
            "blocked_claims": self.blocked_claims,
            "rejected_claims": self.rejected_claims,
            "quorum_blockers": self.quorum_blockers,
            "command_hints": self.command_hints,
            "raw_payload_records": self.raw_payload_records,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub promoted_slot_roots: Vec<PromotedSlotRoot>,
    pub release_claims: Vec<ReleaseClaim>,
    pub quorum_blockers: Vec<QuorumBlocker>,
    pub command_hints: Vec<CommandHint>,
    pub counters: Counters,
}

impl State {
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;
        let promoted_slot_roots = LaneSlotKind::all()
            .into_iter()
            .map(|slot_kind| PromotedSlotRoot::placeholder(slot_kind, &config))
            .collect::<Vec<_>>();
        let release_claims = promoted_slot_roots
            .iter()
            .map(|promoted| ReleaseClaim::blocked(promoted.slot_kind, promoted, &config))
            .collect::<Vec<_>>();
        let mut state = Self {
            command_hints: CommandHint::canonical(&config),
            quorum_blockers: Vec::new(),
            counters: Counters::default(),
            release_claims,
            promoted_slot_roots,
            config,
        };
        state.recompute();
        state.validate()?;
        Ok(state)
    }

    pub fn stage_release_claim(
        &mut self,
        slot_kind: LaneSlotKind,
        promoted_slot_root: &str,
        release_claim_root: &str,
    ) -> Result<String> {
        ensure_root("promoted_slot_root", promoted_slot_root)?;
        ensure_root("release_claim_root", release_claim_root)?;
        let promoted = PromotedSlotRoot::from_root(slot_kind, promoted_slot_root, &self.config)?;
        for stored in &mut self.promoted_slot_roots {
            if stored.slot_kind == slot_kind {
                *stored = promoted.clone();
            }
        }
        let claim = ReleaseClaim::pending(slot_kind, &promoted, release_claim_root)?;
        for stored in &mut self.release_claims {
            if stored.slot_kind == slot_kind {
                *stored = claim.clone();
            }
        }
        self.recompute();
        self.validate()?;
        Ok(self.release_claim_root())
    }

    pub fn recompute(&mut self) {
        self.quorum_blockers = self.compute_blockers();
        self.counters = self.compute_counters();
    }

    pub fn compute_blockers(&self) -> Vec<QuorumBlocker> {
        let mut blockers = Vec::new();
        for promoted in &self.promoted_slot_roots {
            if promoted.promoted_slot_root
                == empty_root(&format!("{}-promoted-slot", promoted.slot_kind.as_str()))
            {
                blockers.push(QuorumBlocker::new(
                    QuorumBlockerKind::PromotedSlotRootMissing,
                    promoted.slot_kind,
                    &promoted.binding_root,
                ));
            }
            if promoted.source_promotion_root != self.config.source_promotion_root {
                blockers.push(QuorumBlocker::new(
                    QuorumBlockerKind::PromotedSlotRootNotBound,
                    promoted.slot_kind,
                    &promoted.binding_root,
                ));
            }
        }
        for claim in &self.release_claims {
            if claim.release_claim_root == self.config.release_claim_placeholder_root {
                blockers.push(QuorumBlocker::new(
                    QuorumBlockerKind::ReleaseClaimPlaceholder,
                    claim.slot_kind,
                    &claim.quorum_claim_root,
                ));
            }
            if !claim.status.is_ready() {
                blockers.push(QuorumBlocker::new(
                    QuorumBlockerKind::QuorumUnmet,
                    claim.slot_kind,
                    &claim.quorum_claim_root,
                ));
            }
            blockers.push(QuorumBlocker::new(
                QuorumBlockerKind::ReviewerSignoffMissing,
                claim.slot_kind,
                &claim.quorum_claim_root,
            ));
            blockers.push(QuorumBlocker::new(
                QuorumBlockerKind::OperatorSignoffMissing,
                claim.slot_kind,
                &claim.quorum_claim_root,
            ));
        }
        if self.config.max_release_ready_lanes == 0 {
            blockers.push(QuorumBlocker::new(
                QuorumBlockerKind::ReleaseReadyBudgetZero,
                LaneSlotKind::OperatorSignoff,
                &self.config.state_root(),
            ));
        }
        if !self.config.production_allowed {
            blockers.push(QuorumBlocker::new(
                QuorumBlockerKind::ProductionDenied,
                LaneSlotKind::OperatorSignoff,
                &self.config.state_root(),
            ));
        }
        if !self.config.fail_closed_armed {
            blockers.push(QuorumBlocker::new(
                QuorumBlockerKind::FailClosedDisarmed,
                LaneSlotKind::AuditReview,
                &self.config.state_root(),
            ));
        }
        if self.config.heavy_gates_ran {
            blockers.push(QuorumBlocker::new(
                QuorumBlockerKind::HeavyGateRunClaimed,
                LaneSlotKind::OperatorSignoff,
                &self.config.state_root(),
            ));
        }
        if self.counters.raw_payload_records > self.config.max_raw_payload_records {
            blockers.push(QuorumBlocker::new(
                QuorumBlockerKind::RawPayloadPresent,
                LaneSlotKind::PrivacyReview,
                &self.state_material_root(),
            ));
        }
        blockers
    }

    pub fn compute_counters(&self) -> Counters {
        Counters {
            slot_count: self.release_claims.len() as u64,
            promoted_slot_roots: self.promoted_slot_roots.len() as u64,
            release_claims: self.release_claims.len() as u64,
            release_ready_lanes: self
                .release_claims
                .iter()
                .filter(|claim| claim.status == ReadinessStatus::ReleaseReady)
                .count() as u64,
            blocked_claims: self
                .release_claims
                .iter()
                .filter(|claim| {
                    claim.status == ReadinessStatus::BlockedEmpty
                        || claim.status == ReadinessStatus::PendingQuorum
                })
                .count() as u64,
            rejected_claims: self
                .release_claims
                .iter()
                .filter(|claim| claim.status == ReadinessStatus::Rejected)
                .count() as u64,
            quorum_blockers: self.quorum_blockers.len() as u64,
            command_hints: self.command_hints.len() as u64,
            raw_payload_records: 0,
        }
    }

    pub fn promoted_slot_root(&self) -> String {
        collection_root(
            "WAVE96-PROMOTED-SLOT-ROOTS",
            self.promoted_slot_roots
                .iter()
                .map(PromotedSlotRoot::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn release_claim_root(&self) -> String {
        collection_root(
            "WAVE96-RELEASE-CLAIMS",
            self.release_claims
                .iter()
                .map(ReleaseClaim::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn quorum_blocker_root(&self) -> String {
        collection_root(
            "WAVE96-QUORUM-BLOCKERS",
            self.quorum_blockers
                .iter()
                .map(QuorumBlocker::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn command_hint_root(&self) -> String {
        collection_root(
            "WAVE96-COMMAND-HINTS",
            self.command_hints
                .iter()
                .map(CommandHint::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn lane_readiness_root(&self) -> String {
        collection_root(
            "WAVE96-LANE-RELEASE-READINESS",
            self.release_claims
                .iter()
                .map(|claim| {
                    json!({
                        "slot_kind": claim.slot_kind.as_str(),
                        "quorum_claim_root": claim.quorum_claim_root,
                        "release_ready": false,
                    })
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn state_material_root(&self) -> String {
        value_root(
            "WAVE96-STATE-MATERIAL",
            &json!({
                "config_root": self.config.state_root(),
                "promoted_slot_root": self.promoted_slot_root(),
                "release_claim_root": self.release_claim_root(),
                "lane_readiness_root": self.lane_readiness_root(),
                "command_hint_root": self.command_hint_root(),
                "counters": self.counters.public_record(),
                "quorum_met": false,
                "production_allowed": false,
                "heavy_gates_ran": false,
            }),
        )
    }

    pub fn state_root(&self) -> String {
        value_root(
            "WAVE96-STATE",
            &json!({
                "state_material_root": self.state_material_root(),
                "quorum_blocker_root": self.quorum_blocker_root(),
                "fail_closed_armed": self.config.fail_closed_armed,
                "quorum_met": false,
                "production_allowed": false,
                "release_ready_lanes": 0,
                "heavy_gates_ran": false,
            }),
        )
    }

    pub fn quorum_met(&self) -> bool {
        self.counters.release_ready_lanes >= self.config.quorum_threshold
            && self.quorum_blockers.is_empty()
    }

    pub fn production_denied(&self) -> bool {
        !self.config.production_allowed || !self.quorum_met()
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        ensure_min_count(
            "slot count",
            self.release_claims.len() as u64,
            self.config.min_slot_count,
        )?;
        for promoted in &self.promoted_slot_roots {
            promoted.validate(&self.config)?;
        }
        for claim in &self.release_claims {
            claim.validate()?;
        }
        for blocker in &self.quorum_blockers {
            blocker.validate()?;
        }
        for command in &self.command_hints {
            command.validate()?;
        }
        if self.counters.raw_payload_records > self.config.max_raw_payload_records {
            return Err("release readiness lane contains raw payload records".to_string());
        }
        if self.counters.release_ready_lanes > self.config.max_release_ready_lanes {
            return Err("release readiness lanes above configured limit".to_string());
        }
        if self.counters.release_ready_lanes != 0 {
            return Err("devnet release readiness lane must not mark lanes ready".to_string());
        }
        if self.compute_counters() != self.counters {
            return Err("release readiness counters do not match state".to_string());
        }
        if self.quorum_met() {
            return Err("devnet release readiness quorum must remain unmet".to_string());
        }
        if !self.production_denied() {
            return Err("release readiness lane cannot allow production".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave96_live_heavy_gate_receipt_release_readiness_quorum_audit_security_lane_state",
            "config": self.config.public_record(),
            "promoted_slot_root": self.promoted_slot_root(),
            "release_claim_root": self.release_claim_root(),
            "lane_readiness_root": self.lane_readiness_root(),
            "quorum_blocker_root": self.quorum_blocker_root(),
            "command_hint_root": self.command_hint_root(),
            "state_root": self.state_root(),
            "counters": self.counters.public_record(),
            "quorum_met": false,
            "production_denied": self.production_denied(),
            "heavy_gates_ran": false,
            "promoted_slot_roots": self.promoted_slot_roots.iter().map(PromotedSlotRoot::public_record).collect::<Vec<_>>(),
            "release_claims": self.release_claims.iter().map(ReleaseClaim::public_record).collect::<Vec<_>>(),
            "quorum_blockers": self.quorum_blockers.iter().map(QuorumBlocker::public_record).collect::<Vec<_>>(),
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
    let promoted_slot_roots = LaneSlotKind::all()
        .into_iter()
        .map(|slot_kind| PromotedSlotRoot::placeholder(slot_kind, &config))
        .collect::<Vec<_>>();
    let release_claims = promoted_slot_roots
        .iter()
        .map(|promoted| ReleaseClaim::blocked(promoted.slot_kind, promoted, &config))
        .collect::<Vec<_>>();
    let mut state = State {
        command_hints: CommandHint::canonical(&config),
        quorum_blockers: vec![QuorumBlocker::new(
            QuorumBlockerKind::ProductionDenied,
            LaneSlotKind::OperatorSignoff,
            &value_root(
                "WAVE96-FALLBACK-ERROR",
                &json!({"error_root": stable_id("fallback-error", &error)}),
            ),
        )],
        counters: Counters::default(),
        release_claims,
        promoted_slot_roots,
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
        "MONERO-L2-PQ-FORCE-EXIT-WAVE96-STABLE-ID",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(QUORUM_SUITE),
            HashPart::Str(domain),
            HashPart::Str(label),
        ],
        32,
    )
}

fn deterministic_root(label: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WAVE96-DETERMINISTIC-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(QUORUM_SUITE),
            HashPart::Str(label),
        ],
        32,
    )
}

fn empty_root(label: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WAVE96-EMPTY-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(QUORUM_SUITE),
            HashPart::Str(label),
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
