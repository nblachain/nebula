use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;

pub const PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave98-live-heavy-gate-release-claim-challenge-window-holdoff-ledger-pq-reserve-privacy-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const HOLDOFF_LEDGER_SUITE: &str =
    "monero-l2-wave98-wave97-release-claim-challenge-window-holdoff-ledger-pq-reserve-privacy-v1";
pub const DEFAULT_WAVE: u64 = 98;
pub const RELEASE_CLAIM_SEAL_WAVE: u64 = 97;
pub const RELEASE_READINESS_WAVE: u64 = 96;
pub const DEFAULT_AUTHORITY_EPOCH: u64 = 98;
pub const DEFAULT_MIN_CHALLENGE_WINDOW_BLOCKS: u64 = 720;
pub const DEFAULT_MIN_APPEAL_WINDOW_BLOCKS: u64 = 1_440;
pub const DEFAULT_MIN_DISPUTE_BOND_BPS: u64 = 250;
pub const DEFAULT_MAX_LINKAGE_RISK_BPS: u64 = 25;
pub const DEFAULT_MIN_PRIVACY_BUDGET_REMAINING_BPS: u64 = 8_000;
pub const DEFAULT_MAX_LEDGER_ENTRIES: usize = 64;

const DOMAIN: &str =
    "monero-l2-pq-bridge-exit-force-exit-wave98-live-heavy-gate-release-claim-challenge-window-holdoff-ledger-pq-reserve-privacy-lane-runtime";

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneKind {
    PqReservePrivacy,
}

impl LaneKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PqReservePrivacy => "pq_reserve_privacy",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HoldoffEntryKind {
    Wave97ReleaseClaimSeal,
    ReserveChallenge,
    PrivacyObjection,
    PqAuthorityHold,
    DisputeBond,
    AppealDeadline,
    CommandHint,
}

impl HoldoffEntryKind {
    pub fn all() -> [Self; 7] {
        [
            Self::Wave97ReleaseClaimSeal,
            Self::ReserveChallenge,
            Self::PrivacyObjection,
            Self::PqAuthorityHold,
            Self::DisputeBond,
            Self::AppealDeadline,
            Self::CommandHint,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave97ReleaseClaimSeal => "wave97_release_claim_seal",
            Self::ReserveChallenge => "reserve_challenge",
            Self::PrivacyObjection => "privacy_objection",
            Self::PqAuthorityHold => "pq_authority_hold",
            Self::DisputeBond => "dispute_bond",
            Self::AppealDeadline => "appeal_deadline",
            Self::CommandHint => "command_hint",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HoldoffStatus {
    Open,
    Blocked,
    Held,
    ReleasedShadow,
}

impl HoldoffStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Blocked => "blocked",
            Self::Held => "held",
            Self::ReleasedShadow => "released_shadow",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HoldoffBlocker {
    HeavyGatesNotRun,
    ProductionDenied,
    ReleaseClaimsNotReleased,
    ChallengeWindowOpen,
    ChallengeWindowBlocked,
    ReserveChallengeActive,
    PrivacyObjectionActive,
    PqAuthorityHoldActive,
    DisputeBondMissing,
    AppealDeadlineOpen,
    Wave97SealRootMissing,
    ReserveChallengeRootMissing,
    PrivacyObjectionRootMissing,
    PqAuthorityHoldRootMissing,
    DisputeBondRootMissing,
    AppealDeadlineRootMissing,
    CommandHintRootMissing,
    DeterministicRootMissing,
    PrivacyBudgetTooLow,
    LinkageRiskTooHigh,
    RootsOnlyBoundary,
    DuplicateLedgerRoot,
    LedgerCapacityReached,
}

impl HoldoffBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HeavyGatesNotRun => "heavy_gates_not_run",
            Self::ProductionDenied => "production_denied",
            Self::ReleaseClaimsNotReleased => "release_claims_not_released",
            Self::ChallengeWindowOpen => "challenge_window_open",
            Self::ChallengeWindowBlocked => "challenge_window_blocked",
            Self::ReserveChallengeActive => "reserve_challenge_active",
            Self::PrivacyObjectionActive => "privacy_objection_active",
            Self::PqAuthorityHoldActive => "pq_authority_hold_active",
            Self::DisputeBondMissing => "dispute_bond_missing",
            Self::AppealDeadlineOpen => "appeal_deadline_open",
            Self::Wave97SealRootMissing => "wave97_seal_root_missing",
            Self::ReserveChallengeRootMissing => "reserve_challenge_root_missing",
            Self::PrivacyObjectionRootMissing => "privacy_objection_root_missing",
            Self::PqAuthorityHoldRootMissing => "pq_authority_hold_root_missing",
            Self::DisputeBondRootMissing => "dispute_bond_root_missing",
            Self::AppealDeadlineRootMissing => "appeal_deadline_root_missing",
            Self::CommandHintRootMissing => "command_hint_root_missing",
            Self::DeterministicRootMissing => "deterministic_root_missing",
            Self::PrivacyBudgetTooLow => "privacy_budget_too_low",
            Self::LinkageRiskTooHigh => "linkage_risk_too_high",
            Self::RootsOnlyBoundary => "roots_only_boundary",
            Self::DuplicateLedgerRoot => "duplicate_ledger_root",
            Self::LedgerCapacityReached => "ledger_capacity_reached",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeVerdict {
    FailClosed,
    HoldoffActive,
    ReleaseShadowReady,
}

impl RuntimeVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosed => "fail_closed",
            Self::HoldoffActive => "holdoff_active",
            Self::ReleaseShadowReady => "release_shadow_ready",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorCommand {
    ImportWave97PqReservePrivacySealRoots,
    AttachReserveChallengeRoots,
    AttachPrivacyObjectionRoots,
    AttachPqAuthorityHoldRoots,
    AttachDisputeBondRoots,
    AttachAppealDeadlineRoots,
    KeepChallengeWindowOpen,
    KeepPqReservePrivacyHoldsActive,
    PublishRootsOnlyHoldoffLedger,
}

impl OperatorCommand {
    pub fn sequence() -> Vec<Self> {
        vec![
            Self::ImportWave97PqReservePrivacySealRoots,
            Self::AttachReserveChallengeRoots,
            Self::AttachPrivacyObjectionRoots,
            Self::AttachPqAuthorityHoldRoots,
            Self::AttachDisputeBondRoots,
            Self::AttachAppealDeadlineRoots,
            Self::KeepChallengeWindowOpen,
            Self::KeepPqReservePrivacyHoldsActive,
            Self::PublishRootsOnlyHoldoffLedger,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ImportWave97PqReservePrivacySealRoots => {
                "import_wave97_pq_reserve_privacy_seal_roots"
            }
            Self::AttachReserveChallengeRoots => "attach_reserve_challenge_roots",
            Self::AttachPrivacyObjectionRoots => "attach_privacy_objection_roots",
            Self::AttachPqAuthorityHoldRoots => "attach_pq_authority_hold_roots",
            Self::AttachDisputeBondRoots => "attach_dispute_bond_roots",
            Self::AttachAppealDeadlineRoots => "attach_appeal_deadline_roots",
            Self::KeepChallengeWindowOpen => "keep_challenge_window_open",
            Self::KeepPqReservePrivacyHoldsActive => "keep_pq_reserve_privacy_holds_active",
            Self::PublishRootsOnlyHoldoffLedger => "publish_roots_only_holdoff_ledger",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub holdoff_ledger_suite: String,
    pub wave: u64,
    pub release_claim_seal_wave: u64,
    pub release_readiness_wave: u64,
    pub lane: LaneKind,
    pub authority_epoch: u64,
    pub min_challenge_window_blocks: u64,
    pub min_appeal_window_blocks: u64,
    pub min_dispute_bond_bps: u64,
    pub min_privacy_budget_remaining_bps: u64,
    pub max_linkage_risk_bps: u64,
    pub fail_closed: bool,
    pub heavy_gates_ran: bool,
    pub production_allowed: bool,
    pub release_allowed: bool,
    pub challenge_windows_open: bool,
    pub challenge_windows_blocked: bool,
    pub pq_reserve_hold_active: bool,
    pub privacy_lane_hold_active: bool,
    pub pq_authority_hold_active: bool,
    pub roots_only_public_record: bool,
    pub max_ledger_entries: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            holdoff_ledger_suite: HOLDOFF_LEDGER_SUITE.to_string(),
            wave: DEFAULT_WAVE,
            release_claim_seal_wave: RELEASE_CLAIM_SEAL_WAVE,
            release_readiness_wave: RELEASE_READINESS_WAVE,
            lane: LaneKind::PqReservePrivacy,
            authority_epoch: DEFAULT_AUTHORITY_EPOCH,
            min_challenge_window_blocks: DEFAULT_MIN_CHALLENGE_WINDOW_BLOCKS,
            min_appeal_window_blocks: DEFAULT_MIN_APPEAL_WINDOW_BLOCKS,
            min_dispute_bond_bps: DEFAULT_MIN_DISPUTE_BOND_BPS,
            min_privacy_budget_remaining_bps: DEFAULT_MIN_PRIVACY_BUDGET_REMAINING_BPS,
            max_linkage_risk_bps: DEFAULT_MAX_LINKAGE_RISK_BPS,
            fail_closed: true,
            heavy_gates_ran: false,
            production_allowed: false,
            release_allowed: false,
            challenge_windows_open: true,
            challenge_windows_blocked: true,
            pq_reserve_hold_active: true,
            privacy_lane_hold_active: true,
            pq_authority_hold_active: true,
            roots_only_public_record: true,
            max_ledger_entries: DEFAULT_MAX_LEDGER_ENTRIES,
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
            "holdoff_ledger_suite": self.holdoff_ledger_suite,
            "wave": self.wave,
            "release_claim_seal_wave": self.release_claim_seal_wave,
            "release_readiness_wave": self.release_readiness_wave,
            "lane": self.lane.as_str(),
            "authority_epoch": self.authority_epoch,
            "min_challenge_window_blocks": self.min_challenge_window_blocks,
            "min_appeal_window_blocks": self.min_appeal_window_blocks,
            "min_dispute_bond_bps": self.min_dispute_bond_bps,
            "min_privacy_budget_remaining_bps": self.min_privacy_budget_remaining_bps,
            "max_linkage_risk_bps": self.max_linkage_risk_bps,
            "fail_closed": self.fail_closed,
            "heavy_gates_ran": self.heavy_gates_ran,
            "production_allowed": self.production_allowed,
            "release_allowed": self.release_allowed,
            "challenge_windows_open": self.challenge_windows_open,
            "challenge_windows_blocked": self.challenge_windows_blocked,
            "pq_reserve_hold_active": self.pq_reserve_hold_active,
            "privacy_lane_hold_active": self.privacy_lane_hold_active,
            "pq_authority_hold_active": self.pq_authority_hold_active,
            "roots_only_public_record": self.roots_only_public_record,
            "max_ledger_entries": self.max_ledger_entries,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct HoldoffRoots {
    pub wave97_release_claim_seal_root: String,
    pub wave97_pq_reserve_privacy_lane_root: String,
    pub reserve_challenge_root: String,
    pub reserve_challenge_evidence_root: String,
    pub privacy_objection_root: String,
    pub privacy_objection_evidence_root: String,
    pub pq_authority_hold_root: String,
    pub pq_authority_epoch_root: String,
    pub dispute_bond_root: String,
    pub dispute_bond_escrow_root: String,
    pub appeal_deadline_root: String,
    pub appeal_clock_root: String,
    pub command_hint_root: String,
    pub deterministic_holdoff_root: String,
}

impl HoldoffRoots {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn devnet() -> Self {
        Self {
            wave97_release_claim_seal_root: sample_root("wave97-release-claim-seal"),
            wave97_pq_reserve_privacy_lane_root: sample_root("wave97-pq-reserve-privacy-lane"),
            reserve_challenge_root: sample_root("reserve-challenge-open"),
            reserve_challenge_evidence_root: sample_root("reserve-challenge-evidence-root"),
            privacy_objection_root: sample_root("privacy-objection-open"),
            privacy_objection_evidence_root: sample_root("privacy-objection-evidence-root"),
            pq_authority_hold_root: sample_root("pq-authority-hold-active"),
            pq_authority_epoch_root: sample_root("pq-authority-epoch-root"),
            dispute_bond_root: sample_root("dispute-bond-held"),
            dispute_bond_escrow_root: sample_root("dispute-bond-escrow-root"),
            appeal_deadline_root: sample_root("appeal-deadline-open"),
            appeal_clock_root: sample_root("appeal-clock-root"),
            command_hint_root: operator_command_root(&OperatorCommand::sequence()),
            deterministic_holdoff_root: deterministic_holdoff_root("devnet"),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "wave97_release_claim_seal_root": self.wave97_release_claim_seal_root,
            "wave97_pq_reserve_privacy_lane_root": self.wave97_pq_reserve_privacy_lane_root,
            "reserve_challenge_root": self.reserve_challenge_root,
            "reserve_challenge_evidence_root": self.reserve_challenge_evidence_root,
            "privacy_objection_root": self.privacy_objection_root,
            "privacy_objection_evidence_root": self.privacy_objection_evidence_root,
            "pq_authority_hold_root": self.pq_authority_hold_root,
            "pq_authority_epoch_root": self.pq_authority_epoch_root,
            "dispute_bond_root": self.dispute_bond_root,
            "dispute_bond_escrow_root": self.dispute_bond_escrow_root,
            "appeal_deadline_root": self.appeal_deadline_root,
            "appeal_clock_root": self.appeal_clock_root,
            "command_hint_root": self.command_hint_root,
            "deterministic_holdoff_root": self.deterministic_holdoff_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("holdoff_roots", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HoldoffMetrics {
    pub challenge_window_blocks_remaining: u64,
    pub appeal_window_blocks_remaining: u64,
    pub dispute_bond_bps: u64,
    pub reserve_challenge_count: u64,
    pub privacy_objection_count: u64,
    pub pq_authority_epoch: u64,
    pub privacy_budget_remaining_bps: u64,
    pub linkage_risk_bps: u64,
    pub release_claims_released: bool,
    pub reserve_challenges_resolved: bool,
    pub privacy_objections_resolved: bool,
    pub pq_authority_hold_resolved: bool,
    pub dispute_bond_locked: bool,
    pub appeal_deadline_elapsed: bool,
    pub root_only: bool,
}

impl Default for HoldoffMetrics {
    fn default() -> Self {
        Self {
            challenge_window_blocks_remaining: DEFAULT_MIN_CHALLENGE_WINDOW_BLOCKS,
            appeal_window_blocks_remaining: DEFAULT_MIN_APPEAL_WINDOW_BLOCKS,
            dispute_bond_bps: 0,
            reserve_challenge_count: 1,
            privacy_objection_count: 1,
            pq_authority_epoch: DEFAULT_AUTHORITY_EPOCH.saturating_sub(1),
            privacy_budget_remaining_bps: 0,
            linkage_risk_bps: DEFAULT_MAX_LINKAGE_RISK_BPS.saturating_add(1),
            release_claims_released: false,
            reserve_challenges_resolved: false,
            privacy_objections_resolved: false,
            pq_authority_hold_resolved: false,
            dispute_bond_locked: false,
            appeal_deadline_elapsed: false,
            root_only: true,
        }
    }
}

impl HoldoffMetrics {
    pub fn public_record(&self) -> Value {
        json!(self)
    }

    pub fn state_root(&self) -> String {
        record_root("holdoff_metrics", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HoldoffLedgerEntry {
    pub ledger_root: String,
    pub entry_kind: HoldoffEntryKind,
    pub status: HoldoffStatus,
    pub roots: HoldoffRoots,
    pub metrics: HoldoffMetrics,
    pub blockers: Vec<HoldoffBlocker>,
}

impl HoldoffLedgerEntry {
    pub fn evaluate(
        config: &Config,
        entry_kind: HoldoffEntryKind,
        roots: HoldoffRoots,
        metrics: HoldoffMetrics,
    ) -> Self {
        let blockers = holdoff_blockers(config, &roots, &metrics);
        let status = if blockers.is_empty() {
            HoldoffStatus::ReleasedShadow
        } else if config.challenge_windows_open {
            HoldoffStatus::Open
        } else if config.pq_reserve_hold_active
            || config.privacy_lane_hold_active
            || config.pq_authority_hold_active
        {
            HoldoffStatus::Held
        } else {
            HoldoffStatus::Blocked
        };
        let ledger_root = entry_root(entry_kind, status, &roots, &metrics, &blockers);
        Self {
            ledger_root,
            entry_kind,
            status,
            roots,
            metrics,
            blockers,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "ledger_root": self.ledger_root,
            "entry_kind": self.entry_kind.as_str(),
            "status": self.status.as_str(),
            "roots": self.roots.public_record(),
            "metrics": self.metrics.public_record(),
            "blocker_root": blocker_root(&self.blockers),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("holdoff_ledger_entry", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EntryState {
    pub entry_kind: HoldoffEntryKind,
    pub status: HoldoffStatus,
    pub ledger_root: String,
    pub blocker_root: String,
    pub release_denied: bool,
    pub command_hint_root: String,
}

impl EntryState {
    pub fn blocked(entry_kind: HoldoffEntryKind) -> Self {
        let blockers = default_active_blockers();
        Self {
            entry_kind,
            status: HoldoffStatus::Open,
            ledger_root: empty_root("entry", entry_kind.as_str()),
            blocker_root: blocker_root(&blockers),
            release_denied: true,
            command_hint_root: command_hint_root(entry_kind),
        }
    }

    pub fn from_entry(entry: &HoldoffLedgerEntry) -> Self {
        Self {
            entry_kind: entry.entry_kind,
            status: entry.status,
            ledger_root: entry.ledger_root.clone(),
            blocker_root: blocker_root(&entry.blockers),
            release_denied: entry.status != HoldoffStatus::ReleasedShadow,
            command_hint_root: command_hint_root(entry.entry_kind),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "entry_kind": self.entry_kind.as_str(),
            "status": self.status.as_str(),
            "ledger_root": self.ledger_root,
            "blocker_root": self.blocker_root,
            "release_denied": self.release_denied,
            "command_hint_root": self.command_hint_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("entry_state", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct HoldoffCounters {
    pub ledger_entry_count: u64,
    pub open_count: u64,
    pub blocked_count: u64,
    pub held_count: u64,
    pub release_shadow_ready_count: u64,
    pub release_denied_count: u64,
}

impl HoldoffCounters {
    pub fn from_parts(
        entries: &[HoldoffLedgerEntry],
        entry_states: &BTreeMap<HoldoffEntryKind, EntryState>,
    ) -> Self {
        let mut counters = Self::default();
        for entry in entries {
            counters.ledger_entry_count = counters.ledger_entry_count.saturating_add(1);
            match entry.status {
                HoldoffStatus::Open => counters.open_count = counters.open_count.saturating_add(1),
                HoldoffStatus::Blocked => {
                    counters.blocked_count = counters.blocked_count.saturating_add(1);
                }
                HoldoffStatus::Held => counters.held_count = counters.held_count.saturating_add(1),
                HoldoffStatus::ReleasedShadow => {
                    counters.release_shadow_ready_count =
                        counters.release_shadow_ready_count.saturating_add(1);
                }
            }
        }
        counters.release_denied_count = entry_states
            .values()
            .filter(|entry| entry.release_denied)
            .count() as u64;
        counters
    }

    pub fn public_record(&self) -> Value {
        json!(self)
    }

    pub fn state_root(&self) -> String {
        record_root("holdoff_counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub ledger_entries: Vec<HoldoffLedgerEntry>,
    pub entry_states: BTreeMap<HoldoffEntryKind, EntryState>,
    pub counters: HoldoffCounters,
    pub operator_commands: Vec<OperatorCommand>,
}

impl State {
    pub fn new(config: Config, ledger_entries: Vec<HoldoffLedgerEntry>) -> Result<Self> {
        if ledger_entries.len() > config.max_ledger_entries {
            return Err(HoldoffBlocker::LedgerCapacityReached.as_str().to_string());
        }
        let mut seen = BTreeSet::new();
        for entry in &ledger_entries {
            if !seen.insert(entry.ledger_root.clone()) {
                return Err(HoldoffBlocker::DuplicateLedgerRoot.as_str().to_string());
            }
        }
        let mut entry_states = BTreeMap::new();
        for entry_kind in HoldoffEntryKind::all() {
            let maybe_entry = ledger_entries
                .iter()
                .rev()
                .find(|entry| entry.entry_kind == entry_kind);
            let state = match maybe_entry {
                Some(entry) => EntryState::from_entry(entry),
                None => EntryState::blocked(entry_kind),
            };
            entry_states.insert(entry_kind, state);
        }
        let counters = HoldoffCounters::from_parts(&ledger_entries, &entry_states);
        Ok(Self {
            config,
            ledger_entries,
            entry_states,
            counters,
            operator_commands: OperatorCommand::sequence(),
        })
    }

    pub fn devnet() -> Self {
        devnet()
    }

    pub fn append_holdoff_entry(
        &self,
        entry_kind: HoldoffEntryKind,
        roots: HoldoffRoots,
        metrics: HoldoffMetrics,
    ) -> Result<Self> {
        let mut ledger_entries = self.ledger_entries.clone();
        if ledger_entries.len() >= self.config.max_ledger_entries {
            return Err(HoldoffBlocker::LedgerCapacityReached.as_str().to_string());
        }
        let entry = HoldoffLedgerEntry::evaluate(&self.config, entry_kind, roots, metrics);
        if ledger_entries
            .iter()
            .any(|item| item.ledger_root == entry.ledger_root)
        {
            return Err(HoldoffBlocker::DuplicateLedgerRoot.as_str().to_string());
        }
        ledger_entries.push(entry);
        Self::new(self.config.clone(), ledger_entries)
    }

    pub fn verdict(&self) -> RuntimeVerdict {
        if self.config.fail_closed && self.ledger_entries.is_empty() {
            return RuntimeVerdict::FailClosed;
        }
        if self.counters.release_denied_count == 0
            && self.counters.release_shadow_ready_count == HoldoffEntryKind::all().len() as u64
        {
            RuntimeVerdict::ReleaseShadowReady
        } else {
            RuntimeVerdict::HoldoffActive
        }
    }

    pub fn public_record(&self) -> Value {
        let entry_records = self
            .entry_states
            .values()
            .map(EntryState::public_record)
            .collect::<Vec<_>>();
        let ledger_records = self
            .ledger_entries
            .iter()
            .map(HoldoffLedgerEntry::public_record)
            .collect::<Vec<_>>();
        json!({
            "config_root": self.config.state_root(),
            "entry_state_root": list_root("entry_states", entry_records),
            "holdoff_ledger_root": list_root("holdoff_ledger_entries", ledger_records),
            "wave97_release_claim_seal_root": wave97_release_claim_seal_root(&self.ledger_entries),
            "reserve_challenge_root": reserve_challenge_root(&self.ledger_entries),
            "privacy_objection_root": privacy_objection_root(&self.ledger_entries),
            "pq_authority_hold_root": pq_authority_hold_root(&self.ledger_entries),
            "dispute_bond_root": dispute_bond_root(&self.ledger_entries),
            "appeal_deadline_root": appeal_deadline_root(&self.ledger_entries),
            "command_hint_root": operator_command_root(&self.operator_commands),
            "deterministic_root": deterministic_state_root(&self.ledger_entries, &self.entry_states),
            "counter_root": self.counters.state_root(),
            "blocker_root": all_blockers_root(&self.ledger_entries, &self.entry_states),
            "active_holdoff_blocker_root": blocker_root(&default_active_blockers()),
            "verdict": self.verdict().as_str(),
            "release_denied": self.verdict() != RuntimeVerdict::ReleaseShadowReady,
            "released_claim_count": 0_u64,
            "counters": self.counters.public_record(),
            "state_root": self.state_root_without_public_record(),
        })
    }

    pub fn state_root(&self) -> String {
        domain_hash(
            &format!("{DOMAIN}:state"),
            &[HashPart::Json(&self.public_record())],
            32,
        )
    }

    fn state_root_without_public_record(&self) -> String {
        domain_hash(
            &format!("{DOMAIN}:state-core"),
            &[
                HashPart::Str(&self.config.state_root()),
                HashPart::Str(&entries_root(&self.ledger_entries)),
                HashPart::Str(&entry_states_root(&self.entry_states)),
                HashPart::Str(&self.counters.state_root()),
                HashPart::Str(&blocker_root(&default_active_blockers())),
                HashPart::Str(&operator_command_root(&self.operator_commands)),
            ],
            32,
        )
    }
}

pub fn devnet() -> Runtime {
    let config = Config::devnet();
    let roots = HoldoffRoots::devnet();
    let metrics = HoldoffMetrics::default();
    let entries = HoldoffEntryKind::all()
        .into_iter()
        .map(|entry_kind| {
            HoldoffLedgerEntry::evaluate(&config, entry_kind, roots.clone(), metrics.clone())
        })
        .collect::<Vec<_>>();
    match State::new(config.clone(), entries) {
        Ok(state) => state,
        Err(reason) => {
            let fallback_roots = HoldoffRoots {
                deterministic_holdoff_root: record_root(
                    "closed_state_reason",
                    &json!({ "root": reason }),
                ),
                ..HoldoffRoots::empty()
            };
            let fallback_entry = HoldoffLedgerEntry::evaluate(
                &config,
                HoldoffEntryKind::CommandHint,
                fallback_roots,
                HoldoffMetrics::default(),
            );
            State {
                config,
                ledger_entries: Vec::new(),
                entry_states: HoldoffEntryKind::all()
                    .into_iter()
                    .map(|entry_kind| (entry_kind, EntryState::blocked(entry_kind)))
                    .collect(),
                counters: HoldoffCounters::from_parts(&[fallback_entry], &BTreeMap::new()),
                operator_commands: OperatorCommand::sequence(),
            }
        }
    }
}

pub fn public_record() -> Value {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn holdoff_blockers(
    config: &Config,
    roots: &HoldoffRoots,
    metrics: &HoldoffMetrics,
) -> Vec<HoldoffBlocker> {
    let mut blockers = Vec::new();
    if !config.heavy_gates_ran {
        blockers.push(HoldoffBlocker::HeavyGatesNotRun);
    }
    if !config.production_allowed || !config.release_allowed {
        blockers.push(HoldoffBlocker::ProductionDenied);
    }
    if !metrics.release_claims_released {
        blockers.push(HoldoffBlocker::ReleaseClaimsNotReleased);
    }
    if config.challenge_windows_open || metrics.challenge_window_blocks_remaining > 0 {
        blockers.push(HoldoffBlocker::ChallengeWindowOpen);
    }
    if config.challenge_windows_blocked {
        blockers.push(HoldoffBlocker::ChallengeWindowBlocked);
    }
    if config.pq_reserve_hold_active || !metrics.reserve_challenges_resolved {
        blockers.push(HoldoffBlocker::ReserveChallengeActive);
    }
    if config.privacy_lane_hold_active || !metrics.privacy_objections_resolved {
        blockers.push(HoldoffBlocker::PrivacyObjectionActive);
    }
    if config.pq_authority_hold_active || !metrics.pq_authority_hold_resolved {
        blockers.push(HoldoffBlocker::PqAuthorityHoldActive);
    }
    if !metrics.dispute_bond_locked || metrics.dispute_bond_bps < config.min_dispute_bond_bps {
        blockers.push(HoldoffBlocker::DisputeBondMissing);
    }
    if !metrics.appeal_deadline_elapsed || metrics.appeal_window_blocks_remaining > 0 {
        blockers.push(HoldoffBlocker::AppealDeadlineOpen);
    }
    if roots.wave97_release_claim_seal_root.is_empty()
        || roots.wave97_pq_reserve_privacy_lane_root.is_empty()
    {
        blockers.push(HoldoffBlocker::Wave97SealRootMissing);
    }
    if roots.reserve_challenge_root.is_empty() || roots.reserve_challenge_evidence_root.is_empty() {
        blockers.push(HoldoffBlocker::ReserveChallengeRootMissing);
    }
    if roots.privacy_objection_root.is_empty() || roots.privacy_objection_evidence_root.is_empty() {
        blockers.push(HoldoffBlocker::PrivacyObjectionRootMissing);
    }
    if roots.pq_authority_hold_root.is_empty() || roots.pq_authority_epoch_root.is_empty() {
        blockers.push(HoldoffBlocker::PqAuthorityHoldRootMissing);
    }
    if roots.dispute_bond_root.is_empty() || roots.dispute_bond_escrow_root.is_empty() {
        blockers.push(HoldoffBlocker::DisputeBondRootMissing);
    }
    if roots.appeal_deadline_root.is_empty() || roots.appeal_clock_root.is_empty() {
        blockers.push(HoldoffBlocker::AppealDeadlineRootMissing);
    }
    if roots.command_hint_root.is_empty() {
        blockers.push(HoldoffBlocker::CommandHintRootMissing);
    }
    if roots.deterministic_holdoff_root.is_empty() {
        blockers.push(HoldoffBlocker::DeterministicRootMissing);
    }
    if metrics.privacy_budget_remaining_bps < config.min_privacy_budget_remaining_bps {
        blockers.push(HoldoffBlocker::PrivacyBudgetTooLow);
    }
    if metrics.linkage_risk_bps > config.max_linkage_risk_bps {
        blockers.push(HoldoffBlocker::LinkageRiskTooHigh);
    }
    if !metrics.root_only || !config.roots_only_public_record {
        blockers.push(HoldoffBlocker::RootsOnlyBoundary);
    }
    dedupe_blockers(&mut blockers);
    blockers
}

fn default_active_blockers() -> Vec<HoldoffBlocker> {
    vec![
        HoldoffBlocker::HeavyGatesNotRun,
        HoldoffBlocker::ProductionDenied,
        HoldoffBlocker::ReleaseClaimsNotReleased,
        HoldoffBlocker::ChallengeWindowOpen,
        HoldoffBlocker::ChallengeWindowBlocked,
        HoldoffBlocker::ReserveChallengeActive,
        HoldoffBlocker::PrivacyObjectionActive,
        HoldoffBlocker::PqAuthorityHoldActive,
        HoldoffBlocker::DisputeBondMissing,
        HoldoffBlocker::AppealDeadlineOpen,
    ]
}

fn entry_root(
    entry_kind: HoldoffEntryKind,
    status: HoldoffStatus,
    roots: &HoldoffRoots,
    metrics: &HoldoffMetrics,
    blockers: &[HoldoffBlocker],
) -> String {
    domain_hash(
        &format!("{DOMAIN}:holdoff-ledger-entry"),
        &[
            HashPart::Str(entry_kind.as_str()),
            HashPart::Str(status.as_str()),
            HashPart::Str(&roots.state_root()),
            HashPart::Str(&metrics.state_root()),
            HashPart::Str(&blocker_root(blockers)),
        ],
        32,
    )
}

fn entries_root(entries: &[HoldoffLedgerEntry]) -> String {
    let leaves = entries
        .iter()
        .map(|entry| Value::String(entry.state_root()))
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:entries"), &leaves)
}

fn entry_states_root(entry_states: &BTreeMap<HoldoffEntryKind, EntryState>) -> String {
    let leaves = entry_states
        .values()
        .map(|entry| Value::String(entry.state_root()))
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:entry-states"), &leaves)
}

fn wave97_release_claim_seal_root(entries: &[HoldoffLedgerEntry]) -> String {
    let leaves = entries
        .iter()
        .map(|entry| {
            json!({
                "entry_kind": entry.entry_kind.as_str(),
                "wave97_release_claim_seal_root": entry.roots.wave97_release_claim_seal_root,
                "wave97_pq_reserve_privacy_lane_root": entry.roots.wave97_pq_reserve_privacy_lane_root,
            })
        })
        .collect::<Vec<_>>();
    merkle_root(
        &format!("{DOMAIN}:wave97-release-claim-seal-roots"),
        &leaves,
    )
}

fn reserve_challenge_root(entries: &[HoldoffLedgerEntry]) -> String {
    let leaves = entries
        .iter()
        .map(|entry| {
            json!({
                "entry_kind": entry.entry_kind.as_str(),
                "reserve_challenge_root": entry.roots.reserve_challenge_root,
                "reserve_challenge_evidence_root": entry.roots.reserve_challenge_evidence_root,
                "reserve_challenge_count": entry.metrics.reserve_challenge_count,
            })
        })
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:reserve-challenge-roots"), &leaves)
}

fn privacy_objection_root(entries: &[HoldoffLedgerEntry]) -> String {
    let leaves = entries
        .iter()
        .map(|entry| {
            json!({
                "entry_kind": entry.entry_kind.as_str(),
                "privacy_objection_root": entry.roots.privacy_objection_root,
                "privacy_objection_evidence_root": entry.roots.privacy_objection_evidence_root,
                "privacy_objection_count": entry.metrics.privacy_objection_count,
            })
        })
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:privacy-objection-roots"), &leaves)
}

fn pq_authority_hold_root(entries: &[HoldoffLedgerEntry]) -> String {
    let leaves = entries
        .iter()
        .map(|entry| {
            json!({
                "entry_kind": entry.entry_kind.as_str(),
                "pq_authority_hold_root": entry.roots.pq_authority_hold_root,
                "pq_authority_epoch_root": entry.roots.pq_authority_epoch_root,
                "pq_authority_epoch": entry.metrics.pq_authority_epoch,
            })
        })
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:pq-authority-hold-roots"), &leaves)
}

fn dispute_bond_root(entries: &[HoldoffLedgerEntry]) -> String {
    let leaves = entries
        .iter()
        .map(|entry| {
            json!({
                "entry_kind": entry.entry_kind.as_str(),
                "dispute_bond_root": entry.roots.dispute_bond_root,
                "dispute_bond_escrow_root": entry.roots.dispute_bond_escrow_root,
                "dispute_bond_bps": entry.metrics.dispute_bond_bps,
            })
        })
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:dispute-bond-roots"), &leaves)
}

fn appeal_deadline_root(entries: &[HoldoffLedgerEntry]) -> String {
    let leaves = entries
        .iter()
        .map(|entry| {
            json!({
                "entry_kind": entry.entry_kind.as_str(),
                "appeal_deadline_root": entry.roots.appeal_deadline_root,
                "appeal_clock_root": entry.roots.appeal_clock_root,
                "appeal_window_blocks_remaining": entry.metrics.appeal_window_blocks_remaining,
            })
        })
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:appeal-deadline-roots"), &leaves)
}

fn deterministic_state_root(
    entries: &[HoldoffLedgerEntry],
    entry_states: &BTreeMap<HoldoffEntryKind, EntryState>,
) -> String {
    domain_hash(
        &format!("{DOMAIN}:deterministic-state"),
        &[
            HashPart::Str(&entries_root(entries)),
            HashPart::Str(&entry_states_root(entry_states)),
            HashPart::Str(&blocker_root(&default_active_blockers())),
        ],
        32,
    )
}

fn all_blockers_root(
    entries: &[HoldoffLedgerEntry],
    entry_states: &BTreeMap<HoldoffEntryKind, EntryState>,
) -> String {
    let mut leaves = entries
        .iter()
        .flat_map(|entry| {
            entry.blockers.iter().map(|blocker| {
                json!({
                    "ledger_root": entry.ledger_root,
                    "blocker": blocker.as_str(),
                })
            })
        })
        .collect::<Vec<_>>();
    leaves.extend(
        entry_states
            .values()
            .filter(|entry| entry.release_denied)
            .map(|entry| {
                json!({
                    "entry_kind": entry.entry_kind.as_str(),
                    "blocker_root": entry.blocker_root,
                })
            }),
    );
    merkle_root(&format!("{DOMAIN}:all-blockers"), &leaves)
}

fn blocker_root(blockers: &[HoldoffBlocker]) -> String {
    let leaves = blockers
        .iter()
        .map(|blocker| json!({ "blocker": blocker.as_str() }))
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:blockers"), &leaves)
}

fn operator_command_root(commands: &[OperatorCommand]) -> String {
    let leaves = commands
        .iter()
        .map(|command| json!({ "command": command.as_str() }))
        .collect::<Vec<_>>();
    merkle_root(&format!("{DOMAIN}:operator-commands"), &leaves)
}

fn command_hint_root(entry_kind: HoldoffEntryKind) -> String {
    domain_hash(
        &format!("{DOMAIN}:command-hint"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(entry_kind.as_str()),
            HashPart::Str(HOLDOFF_LEDGER_SUITE),
        ],
        32,
    )
}

fn deterministic_holdoff_root(label: &str) -> String {
    domain_hash(
        &format!("{DOMAIN}:deterministic-holdoff-root"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(label),
            HashPart::U64(DEFAULT_WAVE),
        ],
        32,
    )
}

fn sample_root(label: &str) -> String {
    domain_hash(
        &format!("{DOMAIN}:sample-root"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(label),
        ],
        32,
    )
}

fn list_root(kind: &str, leaves: Vec<Value>) -> String {
    merkle_root(&format!("{DOMAIN}:{kind}"), &leaves)
}

fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        &format!("{DOMAIN}:record"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Json(record),
        ],
        32,
    )
}

fn empty_root(kind: &str, entry_kind: &str) -> String {
    domain_hash(
        &format!("{DOMAIN}:empty"),
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(kind),
            HashPart::Str(entry_kind),
        ],
        32,
    )
}

fn dedupe_blockers(blockers: &mut Vec<HoldoffBlocker>) {
    let mut seen = BTreeSet::new();
    blockers.retain(|blocker| seen.insert(*blocker));
}
