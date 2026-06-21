use std::collections::BTreeMap;

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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave96-live-heavy-gate-receipt-release-readiness-quorum-wallet-watchtower-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const WAVE_LABEL: &str = "wave96";
pub const SOURCE_WAVE_LABEL: &str = "wave95";
pub const TARGET_WAVE_LABEL: &str = "future-release";
pub const SOURCE_LANE: &str =
    "force-exit-live-heavy-gate-receipt-slot-promotion-wallet-watchtower-lane";
pub const RELEASE_READINESS_LANE: &str =
    "force-exit-live-heavy-gate-receipt-release-readiness-quorum-wallet-watchtower-lane";
pub const EMPTY_ROOT_MARKER: &str = "empty-wave96-release-readiness-root";
pub const DEFAULT_RELEASE_EPOCH: u64 = 96;
pub const DEFAULT_MIN_PROMOTED_SLOT_ROOTS: u64 = 6;
pub const DEFAULT_MIN_WATCHTOWER_QUORUM_ROOTS: u64 = 4;
pub const DEFAULT_MIN_USER_RUNBOOK_REPLAY_ROOTS: u64 = 2;
pub const DEFAULT_MIN_OPERATOR_SIGNOFF_ROOTS: u64 = 3;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub wave_label: String,
    pub source_wave_label: String,
    pub target_wave_label: String,
    pub source_lane: String,
    pub release_readiness_lane: String,
    pub empty_root_marker: String,
    pub release_epoch: u64,
    pub min_promoted_slot_roots: u64,
    pub min_watchtower_quorum_roots: u64,
    pub min_user_runbook_replay_roots: u64,
    pub min_operator_signoff_roots: u64,
    pub require_wallet_escape_dry_run: bool,
    pub require_watchtower_quorum: bool,
    pub require_user_runbook_replay: bool,
    pub require_redacted_recovery_proof: bool,
    pub require_wallet_visible_receipt: bool,
    pub require_operator_signoff: bool,
    pub require_release_claim_placeholder: bool,
    pub require_roots_only_public_record: bool,
    pub fail_closed_on_empty_release_readiness: bool,
    pub production_enabled: bool,
    pub heavy_gate_execution_allowed: bool,
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
            target_wave_label: TARGET_WAVE_LABEL.to_string(),
            source_lane: SOURCE_LANE.to_string(),
            release_readiness_lane: RELEASE_READINESS_LANE.to_string(),
            empty_root_marker: EMPTY_ROOT_MARKER.to_string(),
            release_epoch: DEFAULT_RELEASE_EPOCH,
            min_promoted_slot_roots: DEFAULT_MIN_PROMOTED_SLOT_ROOTS,
            min_watchtower_quorum_roots: DEFAULT_MIN_WATCHTOWER_QUORUM_ROOTS,
            min_user_runbook_replay_roots: DEFAULT_MIN_USER_RUNBOOK_REPLAY_ROOTS,
            min_operator_signoff_roots: DEFAULT_MIN_OPERATOR_SIGNOFF_ROOTS,
            require_wallet_escape_dry_run: true,
            require_watchtower_quorum: true,
            require_user_runbook_replay: true,
            require_redacted_recovery_proof: true,
            require_wallet_visible_receipt: true,
            require_operator_signoff: true,
            require_release_claim_placeholder: true,
            require_roots_only_public_record: true,
            fail_closed_on_empty_release_readiness: true,
            production_enabled: false,
            heavy_gate_execution_allowed: false,
        }
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }

    pub fn min_roots_for_lane(&self, lane: ReadinessLaneKind) -> u64 {
        match lane {
            ReadinessLaneKind::WalletEscapeDryRun => 1,
            ReadinessLaneKind::WatchtowerQuorum => self.min_watchtower_quorum_roots,
            ReadinessLaneKind::UserRunbookReplay => self.min_user_runbook_replay_roots,
            ReadinessLaneKind::RedactedRecoveryProof => 1,
            ReadinessLaneKind::WalletVisibleReceipt => 1,
            ReadinessLaneKind::OperatorSignoff => self.min_operator_signoff_roots,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "wave_label": self.wave_label,
            "source_wave_label": self.source_wave_label,
            "target_wave_label": self.target_wave_label,
            "source_lane": self.source_lane,
            "release_readiness_lane": self.release_readiness_lane,
            "empty_root_marker": self.empty_root_marker,
            "release_epoch": self.release_epoch,
            "min_promoted_slot_roots": self.min_promoted_slot_roots,
            "min_watchtower_quorum_roots": self.min_watchtower_quorum_roots,
            "min_user_runbook_replay_roots": self.min_user_runbook_replay_roots,
            "min_operator_signoff_roots": self.min_operator_signoff_roots,
            "require_wallet_escape_dry_run": self.require_wallet_escape_dry_run,
            "require_watchtower_quorum": self.require_watchtower_quorum,
            "require_user_runbook_replay": self.require_user_runbook_replay,
            "require_redacted_recovery_proof": self.require_redacted_recovery_proof,
            "require_wallet_visible_receipt": self.require_wallet_visible_receipt,
            "require_operator_signoff": self.require_operator_signoff,
            "require_release_claim_placeholder": self.require_release_claim_placeholder,
            "require_roots_only_public_record": self.require_roots_only_public_record,
            "fail_closed_on_empty_release_readiness": self.fail_closed_on_empty_release_readiness,
            "production_enabled": self.production_enabled,
            "heavy_gate_execution_allowed": self.heavy_gate_execution_allowed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessLaneKind {
    WalletEscapeDryRun,
    WatchtowerQuorum,
    UserRunbookReplay,
    RedactedRecoveryProof,
    WalletVisibleReceipt,
    OperatorSignoff,
}

impl ReadinessLaneKind {
    pub fn all() -> [Self; 6] {
        [
            Self::WalletEscapeDryRun,
            Self::WatchtowerQuorum,
            Self::UserRunbookReplay,
            Self::RedactedRecoveryProof,
            Self::WalletVisibleReceipt,
            Self::OperatorSignoff,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::WalletEscapeDryRun => "wallet_escape_dry_run",
            Self::WatchtowerQuorum => "watchtower_quorum",
            Self::UserRunbookReplay => "user_runbook_replay",
            Self::RedactedRecoveryProof => "redacted_recovery_proof",
            Self::WalletVisibleReceipt => "wallet_visible_receipt",
            Self::OperatorSignoff => "operator_signoff",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseReadinessStatus {
    EmptyBlocked,
    QuorumBlocked,
    ReleaseReady,
    Denied,
}

impl ReleaseReadinessStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EmptyBlocked => "empty_blocked",
            Self::QuorumBlocked => "quorum_blocked",
            Self::ReleaseReady => "release_ready",
            Self::Denied => "denied",
        }
    }

    pub fn can_release(self) -> bool {
        matches!(self, Self::ReleaseReady)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QuorumBlocker {
    EmptyPromotedSlots,
    PromotedSlotRootMissing,
    PromotedSlotQuorumMissing,
    LaneEvidenceRootMissing,
    RootShapeInvalid,
    RootsOnlyRecordMissing,
    ReleaseClaimPlaceholderMissing,
    WalletEscapeDryRunMissing,
    WatchtowerQuorumMissing,
    UserRunbookReplayMissing,
    RedactedRecoveryProofMissing,
    WalletVisibleReceiptMissing,
    OperatorSignoffMissing,
    ProductionDenied,
    HeavyGateClaimPresent,
}

impl QuorumBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EmptyPromotedSlots => "empty_promoted_slots",
            Self::PromotedSlotRootMissing => "promoted_slot_root_missing",
            Self::PromotedSlotQuorumMissing => "promoted_slot_quorum_missing",
            Self::LaneEvidenceRootMissing => "lane_evidence_root_missing",
            Self::RootShapeInvalid => "root_shape_invalid",
            Self::RootsOnlyRecordMissing => "roots_only_record_missing",
            Self::ReleaseClaimPlaceholderMissing => "release_claim_placeholder_missing",
            Self::WalletEscapeDryRunMissing => "wallet_escape_dry_run_missing",
            Self::WatchtowerQuorumMissing => "watchtower_quorum_missing",
            Self::UserRunbookReplayMissing => "user_runbook_replay_missing",
            Self::RedactedRecoveryProofMissing => "redacted_recovery_proof_missing",
            Self::WalletVisibleReceiptMissing => "wallet_visible_receipt_missing",
            Self::OperatorSignoffMissing => "operator_signoff_missing",
            Self::ProductionDenied => "production_denied",
            Self::HeavyGateClaimPresent => "heavy_gate_claim_present",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandHintKind {
    HoldBlockedRelease,
    ImportPromotedSlotRoot,
    RequestWalletEscapeDryRunRoot,
    RequestWatchtowerQuorumRoot,
    RequestUserRunbookReplayRoot,
    RequestRedactedRecoveryProofRoot,
    RequestWalletVisibleReceiptRoot,
    RequestOperatorSignoffRoot,
    PrepareReleaseClaimPlaceholder,
    PublishReleaseReadinessClaim,
}

impl CommandHintKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HoldBlockedRelease => "hold_blocked_release",
            Self::ImportPromotedSlotRoot => "import_promoted_slot_root",
            Self::RequestWalletEscapeDryRunRoot => "request_wallet_escape_dry_run_root",
            Self::RequestWatchtowerQuorumRoot => "request_watchtower_quorum_root",
            Self::RequestUserRunbookReplayRoot => "request_user_runbook_replay_root",
            Self::RequestRedactedRecoveryProofRoot => "request_redacted_recovery_proof_root",
            Self::RequestWalletVisibleReceiptRoot => "request_wallet_visible_receipt_root",
            Self::RequestOperatorSignoffRoot => "request_operator_signoff_root",
            Self::PrepareReleaseClaimPlaceholder => "prepare_release_claim_placeholder",
            Self::PublishReleaseReadinessClaim => "publish_release_readiness_claim",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandHint {
    pub kind: CommandHintKind,
    pub command_root: String,
    pub next_blocker_root: String,
}

impl CommandHint {
    pub fn new(
        kind: CommandHintKind,
        lane_kind: ReadinessLaneKind,
        blockers: &[QuorumBlocker],
    ) -> Self {
        let blocker_names = blockers
            .iter()
            .map(|blocker| Value::String(blocker.as_str().to_string()))
            .collect::<Vec<_>>();
        let next_blocker_root =
            merkle_root("WAVE96-WALLET-WATCHTOWER-NEXT-BLOCKERS", &blocker_names);
        let command_root = record_root(
            "command-hint",
            &json!({
                "lane_kind": lane_kind.as_str(),
                "hint": kind.as_str(),
                "next_blocker_root": next_blocker_root,
            }),
        );
        Self {
            kind,
            command_root,
            next_blocker_root,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": self.kind.as_str(),
            "command_root": self.command_root,
            "next_blocker_root": self.next_blocker_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LaneReleaseReadinessClaim {
    pub lane_kind: ReadinessLaneKind,
    pub promoted_slot_roots: Vec<String>,
    pub lane_evidence_roots: Vec<String>,
    pub promoted_slot_root: String,
    pub lane_evidence_root: String,
    pub quorum_blocker_root: String,
    pub release_claim_placeholder_root: String,
    pub readiness_claim_root: String,
    pub blockers: Vec<QuorumBlocker>,
    pub status: ReleaseReadinessStatus,
    pub command_hint: CommandHint,
    pub heavy_gate_claimed: bool,
    pub release_ready: bool,
}

impl LaneReleaseReadinessClaim {
    pub fn empty(lane_kind: ReadinessLaneKind, config: &Config) -> Self {
        Self::from_roots(lane_kind, Vec::new(), Vec::new(), false, config)
    }

    pub fn from_roots(
        lane_kind: ReadinessLaneKind,
        promoted_slot_roots: Vec<String>,
        lane_evidence_roots: Vec<String>,
        heavy_gate_claimed: bool,
        config: &Config,
    ) -> Self {
        let promoted_slot_root = promoted_slot_aggregate_root(lane_kind, &promoted_slot_roots);
        let lane_evidence_root = lane_evidence_aggregate_root(lane_kind, &lane_evidence_roots);
        let release_claim_placeholder_root =
            release_claim_placeholder_root(lane_kind, &promoted_slot_root, &lane_evidence_root);
        let readiness_claim_root =
            readiness_claim_root(lane_kind, &promoted_slot_root, &lane_evidence_root);
        let mut claim = Self {
            lane_kind,
            promoted_slot_roots,
            lane_evidence_roots,
            promoted_slot_root,
            lane_evidence_root,
            quorum_blocker_root: empty_root("quorum-blockers"),
            release_claim_placeholder_root,
            readiness_claim_root,
            blockers: Vec::new(),
            status: ReleaseReadinessStatus::EmptyBlocked,
            command_hint: CommandHint::new(CommandHintKind::HoldBlockedRelease, lane_kind, &[]),
            heavy_gate_claimed,
            release_ready: false,
        };
        claim.recompute(config);
        claim
    }

    pub fn recompute(&mut self, config: &Config) {
        self.promoted_slot_root =
            promoted_slot_aggregate_root(self.lane_kind, &self.promoted_slot_roots);
        self.lane_evidence_root =
            lane_evidence_aggregate_root(self.lane_kind, &self.lane_evidence_roots);
        self.release_claim_placeholder_root = release_claim_placeholder_root(
            self.lane_kind,
            &self.promoted_slot_root,
            &self.lane_evidence_root,
        );
        self.readiness_claim_root = readiness_claim_root(
            self.lane_kind,
            &self.promoted_slot_root,
            &self.lane_evidence_root,
        );
        self.blockers = quorum_blockers(self, config);
        self.quorum_blocker_root = self.blocker_root();
        self.status = if self.heavy_gate_claimed {
            ReleaseReadinessStatus::Denied
        } else if self.promoted_slot_roots.is_empty() {
            ReleaseReadinessStatus::EmptyBlocked
        } else if self.blockers.is_empty() {
            ReleaseReadinessStatus::ReleaseReady
        } else {
            ReleaseReadinessStatus::QuorumBlocked
        };
        self.release_ready = self.status.can_release() && config.production_enabled;
        self.command_hint =
            CommandHint::new(command_for_claim(self), self.lane_kind, &self.blockers);
    }

    pub fn blocker_root(&self) -> String {
        merkle_root(
            "WAVE96-WALLET-WATCHTOWER-QUORUM-BLOCKERS",
            &self
                .blockers
                .iter()
                .map(|blocker| Value::String(blocker.as_str().to_string()))
                .collect::<Vec<_>>(),
        )
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane_kind": self.lane_kind.as_str(),
            "status": self.status.as_str(),
            "promoted_slot_root": self.promoted_slot_root,
            "lane_evidence_root": self.lane_evidence_root,
            "quorum_blocker_root": self.quorum_blocker_root,
            "release_claim_placeholder_root": self.release_claim_placeholder_root,
            "readiness_claim_root": self.readiness_claim_root,
            "command_hint": self.command_hint.public_record(),
            "heavy_gate_claimed": self.heavy_gate_claimed,
            "release_ready": self.release_ready,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("lane-release-readiness-claim", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleaseReadinessSummary {
    pub fail_closed: bool,
    pub production_denied: bool,
    pub release_ready_lanes: u64,
    pub quorum_unmet_lanes: u64,
    pub denied_lanes: u64,
    pub promoted_slot_roots_seen: u64,
    pub blocker_root: String,
    pub command_root: String,
    pub release_claim_placeholder_root: String,
    pub readiness_claim_root: String,
    pub lane_claim_root: String,
    pub heavy_gate_claimed: bool,
}

impl ReleaseReadinessSummary {
    pub fn from_claims(claims: &BTreeMap<String, LaneReleaseReadinessClaim>) -> Self {
        let release_ready_lanes =
            claims.values().filter(|claim| claim.release_ready).count() as u64;
        let quorum_unmet_lanes = claims
            .values()
            .filter(|claim| {
                matches!(
                    claim.status,
                    ReleaseReadinessStatus::EmptyBlocked | ReleaseReadinessStatus::QuorumBlocked
                )
            })
            .count() as u64;
        let denied_lanes = claims
            .values()
            .filter(|claim| matches!(claim.status, ReleaseReadinessStatus::Denied))
            .count() as u64;
        let promoted_slot_roots_seen = claims
            .values()
            .map(|claim| claim.promoted_slot_roots.len() as u64)
            .sum::<u64>();
        let heavy_gate_claimed = claims.values().any(|claim| claim.heavy_gate_claimed);
        let blocker_root = merkle_root(
            "WAVE96-WALLET-WATCHTOWER-ALL-BLOCKERS",
            &claims
                .values()
                .map(|claim| Value::String(claim.blocker_root()))
                .collect::<Vec<_>>(),
        );
        let command_root = merkle_root(
            "WAVE96-WALLET-WATCHTOWER-COMMAND-HINTS",
            &claims
                .values()
                .map(|claim| claim.command_hint.public_record())
                .collect::<Vec<_>>(),
        );
        let release_claim_placeholder_root = merkle_root(
            "WAVE96-WALLET-WATCHTOWER-RELEASE-CLAIM-PLACEHOLDERS",
            &claims
                .values()
                .map(|claim| Value::String(claim.release_claim_placeholder_root.clone()))
                .collect::<Vec<_>>(),
        );
        let readiness_claim_root = merkle_root(
            "WAVE96-WALLET-WATCHTOWER-READINESS-CLAIMS",
            &claims
                .values()
                .map(|claim| Value::String(claim.readiness_claim_root.clone()))
                .collect::<Vec<_>>(),
        );
        let lane_claim_root = merkle_root(
            "WAVE96-WALLET-WATCHTOWER-LANE-CLAIMS",
            &claims
                .values()
                .map(|claim| Value::String(claim.state_root()))
                .collect::<Vec<_>>(),
        );
        Self {
            fail_closed: release_ready_lanes == 0 || quorum_unmet_lanes > 0 || denied_lanes > 0,
            production_denied: true,
            release_ready_lanes,
            quorum_unmet_lanes,
            denied_lanes,
            promoted_slot_roots_seen,
            blocker_root,
            command_root,
            release_claim_placeholder_root,
            readiness_claim_root,
            lane_claim_root,
            heavy_gate_claimed,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "fail_closed": self.fail_closed,
            "production_denied": self.production_denied,
            "release_ready_lanes": self.release_ready_lanes,
            "quorum_unmet_lanes": self.quorum_unmet_lanes,
            "denied_lanes": self.denied_lanes,
            "promoted_slot_roots_seen": self.promoted_slot_roots_seen,
            "blocker_root": self.blocker_root,
            "command_root": self.command_root,
            "release_claim_placeholder_root": self.release_claim_placeholder_root,
            "readiness_claim_root": self.readiness_claim_root,
            "lane_claim_root": self.lane_claim_root,
            "heavy_gate_claimed": self.heavy_gate_claimed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("release-readiness-summary", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub lane_claims: BTreeMap<String, LaneReleaseReadinessClaim>,
    pub summary: ReleaseReadinessSummary,
}

impl Default for State {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

impl State {
    pub fn new(config: Config) -> Self {
        let lane_claims = ReadinessLaneKind::all()
            .iter()
            .map(|lane_kind| {
                let claim = LaneReleaseReadinessClaim::empty(*lane_kind, &config);
                (lane_kind.as_str().to_string(), claim)
            })
            .collect::<BTreeMap<_, _>>();
        let summary = ReleaseReadinessSummary::from_claims(&lane_claims);
        Self {
            config,
            lane_claims,
            summary,
        }
    }

    pub fn stage_lane_claim(
        mut self,
        lane_kind: ReadinessLaneKind,
        promoted_slot_roots: Vec<String>,
        lane_evidence_roots: Vec<String>,
    ) -> Result<Self> {
        let claim = LaneReleaseReadinessClaim::from_roots(
            lane_kind,
            promoted_slot_roots,
            lane_evidence_roots,
            false,
            &self.config,
        );
        self.lane_claims
            .insert(lane_kind.as_str().to_string(), claim);
        self.recompute();
        Ok(self)
    }

    pub fn deny_heavy_gate_claim(mut self, lane_kind: ReadinessLaneKind) -> Result<Self> {
        let key = lane_kind.as_str().to_string();
        match self.lane_claims.remove(&key) {
            Some(mut claim) => {
                claim.heavy_gate_claimed = true;
                claim.recompute(&self.config);
                self.lane_claims.insert(key, claim);
                self.recompute();
                Ok(self)
            }
            None => Err(format!(
                "release readiness lane missing: {}",
                lane_kind.as_str()
            )),
        }
    }

    pub fn recompute(&mut self) {
        for claim in self.lane_claims.values_mut() {
            claim.recompute(&self.config);
        }
        self.summary = ReleaseReadinessSummary::from_claims(&self.lane_claims);
    }

    pub fn lane_claim_roots(&self) -> BTreeMap<String, String> {
        self.lane_claims
            .iter()
            .map(|(key, claim)| (key.clone(), claim.state_root()))
            .collect::<BTreeMap<_, _>>()
    }

    pub fn lane_claims_root(&self) -> String {
        merkle_root(
            "WAVE96-WALLET-WATCHTOWER-LANE-CLAIM-ROOTS",
            &self
                .lane_claim_roots()
                .values()
                .cloned()
                .map(Value::String)
                .collect::<Vec<_>>(),
        )
    }

    pub fn public_record_without_state_root(&self) -> PublicRecord {
        json!({
            "config": self.config.public_record(),
            "lane_claims": self.lane_claims.iter().map(|(key, claim)| (key.clone(), claim.public_record())).collect::<BTreeMap<_, _>>(),
            "lane_claim_roots": self.lane_claim_roots(),
            "lane_claims_root": self.lane_claims_root(),
            "summary": self.summary.public_record(),
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

pub fn release_readiness_quorum_runtime() -> Runtime {
    devnet()
}

pub fn promoted_slot_aggregate_root(
    lane_kind: ReadinessLaneKind,
    promoted_slot_roots: &[String],
) -> String {
    if promoted_slot_roots.is_empty() {
        return empty_root(lane_kind.as_str());
    }
    merkle_root(
        "WAVE96-WALLET-WATCHTOWER-PROMOTED-SLOT-ROOTS",
        &promoted_slot_roots
            .iter()
            .cloned()
            .map(Value::String)
            .collect::<Vec<_>>(),
    )
}

pub fn lane_evidence_aggregate_root(
    lane_kind: ReadinessLaneKind,
    lane_evidence_roots: &[String],
) -> String {
    if lane_evidence_roots.is_empty() {
        return empty_root(lane_kind.as_str());
    }
    merkle_root(
        "WAVE96-WALLET-WATCHTOWER-LANE-EVIDENCE-ROOTS",
        &lane_evidence_roots
            .iter()
            .cloned()
            .map(Value::String)
            .collect::<Vec<_>>(),
    )
}

pub fn release_claim_placeholder_root(
    lane_kind: ReadinessLaneKind,
    promoted_slot_root: &str,
    lane_evidence_root: &str,
) -> String {
    record_root(
        "release-claim-placeholder-root",
        &json!({
            "lane_kind": lane_kind.as_str(),
            "promoted_slot_root": promoted_slot_root,
            "lane_evidence_root": lane_evidence_root,
            "claim_body_redacted": true,
            "release_claim_not_submitted": true,
            "roots_only": true,
        }),
    )
}

pub fn readiness_claim_root(
    lane_kind: ReadinessLaneKind,
    promoted_slot_root: &str,
    lane_evidence_root: &str,
) -> String {
    record_root(
        "readiness-claim-root",
        &json!({
            "lane_kind": lane_kind.as_str(),
            "source_wave_label": SOURCE_WAVE_LABEL,
            "target_wave_label": TARGET_WAVE_LABEL,
            "promoted_slot_root": promoted_slot_root,
            "lane_evidence_root": lane_evidence_root,
            "wallet_material_absent": true,
            "route_material_absent": true,
            "heavy_gate_claimed": false,
        }),
    )
}

fn quorum_blockers(claim: &LaneReleaseReadinessClaim, config: &Config) -> Vec<QuorumBlocker> {
    let mut blockers = Vec::new();
    if config.fail_closed_on_empty_release_readiness && claim.promoted_slot_roots.is_empty() {
        blockers.push(QuorumBlocker::EmptyPromotedSlots);
    }
    if claim.promoted_slot_roots.is_empty() {
        blockers.push(QuorumBlocker::PromotedSlotRootMissing);
    }
    if claim.promoted_slot_roots.len() < config.min_promoted_slot_roots as usize {
        blockers.push(QuorumBlocker::PromotedSlotQuorumMissing);
    }
    if claim.lane_evidence_roots.is_empty() {
        blockers.push(QuorumBlocker::LaneEvidenceRootMissing);
    }
    if !is_root_like(&claim.promoted_slot_root)
        || !is_root_like(&claim.lane_evidence_root)
        || !claim
            .promoted_slot_roots
            .iter()
            .all(|root| is_root_like(root))
        || !claim
            .lane_evidence_roots
            .iter()
            .all(|root| is_root_like(root))
    {
        blockers.push(QuorumBlocker::RootShapeInvalid);
    }
    if config.require_roots_only_public_record && !roots_only_record_present(claim) {
        blockers.push(QuorumBlocker::RootsOnlyRecordMissing);
    }
    if config.require_release_claim_placeholder
        && !is_root_like(&claim.release_claim_placeholder_root)
    {
        blockers.push(QuorumBlocker::ReleaseClaimPlaceholderMissing);
    }
    if config.require_wallet_escape_dry_run
        && claim.lane_kind == ReadinessLaneKind::WalletEscapeDryRun
        && claim.lane_evidence_roots.len() < config.min_roots_for_lane(claim.lane_kind) as usize
    {
        blockers.push(QuorumBlocker::WalletEscapeDryRunMissing);
    }
    if config.require_watchtower_quorum
        && claim.lane_kind == ReadinessLaneKind::WatchtowerQuorum
        && claim.lane_evidence_roots.len() < config.min_roots_for_lane(claim.lane_kind) as usize
    {
        blockers.push(QuorumBlocker::WatchtowerQuorumMissing);
    }
    if config.require_user_runbook_replay
        && claim.lane_kind == ReadinessLaneKind::UserRunbookReplay
        && claim.lane_evidence_roots.len() < config.min_roots_for_lane(claim.lane_kind) as usize
    {
        blockers.push(QuorumBlocker::UserRunbookReplayMissing);
    }
    if config.require_redacted_recovery_proof
        && claim.lane_kind == ReadinessLaneKind::RedactedRecoveryProof
        && claim.lane_evidence_roots.len() < config.min_roots_for_lane(claim.lane_kind) as usize
    {
        blockers.push(QuorumBlocker::RedactedRecoveryProofMissing);
    }
    if config.require_wallet_visible_receipt
        && claim.lane_kind == ReadinessLaneKind::WalletVisibleReceipt
        && claim.lane_evidence_roots.len() < config.min_roots_for_lane(claim.lane_kind) as usize
    {
        blockers.push(QuorumBlocker::WalletVisibleReceiptMissing);
    }
    if config.require_operator_signoff
        && claim.lane_kind == ReadinessLaneKind::OperatorSignoff
        && claim.lane_evidence_roots.len() < config.min_roots_for_lane(claim.lane_kind) as usize
    {
        blockers.push(QuorumBlocker::OperatorSignoffMissing);
    }
    if !config.production_enabled {
        blockers.push(QuorumBlocker::ProductionDenied);
    }
    if claim.heavy_gate_claimed || config.heavy_gate_execution_allowed {
        blockers.push(QuorumBlocker::HeavyGateClaimPresent);
    }
    blockers
}

fn roots_only_record_present(claim: &LaneReleaseReadinessClaim) -> bool {
    is_root_like(&claim.promoted_slot_root)
        && is_root_like(&claim.lane_evidence_root)
        && is_root_like(&claim.release_claim_placeholder_root)
        && is_root_like(&claim.readiness_claim_root)
}

fn command_for_claim(claim: &LaneReleaseReadinessClaim) -> CommandHintKind {
    if claim.blockers.is_empty() {
        return CommandHintKind::PublishReleaseReadinessClaim;
    }
    match claim.blockers[0] {
        QuorumBlocker::EmptyPromotedSlots
        | QuorumBlocker::PromotedSlotRootMissing
        | QuorumBlocker::PromotedSlotQuorumMissing => CommandHintKind::ImportPromotedSlotRoot,
        QuorumBlocker::WalletEscapeDryRunMissing => CommandHintKind::RequestWalletEscapeDryRunRoot,
        QuorumBlocker::WatchtowerQuorumMissing => CommandHintKind::RequestWatchtowerQuorumRoot,
        QuorumBlocker::UserRunbookReplayMissing => CommandHintKind::RequestUserRunbookReplayRoot,
        QuorumBlocker::RedactedRecoveryProofMissing => {
            CommandHintKind::RequestRedactedRecoveryProofRoot
        }
        QuorumBlocker::WalletVisibleReceiptMissing => {
            CommandHintKind::RequestWalletVisibleReceiptRoot
        }
        QuorumBlocker::OperatorSignoffMissing => CommandHintKind::RequestOperatorSignoffRoot,
        QuorumBlocker::ReleaseClaimPlaceholderMissing => {
            CommandHintKind::PrepareReleaseClaimPlaceholder
        }
        QuorumBlocker::LaneEvidenceRootMissing
        | QuorumBlocker::RootShapeInvalid
        | QuorumBlocker::RootsOnlyRecordMissing
        | QuorumBlocker::ProductionDenied
        | QuorumBlocker::HeavyGateClaimPresent => CommandHintKind::HoldBlockedRelease,
    }
}

fn empty_root(marker_name: &str) -> String {
    let root = record_root(
        "empty-release-readiness-root",
        &json!({
            "marker": EMPTY_ROOT_MARKER,
            "marker_name": marker_name,
        }),
    );
    format!("{EMPTY_ROOT_MARKER}:{root}")
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
        "WAVE96-WALLET-WATCHTOWER-RECEIPT-RELEASE-READINESS-QUORUM",
        &[
            HashPart::Str(domain),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Json(record),
        ],
        32,
    )
}
