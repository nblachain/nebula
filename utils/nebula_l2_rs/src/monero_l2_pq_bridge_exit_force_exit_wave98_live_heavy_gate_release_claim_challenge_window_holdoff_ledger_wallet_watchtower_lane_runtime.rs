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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave98-live-heavy-gate-release-claim-challenge-window-holdoff-ledger-wallet-watchtower-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const WAVE_LABEL: &str = "wave98";
pub const SOURCE_WAVE_LABEL: &str = "wave97";
pub const SOURCE_LANE: &str =
    "force-exit-live-heavy-gate-release-claim-anti-equivocation-seal-wallet-watchtower-lane";
pub const HOLDOFF_LANE: &str =
    "force-exit-live-heavy-gate-release-claim-challenge-window-holdoff-ledger-wallet-watchtower-lane";
pub const EMPTY_ROOT_MARKER: &str = "empty-wave98-wallet-watchtower-holdoff-ledger-root";
pub const DEFAULT_HOLDOFF_EPOCH: u64 = 98;
pub const DEFAULT_MIN_WAVE97_SEAL_ROOTS: u64 = 1;
pub const DEFAULT_MIN_WATCHTOWER_OBJECTION_ROOTS: u64 = 2;
pub const DEFAULT_MIN_WALLET_RECOVERY_CHALLENGE_ROOTS: u64 = 1;
pub const DEFAULT_MIN_REVIEWER_HOLD_ROOTS: u64 = 2;
pub const DEFAULT_MIN_DISPUTE_BOND_ROOTS: u64 = 1;
pub const DEFAULT_MIN_APPEAL_DEADLINE_ROOTS: u64 = 1;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub wave_label: String,
    pub source_wave_label: String,
    pub source_lane: String,
    pub holdoff_lane: String,
    pub empty_root_marker: String,
    pub holdoff_epoch: u64,
    pub min_wave97_seal_roots: u64,
    pub min_watchtower_objection_roots: u64,
    pub min_wallet_recovery_challenge_roots: u64,
    pub min_reviewer_hold_roots: u64,
    pub min_dispute_bond_roots: u64,
    pub min_appeal_deadline_roots: u64,
    pub require_roots_only_public_record: bool,
    pub require_wave97_release_claim_seal_roots: bool,
    pub require_watchtower_objection_roots: bool,
    pub require_wallet_recovery_challenge_roots: bool,
    pub require_reviewer_hold_roots: bool,
    pub require_dispute_bond_roots: bool,
    pub require_appeal_deadline_roots: bool,
    pub fail_closed_on_open_challenge_window: bool,
    pub wallet_hold_active: bool,
    pub watchtower_hold_active: bool,
    pub challenge_window_release_enabled: bool,
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
            holdoff_lane: HOLDOFF_LANE.to_string(),
            empty_root_marker: EMPTY_ROOT_MARKER.to_string(),
            holdoff_epoch: DEFAULT_HOLDOFF_EPOCH,
            min_wave97_seal_roots: DEFAULT_MIN_WAVE97_SEAL_ROOTS,
            min_watchtower_objection_roots: DEFAULT_MIN_WATCHTOWER_OBJECTION_ROOTS,
            min_wallet_recovery_challenge_roots: DEFAULT_MIN_WALLET_RECOVERY_CHALLENGE_ROOTS,
            min_reviewer_hold_roots: DEFAULT_MIN_REVIEWER_HOLD_ROOTS,
            min_dispute_bond_roots: DEFAULT_MIN_DISPUTE_BOND_ROOTS,
            min_appeal_deadline_roots: DEFAULT_MIN_APPEAL_DEADLINE_ROOTS,
            require_roots_only_public_record: true,
            require_wave97_release_claim_seal_roots: true,
            require_watchtower_objection_roots: true,
            require_wallet_recovery_challenge_roots: true,
            require_reviewer_hold_roots: true,
            require_dispute_bond_roots: true,
            require_appeal_deadline_roots: true,
            fail_closed_on_open_challenge_window: true,
            wallet_hold_active: true,
            watchtower_hold_active: true,
            challenge_window_release_enabled: false,
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
            "holdoff_lane": self.holdoff_lane,
            "empty_root_marker": self.empty_root_marker,
            "holdoff_epoch": self.holdoff_epoch,
            "min_wave97_seal_roots": self.min_wave97_seal_roots,
            "min_watchtower_objection_roots": self.min_watchtower_objection_roots,
            "min_wallet_recovery_challenge_roots": self.min_wallet_recovery_challenge_roots,
            "min_reviewer_hold_roots": self.min_reviewer_hold_roots,
            "min_dispute_bond_roots": self.min_dispute_bond_roots,
            "min_appeal_deadline_roots": self.min_appeal_deadline_roots,
            "require_roots_only_public_record": self.require_roots_only_public_record,
            "require_wave97_release_claim_seal_roots": self.require_wave97_release_claim_seal_roots,
            "require_watchtower_objection_roots": self.require_watchtower_objection_roots,
            "require_wallet_recovery_challenge_roots": self.require_wallet_recovery_challenge_roots,
            "require_reviewer_hold_roots": self.require_reviewer_hold_roots,
            "require_dispute_bond_roots": self.require_dispute_bond_roots,
            "require_appeal_deadline_roots": self.require_appeal_deadline_roots,
            "fail_closed_on_open_challenge_window": self.fail_closed_on_open_challenge_window,
            "wallet_hold_active": self.wallet_hold_active,
            "watchtower_hold_active": self.watchtower_hold_active,
            "challenge_window_release_enabled": self.challenge_window_release_enabled,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HoldoffSlotKind {
    WalletReleaseClaimSeal,
    WatchtowerObjection,
    WalletRecoveryChallenge,
    ReviewerHold,
    DisputeBond,
    AppealDeadline,
    OperatorCommand,
}

impl HoldoffSlotKind {
    pub fn all() -> [Self; 7] {
        [
            Self::WalletReleaseClaimSeal,
            Self::WatchtowerObjection,
            Self::WalletRecoveryChallenge,
            Self::ReviewerHold,
            Self::DisputeBond,
            Self::AppealDeadline,
            Self::OperatorCommand,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::WalletReleaseClaimSeal => "wallet_release_claim_seal",
            Self::WatchtowerObjection => "watchtower_objection",
            Self::WalletRecoveryChallenge => "wallet_recovery_challenge",
            Self::ReviewerHold => "reviewer_hold",
            Self::DisputeBond => "dispute_bond",
            Self::AppealDeadline => "appeal_deadline",
            Self::OperatorCommand => "operator_command",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HoldoffStatus {
    OpenBlocked,
    EvidenceBlocked,
    Held,
    ReleaseReady,
    Denied,
}

impl HoldoffStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OpenBlocked => "open_blocked",
            Self::EvidenceBlocked => "evidence_blocked",
            Self::Held => "held",
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
pub enum HoldoffBlocker {
    OpenChallengeWindow,
    Wave97SealRootMissing,
    WatchtowerObjectionRootMissing,
    WalletRecoveryChallengeRootMissing,
    ReviewerHoldRootMissing,
    DisputeBondRootMissing,
    AppealDeadlineRootMissing,
    DuplicateSealRoot,
    RootShapeInvalid,
    RootsOnlyRecordMissing,
    WalletHoldActive,
    WatchtowerHoldActive,
    ReleaseDisabled,
    HeavyGatesNotRun,
}

impl HoldoffBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OpenChallengeWindow => "open_challenge_window",
            Self::Wave97SealRootMissing => "wave97_seal_root_missing",
            Self::WatchtowerObjectionRootMissing => "watchtower_objection_root_missing",
            Self::WalletRecoveryChallengeRootMissing => "wallet_recovery_challenge_root_missing",
            Self::ReviewerHoldRootMissing => "reviewer_hold_root_missing",
            Self::DisputeBondRootMissing => "dispute_bond_root_missing",
            Self::AppealDeadlineRootMissing => "appeal_deadline_root_missing",
            Self::DuplicateSealRoot => "duplicate_seal_root",
            Self::RootShapeInvalid => "root_shape_invalid",
            Self::RootsOnlyRecordMissing => "roots_only_record_missing",
            Self::WalletHoldActive => "wallet_hold_active",
            Self::WatchtowerHoldActive => "watchtower_hold_active",
            Self::ReleaseDisabled => "release_disabled",
            Self::HeavyGatesNotRun => "heavy_gates_not_run",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandHintKind {
    HoldChallengeWindow,
    ImportWave97SealRoot,
    ImportWatchtowerObjectionRoot,
    ImportWalletRecoveryChallengeRoot,
    ImportReviewerHoldRoot,
    ImportDisputeBondRoot,
    ImportAppealDeadlineRoot,
    ResolveDuplicateSealRoot,
    MaintainHoldoff,
    ReleaseAfterChallengeWindow,
}

impl CommandHintKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HoldChallengeWindow => "hold_challenge_window",
            Self::ImportWave97SealRoot => "import_wave97_seal_root",
            Self::ImportWatchtowerObjectionRoot => "import_watchtower_objection_root",
            Self::ImportWalletRecoveryChallengeRoot => "import_wallet_recovery_challenge_root",
            Self::ImportReviewerHoldRoot => "import_reviewer_hold_root",
            Self::ImportDisputeBondRoot => "import_dispute_bond_root",
            Self::ImportAppealDeadlineRoot => "import_appeal_deadline_root",
            Self::ResolveDuplicateSealRoot => "resolve_duplicate_seal_root",
            Self::MaintainHoldoff => "maintain_holdoff",
            Self::ReleaseAfterChallengeWindow => "release_after_challenge_window",
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
    pub fn new(
        slot_kind: HoldoffSlotKind,
        kind: CommandHintKind,
        blockers: &[HoldoffBlocker],
    ) -> Self {
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
pub struct HoldoffEntry {
    pub slot_kind: HoldoffSlotKind,
    pub wave97_release_claim_seal_roots: Vec<String>,
    pub watchtower_objection_roots: Vec<String>,
    pub wallet_recovery_challenge_roots: Vec<String>,
    pub reviewer_hold_roots: Vec<String>,
    pub dispute_bond_roots: Vec<String>,
    pub appeal_deadline_roots: Vec<String>,
    pub wave97_release_claim_seal_root: String,
    pub watchtower_objection_root: String,
    pub wallet_recovery_challenge_root: String,
    pub reviewer_hold_root: String,
    pub dispute_bond_root: String,
    pub appeal_deadline_root: String,
    pub challenge_window_root: String,
    pub holdoff_ledger_root: String,
    pub blockers: Vec<HoldoffBlocker>,
    pub status: HoldoffStatus,
    pub command_hint: CommandHint,
    pub release_allowed: bool,
}

impl HoldoffEntry {
    pub fn empty(slot_kind: HoldoffSlotKind, config: &Config) -> Self {
        Self::from_roots(
            slot_kind,
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
        slot_kind: HoldoffSlotKind,
        wave97_release_claim_seal_roots: Vec<String>,
        watchtower_objection_roots: Vec<String>,
        wallet_recovery_challenge_roots: Vec<String>,
        reviewer_hold_roots: Vec<String>,
        dispute_bond_roots: Vec<String>,
        appeal_deadline_roots: Vec<String>,
        config: &Config,
    ) -> Self {
        let mut entry = Self {
            slot_kind,
            wave97_release_claim_seal_roots,
            watchtower_objection_roots,
            wallet_recovery_challenge_roots,
            reviewer_hold_roots,
            dispute_bond_roots,
            appeal_deadline_roots,
            wave97_release_claim_seal_root: empty_root("wave97-release-claim-seal"),
            watchtower_objection_root: empty_root("watchtower-objection"),
            wallet_recovery_challenge_root: empty_root("wallet-recovery-challenge"),
            reviewer_hold_root: empty_root("reviewer-hold"),
            dispute_bond_root: empty_root("dispute-bond"),
            appeal_deadline_root: empty_root("appeal-deadline"),
            challenge_window_root: empty_root("challenge-window"),
            holdoff_ledger_root: empty_root("holdoff-ledger"),
            blockers: Vec::new(),
            status: HoldoffStatus::OpenBlocked,
            command_hint: CommandHint::new(slot_kind, CommandHintKind::HoldChallengeWindow, &[]),
            release_allowed: false,
        };
        entry.recompute(config);
        entry
    }

    pub fn recompute(&mut self, config: &Config) {
        self.wave97_release_claim_seal_root = aggregate_root(
            "wave97-release-claim-seal-roots",
            &self.wave97_release_claim_seal_roots,
        );
        self.watchtower_objection_root = aggregate_root(
            "watchtower-objection-roots",
            &self.watchtower_objection_roots,
        );
        self.wallet_recovery_challenge_root = aggregate_root(
            "wallet-recovery-challenge-roots",
            &self.wallet_recovery_challenge_roots,
        );
        self.reviewer_hold_root = aggregate_root("reviewer-hold-roots", &self.reviewer_hold_roots);
        self.dispute_bond_root = aggregate_root("dispute-bond-roots", &self.dispute_bond_roots);
        self.appeal_deadline_root =
            aggregate_root("appeal-deadline-roots", &self.appeal_deadline_roots);
        self.challenge_window_root = challenge_window_root(self);
        self.holdoff_ledger_root = holdoff_ledger_root(self);
        self.blockers = holdoff_blockers(self, config);
        self.status = if !config.challenge_window_release_enabled {
            HoldoffStatus::Denied
        } else if config.wallet_hold_active || config.watchtower_hold_active {
            HoldoffStatus::Held
        } else if all_evidence_sets_empty(self) {
            HoldoffStatus::OpenBlocked
        } else if self.blockers.is_empty() {
            HoldoffStatus::ReleaseReady
        } else {
            HoldoffStatus::EvidenceBlocked
        };
        self.release_allowed = self.status.can_release()
            && config.challenge_window_release_enabled
            && config.heavy_gates_ran
            && !config.wallet_hold_active
            && !config.watchtower_hold_active;
        self.command_hint =
            CommandHint::new(self.slot_kind, command_for_entry(self), &self.blockers);
    }

    pub fn blocker_root(&self) -> String {
        blockers_root("entry-blockers", &self.blockers)
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "slot_kind": self.slot_kind.as_str(),
            "wave97_release_claim_seal_root": self.wave97_release_claim_seal_root,
            "watchtower_objection_root": self.watchtower_objection_root,
            "wallet_recovery_challenge_root": self.wallet_recovery_challenge_root,
            "reviewer_hold_root": self.reviewer_hold_root,
            "dispute_bond_root": self.dispute_bond_root,
            "appeal_deadline_root": self.appeal_deadline_root,
            "challenge_window_root": self.challenge_window_root,
            "holdoff_ledger_root": self.holdoff_ledger_root,
            "blocker_root": self.blocker_root(),
            "status": self.status.as_str(),
            "command_hint": self.command_hint.public_record(),
            "release_allowed": self.release_allowed,
            "roots_only": true,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("holdoff-entry", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HoldoffSummary {
    pub fail_closed: bool,
    pub release_denied: bool,
    pub released_claims: u64,
    pub held_claims: u64,
    pub blocked_claims: u64,
    pub denied_claims: u64,
    pub wave97_release_claim_seal_root: String,
    pub watchtower_objection_root: String,
    pub wallet_recovery_challenge_root: String,
    pub reviewer_hold_root: String,
    pub dispute_bond_root: String,
    pub appeal_deadline_root: String,
    pub challenge_window_root: String,
    pub holdoff_ledger_root: String,
    pub blocker_root: String,
    pub command_root: String,
    pub wallet_hold_active: bool,
    pub watchtower_hold_active: bool,
    pub heavy_gates_ran: bool,
}

impl HoldoffSummary {
    pub fn from_entries(config: &Config, entries: &BTreeMap<String, HoldoffEntry>) -> Self {
        let released_claims = entries
            .values()
            .filter(|entry| entry.release_allowed)
            .count() as u64;
        let held_claims = entries
            .values()
            .filter(|entry| entry.status == HoldoffStatus::Held)
            .count() as u64;
        let blocked_claims = entries
            .values()
            .filter(|entry| {
                matches!(
                    entry.status,
                    HoldoffStatus::OpenBlocked | HoldoffStatus::EvidenceBlocked
                )
            })
            .count() as u64;
        let denied_claims = entries
            .values()
            .filter(|entry| entry.status == HoldoffStatus::Denied)
            .count() as u64;
        let wave97_release_claim_seal_root = entry_field_root(
            "summary-wave97-release-claim-seal-roots",
            entries
                .values()
                .map(|entry| entry.wave97_release_claim_seal_root.clone()),
        );
        let watchtower_objection_root = entry_field_root(
            "summary-watchtower-objection-roots",
            entries
                .values()
                .map(|entry| entry.watchtower_objection_root.clone()),
        );
        let wallet_recovery_challenge_root = entry_field_root(
            "summary-wallet-recovery-challenge-roots",
            entries
                .values()
                .map(|entry| entry.wallet_recovery_challenge_root.clone()),
        );
        let reviewer_hold_root = entry_field_root(
            "summary-reviewer-hold-roots",
            entries
                .values()
                .map(|entry| entry.reviewer_hold_root.clone()),
        );
        let dispute_bond_root = entry_field_root(
            "summary-dispute-bond-roots",
            entries
                .values()
                .map(|entry| entry.dispute_bond_root.clone()),
        );
        let appeal_deadline_root = entry_field_root(
            "summary-appeal-deadline-roots",
            entries
                .values()
                .map(|entry| entry.appeal_deadline_root.clone()),
        );
        let challenge_window_root = entry_field_root(
            "summary-challenge-window-roots",
            entries
                .values()
                .map(|entry| entry.challenge_window_root.clone()),
        );
        let holdoff_ledger_root = entry_field_root(
            "summary-holdoff-ledger-roots",
            entries
                .values()
                .map(|entry| entry.holdoff_ledger_root.clone()),
        );
        let blocker_root = entry_field_root(
            "summary-blockers",
            entries.values().map(HoldoffEntry::blocker_root),
        );
        let command_root = merkle_root(
            "WAVE98-WALLET-WATCHTOWER-COMMAND-HINTS",
            &entries
                .values()
                .map(|entry| entry.command_hint.public_record())
                .collect::<Vec<_>>(),
        );
        let fail_closed = released_claims == 0
            || held_claims > 0
            || blocked_claims > 0
            || denied_claims > 0
            || !config.challenge_window_release_enabled
            || config.wallet_hold_active
            || config.watchtower_hold_active
            || !config.heavy_gates_ran;
        Self {
            fail_closed,
            release_denied: fail_closed,
            released_claims,
            held_claims,
            blocked_claims,
            denied_claims,
            wave97_release_claim_seal_root,
            watchtower_objection_root,
            wallet_recovery_challenge_root,
            reviewer_hold_root,
            dispute_bond_root,
            appeal_deadline_root,
            challenge_window_root,
            holdoff_ledger_root,
            blocker_root,
            command_root,
            wallet_hold_active: config.wallet_hold_active,
            watchtower_hold_active: config.watchtower_hold_active,
            heavy_gates_ran: config.heavy_gates_ran,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "fail_closed": self.fail_closed,
            "release_denied": self.release_denied,
            "released_claims": self.released_claims,
            "held_claims": self.held_claims,
            "blocked_claims": self.blocked_claims,
            "denied_claims": self.denied_claims,
            "wave97_release_claim_seal_root": self.wave97_release_claim_seal_root,
            "watchtower_objection_root": self.watchtower_objection_root,
            "wallet_recovery_challenge_root": self.wallet_recovery_challenge_root,
            "reviewer_hold_root": self.reviewer_hold_root,
            "dispute_bond_root": self.dispute_bond_root,
            "appeal_deadline_root": self.appeal_deadline_root,
            "challenge_window_root": self.challenge_window_root,
            "holdoff_ledger_root": self.holdoff_ledger_root,
            "blocker_root": self.blocker_root,
            "command_root": self.command_root,
            "wallet_hold_active": self.wallet_hold_active,
            "watchtower_hold_active": self.watchtower_hold_active,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("holdoff-summary", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub holdoff_entries: BTreeMap<String, HoldoffEntry>,
    pub summary: HoldoffSummary,
}

impl Default for State {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

impl State {
    pub fn new(config: Config) -> Self {
        let holdoff_entries = HoldoffSlotKind::all()
            .iter()
            .map(|slot_kind| {
                let entry = HoldoffEntry::empty(*slot_kind, &config);
                (slot_kind.as_str().to_string(), entry)
            })
            .collect::<BTreeMap<_, _>>();
        let summary = HoldoffSummary::from_entries(&config, &holdoff_entries);
        Self {
            config,
            holdoff_entries,
            summary,
        }
    }

    pub fn stage_holdoff_entry(
        mut self,
        slot_kind: HoldoffSlotKind,
        wave97_release_claim_seal_roots: Vec<String>,
        watchtower_objection_roots: Vec<String>,
        wallet_recovery_challenge_roots: Vec<String>,
        reviewer_hold_roots: Vec<String>,
        dispute_bond_roots: Vec<String>,
        appeal_deadline_roots: Vec<String>,
    ) -> Result<Self> {
        let entry = HoldoffEntry::from_roots(
            slot_kind,
            wave97_release_claim_seal_roots,
            watchtower_objection_roots,
            wallet_recovery_challenge_roots,
            reviewer_hold_roots,
            dispute_bond_roots,
            appeal_deadline_roots,
            &self.config,
        );
        self.holdoff_entries
            .insert(slot_kind.as_str().to_string(), entry);
        self.recompute();
        Ok(self)
    }

    pub fn recompute(&mut self) {
        for entry in self.holdoff_entries.values_mut() {
            entry.recompute(&self.config);
        }
        self.summary = HoldoffSummary::from_entries(&self.config, &self.holdoff_entries);
    }

    pub fn holdoff_entry_roots(&self) -> BTreeMap<String, String> {
        self.holdoff_entries
            .iter()
            .map(|(key, entry)| (key.clone(), entry.state_root()))
            .collect::<BTreeMap<_, _>>()
    }

    pub fn holdoff_entries_root(&self) -> String {
        merkle_root(
            "WAVE98-WALLET-WATCHTOWER-HOLDOFF-ENTRY-ROOTS",
            &self
                .holdoff_entry_roots()
                .values()
                .cloned()
                .map(Value::String)
                .collect::<Vec<_>>(),
        )
    }

    pub fn public_record_without_state_root(&self) -> PublicRecord {
        json!({
            "config": self.config.public_record(),
            "holdoff_entry_roots": self.holdoff_entry_roots(),
            "holdoff_entries_root": self.holdoff_entries_root(),
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

pub fn wallet_watchtower_challenge_window_holdoff_ledger_runtime() -> Runtime {
    devnet()
}

fn holdoff_blockers(entry: &HoldoffEntry, config: &Config) -> Vec<HoldoffBlocker> {
    let mut blockers = Vec::new();
    if config.fail_closed_on_open_challenge_window {
        blockers.push(HoldoffBlocker::OpenChallengeWindow);
    }
    if config.require_wave97_release_claim_seal_roots
        && entry.wave97_release_claim_seal_roots.len() < config.min_wave97_seal_roots as usize
    {
        blockers.push(HoldoffBlocker::Wave97SealRootMissing);
    }
    if config.require_watchtower_objection_roots
        && entry.watchtower_objection_roots.len() < config.min_watchtower_objection_roots as usize
    {
        blockers.push(HoldoffBlocker::WatchtowerObjectionRootMissing);
    }
    if config.require_wallet_recovery_challenge_roots
        && entry.wallet_recovery_challenge_roots.len()
            < config.min_wallet_recovery_challenge_roots as usize
    {
        blockers.push(HoldoffBlocker::WalletRecoveryChallengeRootMissing);
    }
    if config.require_reviewer_hold_roots
        && entry.reviewer_hold_roots.len() < config.min_reviewer_hold_roots as usize
    {
        blockers.push(HoldoffBlocker::ReviewerHoldRootMissing);
    }
    if config.require_dispute_bond_roots
        && entry.dispute_bond_roots.len() < config.min_dispute_bond_roots as usize
    {
        blockers.push(HoldoffBlocker::DisputeBondRootMissing);
    }
    if config.require_appeal_deadline_roots
        && entry.appeal_deadline_roots.len() < config.min_appeal_deadline_roots as usize
    {
        blockers.push(HoldoffBlocker::AppealDeadlineRootMissing);
    }
    if has_duplicate(&entry.wave97_release_claim_seal_roots) {
        blockers.push(HoldoffBlocker::DuplicateSealRoot);
    }
    if !roots_shape_valid(entry) {
        blockers.push(HoldoffBlocker::RootShapeInvalid);
    }
    if config.require_roots_only_public_record && !roots_only_record_present(entry) {
        blockers.push(HoldoffBlocker::RootsOnlyRecordMissing);
    }
    if config.wallet_hold_active {
        blockers.push(HoldoffBlocker::WalletHoldActive);
    }
    if config.watchtower_hold_active {
        blockers.push(HoldoffBlocker::WatchtowerHoldActive);
    }
    if !config.challenge_window_release_enabled {
        blockers.push(HoldoffBlocker::ReleaseDisabled);
    }
    if !config.heavy_gates_ran {
        blockers.push(HoldoffBlocker::HeavyGatesNotRun);
    }
    blockers
}

fn command_for_entry(entry: &HoldoffEntry) -> CommandHintKind {
    if entry.blockers.is_empty() {
        return CommandHintKind::ReleaseAfterChallengeWindow;
    }
    match entry.blockers[0] {
        HoldoffBlocker::Wave97SealRootMissing => CommandHintKind::ImportWave97SealRoot,
        HoldoffBlocker::WatchtowerObjectionRootMissing => {
            CommandHintKind::ImportWatchtowerObjectionRoot
        }
        HoldoffBlocker::WalletRecoveryChallengeRootMissing => {
            CommandHintKind::ImportWalletRecoveryChallengeRoot
        }
        HoldoffBlocker::ReviewerHoldRootMissing => CommandHintKind::ImportReviewerHoldRoot,
        HoldoffBlocker::DisputeBondRootMissing => CommandHintKind::ImportDisputeBondRoot,
        HoldoffBlocker::AppealDeadlineRootMissing => CommandHintKind::ImportAppealDeadlineRoot,
        HoldoffBlocker::DuplicateSealRoot => CommandHintKind::ResolveDuplicateSealRoot,
        HoldoffBlocker::WalletHoldActive | HoldoffBlocker::WatchtowerHoldActive => {
            CommandHintKind::MaintainHoldoff
        }
        HoldoffBlocker::OpenChallengeWindow
        | HoldoffBlocker::RootShapeInvalid
        | HoldoffBlocker::RootsOnlyRecordMissing
        | HoldoffBlocker::ReleaseDisabled
        | HoldoffBlocker::HeavyGatesNotRun => CommandHintKind::HoldChallengeWindow,
    }
}

fn roots_only_record_present(entry: &HoldoffEntry) -> bool {
    is_root_like(&entry.wave97_release_claim_seal_root)
        && is_root_like(&entry.watchtower_objection_root)
        && is_root_like(&entry.wallet_recovery_challenge_root)
        && is_root_like(&entry.reviewer_hold_root)
        && is_root_like(&entry.dispute_bond_root)
        && is_root_like(&entry.appeal_deadline_root)
        && is_root_like(&entry.challenge_window_root)
        && is_root_like(&entry.holdoff_ledger_root)
}

fn roots_shape_valid(entry: &HoldoffEntry) -> bool {
    all_roots_like(&entry.wave97_release_claim_seal_roots)
        && all_roots_like(&entry.watchtower_objection_roots)
        && all_roots_like(&entry.wallet_recovery_challenge_roots)
        && all_roots_like(&entry.reviewer_hold_roots)
        && all_roots_like(&entry.dispute_bond_roots)
        && all_roots_like(&entry.appeal_deadline_roots)
}

fn all_roots_like(roots: &[String]) -> bool {
    roots.iter().all(|root| is_root_like(root))
}

fn all_evidence_sets_empty(entry: &HoldoffEntry) -> bool {
    entry.wave97_release_claim_seal_roots.is_empty()
        && entry.watchtower_objection_roots.is_empty()
        && entry.wallet_recovery_challenge_roots.is_empty()
        && entry.reviewer_hold_roots.is_empty()
        && entry.dispute_bond_roots.is_empty()
        && entry.appeal_deadline_roots.is_empty()
}

fn has_duplicate(roots: &[String]) -> bool {
    let mut seen = BTreeSet::new();
    roots.iter().any(|root| !seen.insert(root))
}

fn challenge_window_root(entry: &HoldoffEntry) -> String {
    record_root(
        "challenge-window-root",
        &json!({
            "slot_kind": entry.slot_kind.as_str(),
            "wave97_release_claim_seal_root": entry.wave97_release_claim_seal_root,
            "watchtower_objection_root": entry.watchtower_objection_root,
            "wallet_recovery_challenge_root": entry.wallet_recovery_challenge_root,
            "reviewer_hold_root": entry.reviewer_hold_root,
            "dispute_bond_root": entry.dispute_bond_root,
            "appeal_deadline_root": entry.appeal_deadline_root,
            "challenge_window_open": true,
            "roots_only": true,
        }),
    )
}

fn holdoff_ledger_root(entry: &HoldoffEntry) -> String {
    record_root(
        "holdoff-ledger-root",
        &json!({
            "slot_kind": entry.slot_kind.as_str(),
            "challenge_window_root": entry.challenge_window_root,
            "wave97_release_claim_seal_root": entry.wave97_release_claim_seal_root,
            "watchtower_objection_root": entry.watchtower_objection_root,
            "wallet_recovery_challenge_root": entry.wallet_recovery_challenge_root,
            "reviewer_hold_root": entry.reviewer_hold_root,
            "dispute_bond_root": entry.dispute_bond_root,
            "appeal_deadline_root": entry.appeal_deadline_root,
            "wallet_material_absent": true,
            "watchtower_identity_material_absent": true,
            "payload_material_absent": true,
            "route_material_absent": true,
            "roots_only": true,
        }),
    )
}

fn aggregate_root(domain: &str, roots: &[String]) -> String {
    if roots.is_empty() {
        return empty_root(domain);
    }
    merkle_root(
        "WAVE98-WALLET-WATCHTOWER-ROOT-AGGREGATE",
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

fn entry_field_root<I>(domain: &str, values: I) -> String
where
    I: IntoIterator<Item = String>,
{
    merkle_root(
        domain,
        &values.into_iter().map(Value::String).collect::<Vec<_>>(),
    )
}

fn blockers_root(domain: &str, blockers: &[HoldoffBlocker]) -> String {
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
        "empty-holdoff-ledger-root",
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
        "WAVE98-WALLET-WATCHTOWER-RELEASE-CLAIM-CHALLENGE-WINDOW-HOLDOFF-LEDGER",
        &[
            HashPart::Str(domain),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Json(record),
        ],
        32,
    )
}
