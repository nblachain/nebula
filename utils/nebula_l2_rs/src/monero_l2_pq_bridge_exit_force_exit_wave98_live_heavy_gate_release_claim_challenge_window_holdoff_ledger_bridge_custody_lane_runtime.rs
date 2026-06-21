use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::hash::{domain_hash, merkle_root, HashPart};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;
pub type PublicRecord = Value;

pub const PROTOCOL_VERSION: &str =
    "monero-l2-pq-bridge-force-exit-wave98-release-claim-challenge-window-holdoff-ledger-bridge-custody-v1";
pub const DEVNET_CHAIN_ID: &str = "nebula-devnet";
pub const DEVNET_LANE_ID: &str = "bridge-custody-force-exit";
pub const WAVE96_RELEASE_READINESS_ROOT: &str =
    "root:wave96:bridge-custody-release-readiness-quorum-denied-placeholder";
pub const WAVE97_RELEASE_CLAIM_SEAL_ROOT: &str =
    "root:wave97:bridge-custody-release-claim-anti-equivocation-seal-blocked-placeholder";
pub const DEFAULT_CHALLENGE_WINDOW_BLOCKS: u64 = 2_880;
pub const DEFAULT_MIN_WATCHER_CHALLENGES: u16 = 4;
pub const DEFAULT_MIN_CUSTODY_OBJECTIONS: u16 = 3;
pub const DEFAULT_MIN_RESERVE_HOLDS: u16 = 3;
pub const DEFAULT_MIN_DISPUTE_BONDS: u16 = 2;
pub const DEFAULT_MIN_APPEAL_DEADLINES: u16 = 2;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub lane_id: String,
    pub protocol_version: String,
    pub wave96_release_readiness_root: String,
    pub wave97_release_claim_seal_root: String,
    pub challenge_window_blocks: u64,
    pub min_watcher_challenges: u16,
    pub min_custody_objections: u16,
    pub min_reserve_holds: u16,
    pub min_dispute_bonds: u16,
    pub min_appeal_deadlines: u16,
    pub roots_only_public_records: bool,
    pub release_enabled: bool,
    pub release_claims_enabled: bool,
    pub challenge_window_open: bool,
    pub custody_holds_active: bool,
    pub heavy_gates_ran: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: DEVNET_CHAIN_ID.to_string(),
            lane_id: DEVNET_LANE_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            wave96_release_readiness_root: WAVE96_RELEASE_READINESS_ROOT.to_string(),
            wave97_release_claim_seal_root: WAVE97_RELEASE_CLAIM_SEAL_ROOT.to_string(),
            challenge_window_blocks: DEFAULT_CHALLENGE_WINDOW_BLOCKS,
            min_watcher_challenges: DEFAULT_MIN_WATCHER_CHALLENGES,
            min_custody_objections: DEFAULT_MIN_CUSTODY_OBJECTIONS,
            min_reserve_holds: DEFAULT_MIN_RESERVE_HOLDS,
            min_dispute_bonds: DEFAULT_MIN_DISPUTE_BONDS,
            min_appeal_deadlines: DEFAULT_MIN_APPEAL_DEADLINES,
            roots_only_public_records: true,
            release_enabled: false,
            release_claims_enabled: false,
            challenge_window_open: true,
            custody_holds_active: true,
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
        ensure_root_like(
            "wave96_release_readiness_root",
            &self.wave96_release_readiness_root,
        )?;
        ensure_root_like(
            "wave97_release_claim_seal_root",
            &self.wave97_release_claim_seal_root,
        )?;
        if self.challenge_window_blocks == 0 {
            return Err("challenge window must be nonzero".to_string());
        }
        if self.min_watcher_challenges == 0 {
            return Err("watcher challenge quorum must be nonzero".to_string());
        }
        if self.min_custody_objections == 0 {
            return Err("custody objection quorum must be nonzero".to_string());
        }
        if self.min_reserve_holds == 0 {
            return Err("reserve hold quorum must be nonzero".to_string());
        }
        if self.min_dispute_bonds == 0 {
            return Err("dispute bond quorum must be nonzero".to_string());
        }
        if self.min_appeal_deadlines == 0 {
            return Err("appeal deadline quorum must be nonzero".to_string());
        }
        if !self.roots_only_public_records {
            return Err("public records must remain roots only".to_string());
        }
        if self.release_enabled || self.release_claims_enabled {
            return Err("challenge-window holdoff must keep release disabled".to_string());
        }
        if !self.challenge_window_open {
            return Err("devnet challenge window must remain open".to_string());
        }
        if !self.custody_holds_active {
            return Err("devnet custody holds must remain active".to_string());
        }
        if self.heavy_gates_ran {
            return Err("wave98 must not claim heavy gates ran".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "chain_id": self.chain_id,
            "lane_id": self.lane_id,
            "protocol_version": self.protocol_version,
            "wave96_release_readiness_root": self.wave96_release_readiness_root,
            "wave97_release_claim_seal_root": self.wave97_release_claim_seal_root,
            "challenge_window_blocks": self.challenge_window_blocks,
            "min_watcher_challenges": self.min_watcher_challenges,
            "min_custody_objections": self.min_custody_objections,
            "min_reserve_holds": self.min_reserve_holds,
            "min_dispute_bonds": self.min_dispute_bonds,
            "min_appeal_deadlines": self.min_appeal_deadlines,
            "roots_only_public_records": self.roots_only_public_records,
            "release_enabled": self.release_enabled,
            "release_claims_enabled": self.release_claims_enabled,
            "challenge_window_open": self.challenge_window_open,
            "custody_holds_active": self.custody_holds_active,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LedgerKind {
    Wave97ReleaseClaimSeal,
    WatcherChallenge,
    CustodyObjection,
    ReserveHold,
    DisputeBond,
    AppealDeadline,
}

impl LedgerKind {
    pub fn all() -> [Self; 6] {
        [
            Self::Wave97ReleaseClaimSeal,
            Self::WatcherChallenge,
            Self::CustodyObjection,
            Self::ReserveHold,
            Self::DisputeBond,
            Self::AppealDeadline,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Wave97ReleaseClaimSeal => "wave97_release_claim_seal",
            Self::WatcherChallenge => "watcher_challenge",
            Self::CustodyObjection => "custody_objection",
            Self::ReserveHold => "reserve_hold",
            Self::DisputeBond => "dispute_bond",
            Self::AppealDeadline => "appeal_deadline",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HoldoffBlockerKind {
    ChallengeWindowOpen,
    WatcherChallengeOpen,
    CustodyObjectionOpen,
    ReserveHoldActive,
    DisputeBondPending,
    AppealDeadlineOpen,
    ReleaseDisabled,
    ReleaseClaimsDisabled,
    HeavyGateMissing,
    RootsOnlyRequired,
}

impl HoldoffBlockerKind {
    pub fn all() -> [Self; 10] {
        [
            Self::ChallengeWindowOpen,
            Self::WatcherChallengeOpen,
            Self::CustodyObjectionOpen,
            Self::ReserveHoldActive,
            Self::DisputeBondPending,
            Self::AppealDeadlineOpen,
            Self::ReleaseDisabled,
            Self::ReleaseClaimsDisabled,
            Self::HeavyGateMissing,
            Self::RootsOnlyRequired,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChallengeWindowOpen => "challenge_window_open",
            Self::WatcherChallengeOpen => "watcher_challenge_open",
            Self::CustodyObjectionOpen => "custody_objection_open",
            Self::ReserveHoldActive => "reserve_hold_active",
            Self::DisputeBondPending => "dispute_bond_pending",
            Self::AppealDeadlineOpen => "appeal_deadline_open",
            Self::ReleaseDisabled => "release_disabled",
            Self::ReleaseClaimsDisabled => "release_claims_disabled",
            Self::HeavyGateMissing => "heavy_gate_missing",
            Self::RootsOnlyRequired => "roots_only_required",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HoldoffStatus {
    Blocked,
    Releasable,
}

impl HoldoffStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Blocked => "blocked",
            Self::Releasable => "releasable",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandHintKind {
    KeepReleaseDisabled,
    KeepChallengeWindowOpen,
    KeepCustodyHoldsActive,
    ReviewWatcherChallenges,
    ReviewCustodyObjections,
    ReviewReserveHolds,
    ReviewDisputeBonds,
    ReviewAppealDeadlines,
    RecomputeHoldoffLedger,
    PreserveRootsOnlyRecord,
}

impl CommandHintKind {
    pub fn all() -> [Self; 10] {
        [
            Self::KeepReleaseDisabled,
            Self::KeepChallengeWindowOpen,
            Self::KeepCustodyHoldsActive,
            Self::ReviewWatcherChallenges,
            Self::ReviewCustodyObjections,
            Self::ReviewReserveHolds,
            Self::ReviewDisputeBonds,
            Self::ReviewAppealDeadlines,
            Self::RecomputeHoldoffLedger,
            Self::PreserveRootsOnlyRecord,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::KeepReleaseDisabled => "keep_release_disabled",
            Self::KeepChallengeWindowOpen => "keep_challenge_window_open",
            Self::KeepCustodyHoldsActive => "keep_custody_holds_active",
            Self::ReviewWatcherChallenges => "review_watcher_challenges",
            Self::ReviewCustodyObjections => "review_custody_objections",
            Self::ReviewReserveHolds => "review_reserve_holds",
            Self::ReviewDisputeBonds => "review_dispute_bonds",
            Self::ReviewAppealDeadlines => "review_appeal_deadlines",
            Self::RecomputeHoldoffLedger => "recompute_holdoff_ledger",
            Self::PreserveRootsOnlyRecord => "preserve_roots_only_record",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LedgerEntry {
    pub kind: LedgerKind,
    pub source_root: String,
    pub holdoff_root: String,
    pub blocker_root: String,
    pub open: bool,
}

impl LedgerEntry {
    pub fn placeholder(kind: LedgerKind, config: &Config) -> Self {
        let source_root = match kind {
            LedgerKind::Wave97ReleaseClaimSeal => config.wave97_release_claim_seal_root.clone(),
            _ => placeholder_root("source", kind.as_str()),
        };
        let holdoff_root = record_root(
            "holdoff-entry",
            &json!({
                "kind": kind.as_str(),
                "source_root": source_root,
                "open": true,
                "release_allowed": false,
            }),
        );
        let blocker_root = record_root(
            "holdoff-entry-blocker",
            &json!({
                "kind": kind.as_str(),
                "holdoff_root": holdoff_root,
                "active": true,
            }),
        );
        Self {
            kind,
            source_root,
            holdoff_root,
            blocker_root,
            open: true,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": self.kind.as_str(),
            "source_root": self.source_root,
            "holdoff_root": self.holdoff_root,
            "blocker_root": self.blocker_root,
            "open": self.open,
        })
    }

    pub fn root(&self) -> String {
        record_root("ledger-entry", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HoldoffBlocker {
    pub kind: HoldoffBlockerKind,
    pub blocker_root: String,
    pub active: bool,
}

impl HoldoffBlocker {
    pub fn active(kind: HoldoffBlockerKind) -> Self {
        let blocker_root = record_root(
            "holdoff-blocker",
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
        record_root("holdoff-blocker-root", &self.public_record())
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
                "dry_run_only": true,
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
pub struct HoldoffSummary {
    pub status: HoldoffStatus,
    pub fail_closed: bool,
    pub release_allowed: bool,
    pub released_claim_count: u64,
    pub open_entry_count: u64,
    pub active_blocker_count: u64,
    pub wave97_release_claim_seal_root: String,
    pub watcher_challenge_root: String,
    pub custody_objection_root: String,
    pub reserve_hold_root: String,
    pub dispute_bond_root: String,
    pub appeal_deadline_root: String,
    pub holdoff_ledger_root: String,
    pub blocker_root: String,
    pub command_hint_root: String,
    pub deterministic_root: String,
    pub challenge_window_open: bool,
    pub custody_holds_active: bool,
    pub heavy_gates_ran: bool,
}

impl HoldoffSummary {
    pub fn from_state(
        entries: &BTreeMap<String, LedgerEntry>,
        blockers: &[HoldoffBlocker],
        command_hints: &[CommandHint],
        config: &Config,
    ) -> Self {
        let root_for = |kind: LedgerKind| -> String {
            let key = kind.as_str();
            match entries.get(key) {
                Some(entry) => entry.holdoff_root.clone(),
                None => placeholder_root("missing-ledger-entry", key),
            }
        };
        let watcher_challenge_root = root_for(LedgerKind::WatcherChallenge);
        let custody_objection_root = root_for(LedgerKind::CustodyObjection);
        let reserve_hold_root = root_for(LedgerKind::ReserveHold);
        let dispute_bond_root = root_for(LedgerKind::DisputeBond);
        let appeal_deadline_root = root_for(LedgerKind::AppealDeadline);
        let holdoff_ledger_root = roots_root(
            "holdoff-ledger-entries",
            entries.values().map(LedgerEntry::root).collect(),
        );
        let blocker_root = roots_root(
            "holdoff-blocker-roots",
            blockers.iter().map(HoldoffBlocker::root).collect(),
        );
        let command_hint_root = roots_root(
            "command-hint-roots",
            command_hints.iter().map(CommandHint::root).collect(),
        );
        let active_blocker_count = blockers.iter().filter(|blocker| blocker.active).count() as u64;
        let open_entry_count = entries.values().filter(|entry| entry.open).count() as u64;
        let release_allowed = false;
        let status = if release_allowed {
            HoldoffStatus::Releasable
        } else {
            HoldoffStatus::Blocked
        };
        let deterministic_record = json!({
            "wave97_release_claim_seal_root": config.wave97_release_claim_seal_root,
            "watcher_challenge_root": watcher_challenge_root,
            "custody_objection_root": custody_objection_root,
            "reserve_hold_root": reserve_hold_root,
            "dispute_bond_root": dispute_bond_root,
            "appeal_deadline_root": appeal_deadline_root,
            "holdoff_ledger_root": holdoff_ledger_root,
            "blocker_root": blocker_root,
            "command_hint_root": command_hint_root,
            "release_allowed": release_allowed,
        });
        Self {
            status,
            fail_closed: status == HoldoffStatus::Blocked,
            release_allowed,
            released_claim_count: 0,
            open_entry_count,
            active_blocker_count,
            wave97_release_claim_seal_root: config.wave97_release_claim_seal_root.clone(),
            watcher_challenge_root,
            custody_objection_root,
            reserve_hold_root,
            dispute_bond_root,
            appeal_deadline_root,
            holdoff_ledger_root,
            blocker_root,
            command_hint_root,
            deterministic_root: record_root("deterministic-holdoff-ledger", &deterministic_record),
            challenge_window_open: config.challenge_window_open,
            custody_holds_active: config.custody_holds_active,
            heavy_gates_ran: config.heavy_gates_ran,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "status": self.status.as_str(),
            "fail_closed": self.fail_closed,
            "release_allowed": self.release_allowed,
            "released_claim_count": self.released_claim_count,
            "open_entry_count": self.open_entry_count,
            "active_blocker_count": self.active_blocker_count,
            "wave97_release_claim_seal_root": self.wave97_release_claim_seal_root,
            "watcher_challenge_root": self.watcher_challenge_root,
            "custody_objection_root": self.custody_objection_root,
            "reserve_hold_root": self.reserve_hold_root,
            "dispute_bond_root": self.dispute_bond_root,
            "appeal_deadline_root": self.appeal_deadline_root,
            "holdoff_ledger_root": self.holdoff_ledger_root,
            "blocker_root": self.blocker_root,
            "command_hint_root": self.command_hint_root,
            "deterministic_root": self.deterministic_root,
            "challenge_window_open": self.challenge_window_open,
            "custody_holds_active": self.custody_holds_active,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub ledger_entries: BTreeMap<String, LedgerEntry>,
    pub blockers: Vec<HoldoffBlocker>,
    pub command_hints: Vec<CommandHint>,
    pub summary: HoldoffSummary,
}

impl Default for State {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

impl State {
    pub fn new(config: Config) -> Self {
        let ledger_entries = LedgerKind::all()
            .iter()
            .map(|kind| {
                let entry = LedgerEntry::placeholder(*kind, &config);
                (kind.as_str().to_string(), entry)
            })
            .collect::<BTreeMap<_, _>>();
        let blockers = HoldoffBlockerKind::all()
            .iter()
            .map(|kind| HoldoffBlocker::active(*kind))
            .collect::<Vec<_>>();
        let command_hints = CommandHint::canonical();
        let summary =
            HoldoffSummary::from_state(&ledger_entries, &blockers, &command_hints, &config);
        Self {
            config,
            ledger_entries,
            blockers,
            command_hints,
            summary,
        }
    }

    pub fn recompute(&mut self) {
        self.summary = HoldoffSummary::from_state(
            &self.ledger_entries,
            &self.blockers,
            &self.command_hints,
            &self.config,
        );
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        if self.summary.release_allowed {
            return Err("devnet release must remain denied".to_string());
        }
        if self.summary.released_claim_count != 0 {
            return Err("devnet must expose zero released claims".to_string());
        }
        if !self.summary.fail_closed {
            return Err("challenge-window holdoff ledger must fail closed".to_string());
        }
        if self.summary.open_entry_count < LedgerKind::all().len() as u64 {
            return Err("all devnet holdoff entries must remain open".to_string());
        }
        if self.summary.active_blocker_count < HoldoffBlockerKind::all().len() as u64 {
            return Err("all devnet holdoff blockers must remain active".to_string());
        }
        ensure_root_like(
            "wave97_release_claim_seal_root",
            &self.summary.wave97_release_claim_seal_root,
        )?;
        ensure_root_like(
            "watcher_challenge_root",
            &self.summary.watcher_challenge_root,
        )?;
        ensure_root_like(
            "custody_objection_root",
            &self.summary.custody_objection_root,
        )?;
        ensure_root_like("reserve_hold_root", &self.summary.reserve_hold_root)?;
        ensure_root_like("dispute_bond_root", &self.summary.dispute_bond_root)?;
        ensure_root_like("appeal_deadline_root", &self.summary.appeal_deadline_root)?;
        for entry in self.ledger_entries.values() {
            ensure_root_like("ledger_source_root", &entry.source_root)?;
            ensure_root_like("ledger_holdoff_root", &entry.holdoff_root)?;
            ensure_root_like("ledger_blocker_root", &entry.blocker_root)?;
            if !entry.open {
                return Err("devnet ledger entries must remain open".to_string());
            }
        }
        for blocker in &self.blockers {
            ensure_root_like("holdoff_blocker_root", &blocker.blocker_root)?;
            if !blocker.active {
                return Err("devnet holdoff blockers must remain active".to_string());
            }
        }
        Ok(())
    }

    pub fn public_record_without_state_root(&self) -> PublicRecord {
        json!({
            "kind": "wave98_bridge_custody_release_claim_challenge_window_holdoff_ledger",
            "config": self.config.public_record(),
            "wave96_release_readiness_root": self.config.wave96_release_readiness_root,
            "wave97_release_claim_seal_root": self.config.wave97_release_claim_seal_root,
            "ledger_entry_count": self.ledger_entries.len() as u64,
            "watcher_challenge_root": self.summary.watcher_challenge_root,
            "custody_objection_root": self.summary.custody_objection_root,
            "reserve_hold_root": self.summary.reserve_hold_root,
            "dispute_bond_root": self.summary.dispute_bond_root,
            "appeal_deadline_root": self.summary.appeal_deadline_root,
            "holdoff_ledger_root": self.summary.holdoff_ledger_root,
            "holdoff_blocker_root": self.summary.blocker_root,
            "command_hint_root": self.summary.command_hint_root,
            "deterministic_root": self.summary.deterministic_root,
            "summary": self.summary.public_record(),
            "roots_only_public_records": true,
            "release_allowed": false,
            "released_claim_count": 0,
            "challenge_window_open": true,
            "custody_holds_active": true,
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
    format!("root:wave98:{}", merkle_root(domain, &leaves))
}

fn record_root(domain: &str, record: &PublicRecord) -> String {
    let hash = domain_hash(
        "WAVE98-BRIDGE-CUSTODY-RELEASE-CLAIM-CHALLENGE-WINDOW-HOLDOFF-LEDGER",
        &[
            HashPart::Str(domain),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Json(record),
        ],
        32,
    );
    format!("root:wave98:{hash}")
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
