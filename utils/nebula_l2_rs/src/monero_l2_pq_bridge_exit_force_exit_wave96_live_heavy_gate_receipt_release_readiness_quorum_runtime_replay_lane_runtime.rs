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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave96-receipt-release-readiness-quorum-runtime-replay-lane-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-root-only-records";
pub const LANE_SUITE: &str =
    "wave96-live-heavy-gate-receipt-release-readiness-quorum-runtime-replay-lane-v1";
pub const DEFAULT_WAVE: u64 = 96;
pub const DEFAULT_PROMOTION_WAVE: u64 = 95;
pub const DEFAULT_FILL_WAVE: u64 = 94;
pub const DEFAULT_ADMISSION_WAVE: u64 = 93;
pub const DEFAULT_SLOT_WAVE: u64 = 92;
pub const DEFAULT_REQUIRED_QUORUM: u64 = 6;
pub const DEFAULT_MAX_PUBLIC_RAW_RECORDS: u64 = 0;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayLane {
    ReplayRun,
    RollbackDrill,
    AdversarialReplay,
    StaleArchiveReplacement,
    LiveExecutionReceipt,
    OperatorSignoff,
}

impl ReplayLane {
    pub fn all() -> Vec<Self> {
        vec![
            Self::ReplayRun,
            Self::RollbackDrill,
            Self::AdversarialReplay,
            Self::StaleArchiveReplacement,
            Self::LiveExecutionReceipt,
            Self::OperatorSignoff,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReplayRun => "replay_run",
            Self::RollbackDrill => "rollback_drill",
            Self::AdversarialReplay => "adversarial_replay",
            Self::StaleArchiveReplacement => "stale_archive_replacement",
            Self::LiveExecutionReceipt => "live_execution_receipt",
            Self::OperatorSignoff => "operator_signoff",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QuorumStatus {
    Unmet,
    Met,
}

impl QuorumStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unmet => "unmet",
            Self::Met => "met",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QuorumBlocker {
    FailClosedArmed,
    ProductionDenied,
    ZeroReleaseReadyLanesDefault,
    HeavyGateReceiptAbsent,
    PromotedSlotRootPlaceholder,
    ReleaseClaimPlaceholder,
    Wave95PromotionRootNotFinal,
    Wave94FillRootNotFinal,
    Wave93AdmissionRootNotFinal,
    Wave92SlotRootNotFinal,
    OperatorSignoffMissing,
}

impl QuorumBlocker {
    pub fn all() -> Vec<Self> {
        vec![
            Self::FailClosedArmed,
            Self::ProductionDenied,
            Self::ZeroReleaseReadyLanesDefault,
            Self::HeavyGateReceiptAbsent,
            Self::PromotedSlotRootPlaceholder,
            Self::ReleaseClaimPlaceholder,
            Self::Wave95PromotionRootNotFinal,
            Self::Wave94FillRootNotFinal,
            Self::Wave93AdmissionRootNotFinal,
            Self::Wave92SlotRootNotFinal,
            Self::OperatorSignoffMissing,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosedArmed => "fail_closed_armed",
            Self::ProductionDenied => "production_denied",
            Self::ZeroReleaseReadyLanesDefault => "zero_release_ready_lanes_default",
            Self::HeavyGateReceiptAbsent => "heavy_gate_receipt_absent",
            Self::PromotedSlotRootPlaceholder => "promoted_slot_root_placeholder",
            Self::ReleaseClaimPlaceholder => "release_claim_placeholder",
            Self::Wave95PromotionRootNotFinal => "wave95_promotion_root_not_final",
            Self::Wave94FillRootNotFinal => "wave94_fill_root_not_final",
            Self::Wave93AdmissionRootNotFinal => "wave93_admission_root_not_final",
            Self::Wave92SlotRootNotFinal => "wave92_slot_root_not_final",
            Self::OperatorSignoffMissing => "operator_signoff_missing",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandHint {
    ReplayPromotedSlotRoots,
    DrillRollbackToUnreleased,
    KeepAdversarialReleaseBlocked,
    ReplaceStaleArchiveRoot,
    BindLiveReceiptRoot,
    SignOffRootsOnly,
}

impl CommandHint {
    pub fn all() -> Vec<Self> {
        vec![
            Self::ReplayPromotedSlotRoots,
            Self::DrillRollbackToUnreleased,
            Self::KeepAdversarialReleaseBlocked,
            Self::ReplaceStaleArchiveRoot,
            Self::BindLiveReceiptRoot,
            Self::SignOffRootsOnly,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReplayPromotedSlotRoots => "replay_promoted_slot_roots",
            Self::DrillRollbackToUnreleased => "drill_rollback_to_unreleased",
            Self::KeepAdversarialReleaseBlocked => "keep_adversarial_release_blocked",
            Self::ReplaceStaleArchiveRoot => "replace_stale_archive_root",
            Self::BindLiveReceiptRoot => "bind_live_receipt_root",
            Self::SignOffRootsOnly => "sign_off_roots_only",
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
    pub promotion_wave: u64,
    pub fill_wave: u64,
    pub admission_wave: u64,
    pub slot_wave: u64,
    pub required_quorum: u64,
    pub wave95_promoted_slot_root: String,
    pub wave94_fill_staging_root: String,
    pub wave93_admission_quarantine_root: String,
    pub wave92_receipt_slot_root: String,
    pub fail_closed_armed: bool,
    pub production_denied: bool,
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
            promotion_wave: DEFAULT_PROMOTION_WAVE,
            fill_wave: DEFAULT_FILL_WAVE,
            admission_wave: DEFAULT_ADMISSION_WAVE,
            slot_wave: DEFAULT_SLOT_WAVE,
            required_quorum: DEFAULT_REQUIRED_QUORUM,
            wave95_promoted_slot_root: stable_id("wave95-promoted-slot-root", "all"),
            wave94_fill_staging_root: stable_id("wave94-fill-staging", "all"),
            wave93_admission_quarantine_root: stable_id("wave93-admission-quarantine", "all"),
            wave92_receipt_slot_root: stable_id("wave92-receipt-slot-registry", "all"),
            fail_closed_armed: true,
            production_denied: true,
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
        ensure_positive("promotion_wave", self.promotion_wave)?;
        ensure_positive("fill_wave", self.fill_wave)?;
        ensure_positive("admission_wave", self.admission_wave)?;
        ensure_positive("slot_wave", self.slot_wave)?;
        ensure_positive("required_quorum", self.required_quorum)?;
        ensure_root("wave95_promoted_slot_root", &self.wave95_promoted_slot_root)?;
        ensure_root("wave94_fill_staging_root", &self.wave94_fill_staging_root)?;
        ensure_root(
            "wave93_admission_quarantine_root",
            &self.wave93_admission_quarantine_root,
        )?;
        ensure_root("wave92_receipt_slot_root", &self.wave92_receipt_slot_root)?;
        if !(self.slot_wave < self.admission_wave
            && self.admission_wave < self.fill_wave
            && self.fill_wave < self.promotion_wave
            && self.promotion_wave < self.wave)
        {
            return Err(
                "wave ordering must be slot, admission, fill, promotion, quorum".to_string(),
            );
        }
        if !self.fail_closed_armed {
            return Err("release readiness fail closed guard is disarmed".to_string());
        }
        if !self.production_denied {
            return Err("devnet release readiness must deny production".to_string());
        }
        if self.heavy_gates_ran {
            return Err("release readiness runtime cannot claim heavy gate execution".to_string());
        }
        if self.max_public_raw_records != 0 {
            return Err("public records must remain roots only".to_string());
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
            "lane_suite": self.lane_suite,
            "wave": self.wave,
            "promotion_wave": self.promotion_wave,
            "fill_wave": self.fill_wave,
            "admission_wave": self.admission_wave,
            "slot_wave": self.slot_wave,
            "required_quorum": self.required_quorum,
            "wave95_promoted_slot_root": self.wave95_promoted_slot_root,
            "wave94_fill_staging_root": self.wave94_fill_staging_root,
            "wave93_admission_quarantine_root": self.wave93_admission_quarantine_root,
            "wave92_receipt_slot_root": self.wave92_receipt_slot_root,
            "fail_closed_armed": self.fail_closed_armed,
            "production_denied": self.production_denied,
            "heavy_gates_ran": self.heavy_gates_ran,
            "max_public_raw_records": self.max_public_raw_records,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE96-CONFIG", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PromotedSlotRootSet {
    pub lane: ReplayLane,
    pub wave95_promotion_root: String,
    pub promoted_slot_root: String,
    pub release_claim_placeholder_root: String,
    pub promoted_slot_count: u64,
    pub release_ready: bool,
}

impl PromotedSlotRootSet {
    pub fn placeholder(lane: ReplayLane, config: &Config) -> Self {
        Self {
            lane,
            wave95_promotion_root: stable_id(
                "wave95-promotion-root",
                &format!("{}:{}", lane.as_str(), config.wave95_promoted_slot_root),
            ),
            promoted_slot_root: stable_id("wave96-promoted-slot-placeholder", lane.as_str()),
            release_claim_placeholder_root: stable_id(
                "wave96-release-claim-placeholder",
                lane.as_str(),
            ),
            promoted_slot_count: 0,
            release_ready: false,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root("wave95_promotion_root", &self.wave95_promotion_root)?;
        ensure_root("promoted_slot_root", &self.promoted_slot_root)?;
        ensure_root(
            "release_claim_placeholder_root",
            &self.release_claim_placeholder_root,
        )?;
        if self.release_ready && self.promoted_slot_count == 0 {
            return Err("release-ready lane requires promoted slot count".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave96_promoted_slot_root_set",
            "lane": self.lane.as_str(),
            "wave95_promotion_root": self.wave95_promotion_root,
            "promoted_slot_root": self.promoted_slot_root,
            "release_claim_placeholder_root": self.release_claim_placeholder_root,
            "promoted_slot_count": self.promoted_slot_count,
            "release_ready": self.release_ready,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE96-PROMOTED-SLOT-ROOT-SET", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleaseReadinessClaim {
    pub lane: ReplayLane,
    pub status: QuorumStatus,
    pub promoted_slot_set_root: String,
    pub release_claim_root: String,
    pub blocker_roots: Vec<String>,
    pub command_hint_root: String,
}

impl ReleaseReadinessClaim {
    pub fn blocked(slot_set: &PromotedSlotRootSet) -> Self {
        Self {
            lane: slot_set.lane,
            status: QuorumStatus::Unmet,
            promoted_slot_set_root: slot_set.state_root(),
            release_claim_root: slot_set.release_claim_placeholder_root.clone(),
            blocker_roots: QuorumBlocker::all()
                .iter()
                .map(|blocker| blocker_root(slot_set.lane, *blocker))
                .collect(),
            command_hint_root: command_hint_root(slot_set.lane),
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root("promoted_slot_set_root", &self.promoted_slot_set_root)?;
        ensure_root("release_claim_root", &self.release_claim_root)?;
        ensure_root("command_hint_root", &self.command_hint_root)?;
        if self.status == QuorumStatus::Unmet && self.blocker_roots.is_empty() {
            return Err("unmet release quorum requires blocker roots".to_string());
        }
        for root in &self.blocker_roots {
            ensure_root("blocker_root", root)?;
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave96_release_readiness_claim",
            "lane": self.lane.as_str(),
            "status": self.status.as_str(),
            "promoted_slot_set_root": self.promoted_slot_set_root,
            "release_claim_root": self.release_claim_root,
            "blocker_root": list_root("release-readiness-blockers", self.blocker_roots.iter().cloned().collect()),
            "command_hint_root": self.command_hint_root,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE96-RELEASE-READINESS-CLAIM", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LaneReceipt {
    pub lane: ReplayLane,
    pub promoted_slot_set_root: String,
    pub release_claim_root: String,
    pub command_hint_root: String,
    pub replay_root: String,
}

impl LaneReceipt {
    pub fn new(slot_set: &PromotedSlotRootSet, claim: &ReleaseReadinessClaim) -> Self {
        let replay_root = domain_hash(
            "WAVE96-LANE-REPLAY-ROOT",
            &[
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(slot_set.lane.as_str()),
                HashPart::Str(&slot_set.state_root()),
                HashPart::Str(&claim.state_root()),
                HashPart::Str(&claim.command_hint_root),
            ],
            32,
        );
        Self {
            lane: slot_set.lane,
            promoted_slot_set_root: slot_set.state_root(),
            release_claim_root: claim.state_root(),
            command_hint_root: claim.command_hint_root.clone(),
            replay_root,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave96_lane_receipt",
            "lane": self.lane.as_str(),
            "promoted_slot_set_root": self.promoted_slot_set_root,
            "release_claim_root": self.release_claim_root,
            "command_hint_root": self.command_hint_root,
            "replay_root": self.replay_root,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE96-LANE-RECEIPT", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub lane_count: u64,
    pub release_ready_lane_count: u64,
    pub quorum_required: u64,
    pub quorum_met: bool,
    pub promoted_slot_count: u64,
    pub blocker_count: u64,
    pub release_claim_placeholder_count: u64,
    pub public_raw_records: u64,
}

impl Counters {
    pub fn from_claims(
        required_quorum: u64,
        sets: &[PromotedSlotRootSet],
        claims: &[ReleaseReadinessClaim],
    ) -> Self {
        let release_ready_lane_count = sets
            .iter()
            .filter(|slot_set| slot_set.release_ready)
            .count() as u64;
        let promoted_slot_count = sets
            .iter()
            .map(|slot_set| slot_set.promoted_slot_count)
            .fold(0_u64, u64::saturating_add);
        let blocker_count = claims
            .iter()
            .map(|claim| claim.blocker_roots.len() as u64)
            .fold(0_u64, u64::saturating_add);
        Self {
            lane_count: sets.len() as u64,
            release_ready_lane_count,
            quorum_required: required_quorum,
            quorum_met: release_ready_lane_count >= required_quorum,
            promoted_slot_count,
            blocker_count,
            release_claim_placeholder_count: claims
                .iter()
                .filter(|claim| claim.status == QuorumStatus::Unmet)
                .count() as u64,
            public_raw_records: 0,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave96_release_readiness_counters",
            "lane_count": self.lane_count,
            "release_ready_lane_count": self.release_ready_lane_count,
            "quorum_required": self.quorum_required,
            "quorum_met": self.quorum_met,
            "promoted_slot_count": self.promoted_slot_count,
            "blocker_count": self.blocker_count,
            "release_claim_placeholder_count": self.release_claim_placeholder_count,
            "public_raw_records": self.public_raw_records,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE96-COUNTERS", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub promoted_slot_sets: Vec<PromotedSlotRootSet>,
    pub release_claims: Vec<ReleaseReadinessClaim>,
    pub lane_receipts: Vec<LaneReceipt>,
    pub command_hints: BTreeMap<String, String>,
    pub counters: Counters,
}

impl State {
    pub fn new(
        config: Config,
        promoted_slot_sets: Vec<PromotedSlotRootSet>,
        release_claims: Vec<ReleaseReadinessClaim>,
        lane_receipts: Vec<LaneReceipt>,
        command_hints: BTreeMap<String, String>,
    ) -> Result<Self> {
        config.validate()?;
        if promoted_slot_sets.len() != ReplayLane::all().len() {
            return Err("release quorum requires one promoted slot root set per lane".to_string());
        }
        if release_claims.len() != promoted_slot_sets.len() {
            return Err("release claims must match promoted slot root sets".to_string());
        }
        if lane_receipts.len() != promoted_slot_sets.len() {
            return Err("lane receipts must match promoted slot root sets".to_string());
        }
        for slot_set in &promoted_slot_sets {
            slot_set.validate()?;
        }
        for claim in &release_claims {
            claim.validate()?;
        }
        for receipt in &lane_receipts {
            ensure_root(
                "receipt_promoted_slot_set_root",
                &receipt.promoted_slot_set_root,
            )?;
            ensure_root("receipt_release_claim_root", &receipt.release_claim_root)?;
            ensure_root("receipt_command_hint_root", &receipt.command_hint_root)?;
            ensure_root("receipt_replay_root", &receipt.replay_root)?;
        }
        for root in command_hints.values() {
            ensure_root("command_hint_root", root)?;
        }
        let counters =
            Counters::from_claims(config.required_quorum, &promoted_slot_sets, &release_claims);
        if counters.release_ready_lane_count != 0 {
            return Err(
                "devnet release readiness must keep release-ready lane count at zero".to_string(),
            );
        }
        if counters.quorum_met {
            return Err("devnet release readiness quorum must remain unmet".to_string());
        }
        if counters.promoted_slot_count != 0 {
            return Err(
                "devnet release readiness must use placeholder promoted slot roots".to_string(),
            );
        }
        if counters.public_raw_records > config.max_public_raw_records {
            return Err("public record contains raw material".to_string());
        }
        Ok(Self {
            config,
            promoted_slot_sets,
            release_claims,
            lane_receipts,
            command_hints,
            counters,
        })
    }

    pub fn devnet() -> Self {
        devnet()
    }

    pub fn public_record(&self) -> PublicRecord {
        let promoted_slot_roots = self
            .promoted_slot_sets
            .iter()
            .map(PromotedSlotRootSet::state_root)
            .collect::<Vec<_>>();
        let release_claim_roots = self
            .release_claims
            .iter()
            .map(ReleaseReadinessClaim::state_root)
            .collect::<Vec<_>>();
        let lane_receipt_roots = self
            .lane_receipts
            .iter()
            .map(LaneReceipt::state_root)
            .collect::<Vec<_>>();
        json!({
            "kind": "wave96_release_readiness_quorum_runtime_replay_lane",
            "config_root": self.config.state_root(),
            "promoted_slot_root": list_root("promoted-slot-root-sets", promoted_slot_roots),
            "release_claim_root": list_root("release-readiness-claims", release_claim_roots),
            "release_claim_placeholder_root": release_claim_placeholder_root(),
            "lane_receipt_root": list_root("lane-receipts", lane_receipt_roots),
            "command_hint_root": command_hints_root(&self.command_hints),
            "counter_root": self.counters.state_root(),
            "counters": self.counters.public_record(),
            "quorum_met": self.counters.quorum_met,
            "release_ready_lane_count": self.counters.release_ready_lane_count,
            "production_denied": self.config.production_denied,
            "fail_closed_armed": self.config.fail_closed_armed,
            "heavy_gates_ran": self.config.heavy_gates_ran,
            "state_root": self.state_root_without_public_record(),
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE96-STATE", &self.public_record())
    }

    fn state_root_without_public_record(&self) -> String {
        domain_hash(
            "WAVE96-STATE-INTERNAL",
            &[
                HashPart::Str(PROTOCOL_VERSION),
                HashPart::Str(&self.config.state_root()),
                HashPart::Str(&promoted_slot_sets_root(&self.promoted_slot_sets)),
                HashPart::Str(&release_claims_root(&self.release_claims)),
                HashPart::Str(&lane_receipts_root(&self.lane_receipts)),
                HashPart::Str(&command_hints_root(&self.command_hints)),
                HashPart::Str(&self.counters.state_root()),
            ],
            32,
        )
    }
}

pub fn devnet() -> Runtime {
    let config = Config::devnet();
    let promoted_slot_sets = ReplayLane::all()
        .iter()
        .map(|lane| PromotedSlotRootSet::placeholder(*lane, &config))
        .collect::<Vec<_>>();
    let release_claims = promoted_slot_sets
        .iter()
        .map(ReleaseReadinessClaim::blocked)
        .collect::<Vec<_>>();
    let lane_receipts = promoted_slot_sets
        .iter()
        .zip(release_claims.iter())
        .map(|(slot_set, claim)| LaneReceipt::new(slot_set, claim))
        .collect::<Vec<_>>();
    let command_hints = CommandHint::all()
        .iter()
        .map(|hint| (hint.as_str().to_string(), command_hint_kind_root(*hint)))
        .collect::<BTreeMap<_, _>>();
    match State::new(
        config,
        promoted_slot_sets,
        release_claims,
        lane_receipts,
        command_hints,
    ) {
        Ok(state) => state,
        Err(_) => State {
            config: Config::devnet(),
            promoted_slot_sets: Vec::new(),
            release_claims: Vec::new(),
            lane_receipts: Vec::new(),
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

fn promoted_slot_sets_root(sets: &[PromotedSlotRootSet]) -> String {
    list_root(
        "promoted-slot-set-state-roots",
        sets.iter().map(PromotedSlotRootSet::state_root).collect(),
    )
}

fn release_claims_root(claims: &[ReleaseReadinessClaim]) -> String {
    list_root(
        "release-claim-state-roots",
        claims
            .iter()
            .map(ReleaseReadinessClaim::state_root)
            .collect(),
    )
}

fn lane_receipts_root(receipts: &[LaneReceipt]) -> String {
    list_root(
        "lane-receipt-state-roots",
        receipts.iter().map(LaneReceipt::state_root).collect(),
    )
}

fn command_hints_root(hints: &BTreeMap<String, String>) -> String {
    let values = hints
        .iter()
        .map(|(hint, root)| {
            domain_hash(
                "WAVE96-COMMAND-HINT-ENTRY",
                &[
                    HashPart::Str(PROTOCOL_VERSION),
                    HashPart::Str(hint),
                    HashPart::Str(root),
                ],
                32,
            )
        })
        .collect::<Vec<_>>();
    list_root("command-hints", values)
}

fn command_hint_root(lane: ReplayLane) -> String {
    domain_hash(
        "WAVE96-LANE-COMMAND-HINT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
        ],
        32,
    )
}

fn command_hint_kind_root(hint: CommandHint) -> String {
    domain_hash(
        "WAVE96-COMMAND-HINT-KIND",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(hint.as_str()),
        ],
        32,
    )
}

fn blocker_root(lane: ReplayLane, blocker: QuorumBlocker) -> String {
    domain_hash(
        "WAVE96-QUORUM-BLOCKER",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(lane.as_str()),
            HashPart::Str(blocker.as_str()),
        ],
        32,
    )
}

fn release_claim_placeholder_root() -> String {
    domain_hash(
        "WAVE96-RELEASE-CLAIM-PLACEHOLDER-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str("zero-release-ready-lanes"),
        ],
        32,
    )
}

fn stable_id(domain: &str, label: &str) -> String {
    domain_hash(
        "WAVE96-STABLE-ROOT",
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
