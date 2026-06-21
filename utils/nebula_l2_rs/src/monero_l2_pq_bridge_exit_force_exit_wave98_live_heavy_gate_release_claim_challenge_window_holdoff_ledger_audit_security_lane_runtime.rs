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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave98-live-heavy-gate-release-claim-challenge-window-holdoff-ledger-audit-security-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-root-only-records";
pub const LEDGER_SUITE: &str =
    "wave98-wave97-release-claim-seal-to-challenge-window-holdoff-ledger-v1";
pub const DEFAULT_WAVE: u64 = 98;
pub const DEFAULT_SOURCE_WAVE: u64 = 97;
pub const DEFAULT_HEIGHT: u64 = 4_282_298;
pub const DEFAULT_MIN_SLOT_COUNT: u64 = 6;
pub const DEFAULT_CHALLENGE_WINDOW_BLOCKS: u64 = 720;
pub const DEFAULT_APPEAL_WINDOW_BLOCKS: u64 = 360;
pub const DEFAULT_MAX_RAW_PAYLOAD_RECORDS: u64 = 0;
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
pub enum HoldoffStatus {
    ChallengeWindowOpen,
    AuditObjectionActive,
    ReviewerHoldActive,
    AppealDeadlineOpen,
    ReleaseBlocked,
}

impl HoldoffStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChallengeWindowOpen => "challenge_window_open",
            Self::AuditObjectionActive => "audit_objection_active",
            Self::ReviewerHoldActive => "reviewer_hold_active",
            Self::AppealDeadlineOpen => "appeal_deadline_open",
            Self::ReleaseBlocked => "release_blocked",
        }
    }

    pub fn blocks_release(self) -> bool {
        true
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HoldoffBlockerKind {
    ChallengeWindowOpen,
    Wave97SealRootMissing,
    AuditObjectionOpen,
    ReviewerHoldOpen,
    DisputeBondUnsettled,
    AppealDeadlineOpen,
    ReleaseBudgetZero,
    ProductionDenied,
    FailClosedDisarmed,
    RawPayloadPresent,
    HeavyGateRunClaimed,
}

impl HoldoffBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChallengeWindowOpen => "challenge_window_open",
            Self::Wave97SealRootMissing => "wave97_seal_root_missing",
            Self::AuditObjectionOpen => "audit_objection_open",
            Self::ReviewerHoldOpen => "reviewer_hold_open",
            Self::DisputeBondUnsettled => "dispute_bond_unsettled",
            Self::AppealDeadlineOpen => "appeal_deadline_open",
            Self::ReleaseBudgetZero => "release_budget_zero",
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
    KeepChallengeWindowOpen,
    ImportWave97SealRoot,
    AttachAuditObjectionRoot,
    AttachReviewerHoldRoot,
    AttachDisputeBondRoot,
    TrackAppealDeadlineRoot,
    KeepFailClosed,
}

impl CommandHintKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HoldRelease => "hold_release",
            Self::KeepChallengeWindowOpen => "keep_challenge_window_open",
            Self::ImportWave97SealRoot => "import_wave97_seal_root",
            Self::AttachAuditObjectionRoot => "attach_audit_objection_root",
            Self::AttachReviewerHoldRoot => "attach_reviewer_hold_root",
            Self::AttachDisputeBondRoot => "attach_dispute_bond_root",
            Self::TrackAppealDeadlineRoot => "track_appeal_deadline_root",
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
    pub ledger_suite: String,
    pub wave: u64,
    pub source_wave: u64,
    pub current_height: u64,
    pub min_slot_count: u64,
    pub challenge_window_blocks: u64,
    pub appeal_window_blocks: u64,
    pub max_raw_payload_records: u64,
    pub max_released_claims: u64,
    pub wave97_audit_security_seal_root: String,
    pub wave97_release_claim_root: String,
    pub wave97_anti_equivocation_guard_root: String,
    pub wave97_blocker_root: String,
    pub fail_closed_armed: bool,
    pub challenge_windows_open: bool,
    pub audit_holds_active: bool,
    pub reviewer_holds_active: bool,
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
            ledger_suite: LEDGER_SUITE.to_string(),
            wave: DEFAULT_WAVE,
            source_wave: DEFAULT_SOURCE_WAVE,
            current_height: DEFAULT_HEIGHT,
            min_slot_count: DEFAULT_MIN_SLOT_COUNT,
            challenge_window_blocks: DEFAULT_CHALLENGE_WINDOW_BLOCKS,
            appeal_window_blocks: DEFAULT_APPEAL_WINDOW_BLOCKS,
            max_raw_payload_records: DEFAULT_MAX_RAW_PAYLOAD_RECORDS,
            max_released_claims: DEFAULT_MAX_RELEASED_CLAIMS,
            wave97_audit_security_seal_root: deterministic_root("wave97-audit-security-seal-root"),
            wave97_release_claim_root: deterministic_root("wave97-release-claim-root"),
            wave97_anti_equivocation_guard_root: deterministic_root(
                "wave97-anti-equivocation-guard-root",
            ),
            wave97_blocker_root: deterministic_root("wave97-blocker-root"),
            fail_closed_armed: true,
            challenge_windows_open: true,
            audit_holds_active: true,
            reviewer_holds_active: true,
            release_allowed: false,
            production_allowed: false,
            heavy_gates_ran: false,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_non_empty("chain_id", &self.chain_id)?;
        ensure_non_empty("protocol_version", &self.protocol_version)?;
        ensure_non_empty("hash_suite", &self.hash_suite)?;
        ensure_non_empty("ledger_suite", &self.ledger_suite)?;
        ensure_positive("wave", self.wave)?;
        ensure_positive("source_wave", self.source_wave)?;
        ensure_positive("current_height", self.current_height)?;
        ensure_positive("min_slot_count", self.min_slot_count)?;
        ensure_positive("challenge_window_blocks", self.challenge_window_blocks)?;
        ensure_positive("appeal_window_blocks", self.appeal_window_blocks)?;
        ensure_root(
            "wave97_audit_security_seal_root",
            &self.wave97_audit_security_seal_root,
        )?;
        ensure_root("wave97_release_claim_root", &self.wave97_release_claim_root)?;
        ensure_root(
            "wave97_anti_equivocation_guard_root",
            &self.wave97_anti_equivocation_guard_root,
        )?;
        ensure_root("wave97_blocker_root", &self.wave97_blocker_root)?;
        if !self.fail_closed_armed {
            return Err("wave98 holdoff ledger fail closed is not armed".to_string());
        }
        if !self.challenge_windows_open {
            return Err("wave98 challenge windows must remain open in devnet".to_string());
        }
        if !self.audit_holds_active {
            return Err("wave98 audit holds must remain active in devnet".to_string());
        }
        if !self.reviewer_holds_active {
            return Err("wave98 reviewer holds must remain active in devnet".to_string());
        }
        if self.release_allowed {
            return Err("wave98 holdoff ledger denies release by default".to_string());
        }
        if self.production_allowed {
            return Err("wave98 holdoff ledger denies production by default".to_string());
        }
        if self.heavy_gates_ran {
            return Err("wave98 holdoff ledger cannot claim gate execution".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave98_challenge_window_holdoff_ledger_config",
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "ledger_suite": self.ledger_suite,
            "wave": self.wave,
            "source_wave": self.source_wave,
            "current_height": self.current_height,
            "min_slot_count": self.min_slot_count,
            "challenge_window_blocks": self.challenge_window_blocks,
            "appeal_window_blocks": self.appeal_window_blocks,
            "max_raw_payload_records": self.max_raw_payload_records,
            "max_released_claims": self.max_released_claims,
            "wave97_audit_security_seal_root": self.wave97_audit_security_seal_root,
            "wave97_release_claim_root": self.wave97_release_claim_root,
            "wave97_anti_equivocation_guard_root": self.wave97_anti_equivocation_guard_root,
            "wave97_blocker_root": self.wave97_blocker_root,
            "fail_closed_armed": self.fail_closed_armed,
            "challenge_windows_open": self.challenge_windows_open,
            "audit_holds_active": self.audit_holds_active,
            "reviewer_holds_active": self.reviewer_holds_active,
            "release_allowed": self.release_allowed,
            "production_allowed": self.production_allowed,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }

    pub fn state_root(&self) -> String {
        value_root("WAVE98-CONFIG", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HoldoffEntry {
    pub slot_kind: LaneSlotKind,
    pub source_seal_root: String,
    pub challenge_window_root: String,
    pub audit_objection_root: String,
    pub reviewer_hold_root: String,
    pub dispute_bond_root: String,
    pub appeal_deadline_root: String,
    pub holdoff_entry_root: String,
    pub status: HoldoffStatus,
}

impl HoldoffEntry {
    pub fn blocked(slot_kind: LaneSlotKind, config: &Config) -> Self {
        let slot = slot_kind.as_str();
        let source_seal_root = bind_root(
            "wave97-audit-security-seal",
            slot,
            &config.wave97_audit_security_seal_root,
        );
        let challenge_window_root = value_root(
            "WAVE98-CHALLENGE-WINDOW",
            &json!({
                "slot_kind": slot,
                "source_seal_root": source_seal_root,
                "window_open": true,
                "opened_at_height": config.current_height,
                "window_blocks": config.challenge_window_blocks,
            }),
        );
        let audit_objection_root = value_root(
            "WAVE98-AUDIT-OBJECTION",
            &json!({
                "slot_kind": slot,
                "source_release_claim_root": config.wave97_release_claim_root,
                "source_guard_root": config.wave97_anti_equivocation_guard_root,
                "objection_active": true,
            }),
        );
        let reviewer_hold_root = value_root(
            "WAVE98-REVIEWER-HOLD",
            &json!({
                "slot_kind": slot,
                "source_blocker_root": config.wave97_blocker_root,
                "hold_active": true,
            }),
        );
        let dispute_bond_root = value_root(
            "WAVE98-DISPUTE-BOND",
            &json!({
                "slot_kind": slot,
                "challenge_window_root": challenge_window_root,
                "bond_settled": false,
            }),
        );
        let appeal_deadline_root = value_root(
            "WAVE98-APPEAL-DEADLINE",
            &json!({
                "slot_kind": slot,
                "challenge_window_root": challenge_window_root,
                "appeal_open": true,
                "appeal_window_blocks": config.appeal_window_blocks,
            }),
        );
        let holdoff_entry_root = value_root(
            "WAVE98-HOLDOFF-ENTRY",
            &json!({
                "slot_kind": slot,
                "source_seal_root": source_seal_root,
                "challenge_window_root": challenge_window_root,
                "audit_objection_root": audit_objection_root,
                "reviewer_hold_root": reviewer_hold_root,
                "dispute_bond_root": dispute_bond_root,
                "appeal_deadline_root": appeal_deadline_root,
                "release_allowed": false,
            }),
        );
        Self {
            slot_kind,
            source_seal_root,
            challenge_window_root,
            audit_objection_root,
            reviewer_hold_root,
            dispute_bond_root,
            appeal_deadline_root,
            holdoff_entry_root,
            status: HoldoffStatus::ChallengeWindowOpen,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root("source_seal_root", &self.source_seal_root)?;
        ensure_root("challenge_window_root", &self.challenge_window_root)?;
        ensure_root("audit_objection_root", &self.audit_objection_root)?;
        ensure_root("reviewer_hold_root", &self.reviewer_hold_root)?;
        ensure_root("dispute_bond_root", &self.dispute_bond_root)?;
        ensure_root("appeal_deadline_root", &self.appeal_deadline_root)?;
        ensure_root("holdoff_entry_root", &self.holdoff_entry_root)?;
        if !self.status.blocks_release() {
            return Err("wave98 holdoff entry does not block release".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "slot_kind": self.slot_kind.as_str(),
            "source_seal_root": self.source_seal_root,
            "challenge_window_root": self.challenge_window_root,
            "audit_objection_root": self.audit_objection_root,
            "reviewer_hold_root": self.reviewer_hold_root,
            "dispute_bond_root": self.dispute_bond_root,
            "appeal_deadline_root": self.appeal_deadline_root,
            "holdoff_entry_root": self.holdoff_entry_root,
            "status": self.status.as_str(),
            "release_allowed": false,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HoldoffBlocker {
    pub kind: HoldoffBlockerKind,
    pub slot_kind: LaneSlotKind,
    pub evidence_root: String,
    pub blocker_root: String,
}

impl HoldoffBlocker {
    pub fn new(kind: HoldoffBlockerKind, slot_kind: LaneSlotKind, evidence_root: &str) -> Self {
        Self {
            kind,
            slot_kind,
            evidence_root: evidence_root.to_string(),
            blocker_root: value_root(
                "WAVE98-HOLDOFF-BLOCKER",
                &json!({
                    "kind": kind.as_str(),
                    "slot_kind": slot_kind.as_str(),
                    "evidence_root": evidence_root,
                    "active": true,
                }),
            ),
        }
    }

    pub fn canonical(entries: &[HoldoffEntry], config: &Config) -> Vec<Self> {
        let mut blockers = Vec::new();
        for entry in entries {
            blockers.push(Self::new(
                HoldoffBlockerKind::ChallengeWindowOpen,
                entry.slot_kind,
                &entry.challenge_window_root,
            ));
            blockers.push(Self::new(
                HoldoffBlockerKind::AuditObjectionOpen,
                entry.slot_kind,
                &entry.audit_objection_root,
            ));
            blockers.push(Self::new(
                HoldoffBlockerKind::ReviewerHoldOpen,
                entry.slot_kind,
                &entry.reviewer_hold_root,
            ));
            blockers.push(Self::new(
                HoldoffBlockerKind::DisputeBondUnsettled,
                entry.slot_kind,
                &entry.dispute_bond_root,
            ));
            blockers.push(Self::new(
                HoldoffBlockerKind::AppealDeadlineOpen,
                entry.slot_kind,
                &entry.appeal_deadline_root,
            ));
        }
        blockers.push(Self::new(
            HoldoffBlockerKind::ReleaseBudgetZero,
            LaneSlotKind::OperatorSignoff,
            &config.state_root(),
        ));
        blockers.push(Self::new(
            HoldoffBlockerKind::ProductionDenied,
            LaneSlotKind::OperatorSignoff,
            &config.wave97_blocker_root,
        ));
        blockers
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root("evidence_root", &self.evidence_root)?;
        ensure_root("blocker_root", &self.blocker_root)
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": self.kind.as_str(),
            "slot_kind": self.slot_kind.as_str(),
            "evidence_root": self.evidence_root,
            "blocker_root": self.blocker_root,
            "active": true,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandHint {
    pub kind: CommandHintKind,
    pub target_root: String,
    pub command_root: String,
}

impl CommandHint {
    pub fn new(kind: CommandHintKind, target_root: &str, config: &Config) -> Self {
        Self {
            kind,
            target_root: target_root.to_string(),
            command_root: value_root(
                "WAVE98-COMMAND-HINT",
                &json!({
                    "kind": kind.as_str(),
                    "target_root": target_root,
                    "wave": config.wave,
                    "source_wave": config.source_wave,
                    "fail_closed_armed": config.fail_closed_armed,
                    "release_allowed": false,
                }),
            ),
        }
    }

    pub fn canonical(config: &Config) -> Vec<Self> {
        vec![
            Self::new(
                CommandHintKind::HoldRelease,
                &config.wave97_blocker_root,
                config,
            ),
            Self::new(
                CommandHintKind::KeepChallengeWindowOpen,
                &config.wave97_audit_security_seal_root,
                config,
            ),
            Self::new(
                CommandHintKind::ImportWave97SealRoot,
                &config.wave97_audit_security_seal_root,
                config,
            ),
            Self::new(
                CommandHintKind::AttachAuditObjectionRoot,
                &config.wave97_release_claim_root,
                config,
            ),
            Self::new(
                CommandHintKind::AttachReviewerHoldRoot,
                &config.wave97_anti_equivocation_guard_root,
                config,
            ),
            Self::new(
                CommandHintKind::AttachDisputeBondRoot,
                &config.wave97_blocker_root,
                config,
            ),
            Self::new(
                CommandHintKind::TrackAppealDeadlineRoot,
                &config.wave97_audit_security_seal_root,
                config,
            ),
            Self::new(
                CommandHintKind::KeepFailClosed,
                &config.state_root(),
                config,
            ),
        ]
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root("target_root", &self.target_root)?;
        ensure_root("command_root", &self.command_root)
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": self.kind.as_str(),
            "target_root": self.target_root,
            "command_root": self.command_root,
        })
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub holdoff_entries: u64,
    pub challenge_window_roots: u64,
    pub audit_objection_roots: u64,
    pub reviewer_hold_roots: u64,
    pub dispute_bond_roots: u64,
    pub appeal_deadline_roots: u64,
    pub blocker_count: u64,
    pub command_hint_count: u64,
    pub released_claims: u64,
    pub raw_payload_records: u64,
}

impl Counters {
    pub fn public_record(&self) -> PublicRecord {
        json!({
            "holdoff_entries": self.holdoff_entries,
            "challenge_window_roots": self.challenge_window_roots,
            "audit_objection_roots": self.audit_objection_roots,
            "reviewer_hold_roots": self.reviewer_hold_roots,
            "dispute_bond_roots": self.dispute_bond_roots,
            "appeal_deadline_roots": self.appeal_deadline_roots,
            "blocker_count": self.blocker_count,
            "command_hint_count": self.command_hint_count,
            "released_claims": self.released_claims,
            "raw_payload_records": self.raw_payload_records,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub holdoff_entries: Vec<HoldoffEntry>,
    pub blockers: Vec<HoldoffBlocker>,
    pub command_hints: Vec<CommandHint>,
    pub counters: Counters,
}

impl State {
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;
        let holdoff_entries = LaneSlotKind::all()
            .into_iter()
            .map(|slot_kind| HoldoffEntry::blocked(slot_kind, &config))
            .collect::<Vec<_>>();
        let blockers = HoldoffBlocker::canonical(&holdoff_entries, &config);
        let command_hints = CommandHint::canonical(&config);
        let mut state = Self {
            config,
            holdoff_entries,
            blockers,
            command_hints,
            counters: Counters::default(),
        };
        state.counters = state.compute_counters();
        state.validate()?;
        Ok(state)
    }

    pub fn compute_counters(&self) -> Counters {
        let entry_count = self.holdoff_entries.len() as u64;
        Counters {
            holdoff_entries: entry_count,
            challenge_window_roots: entry_count,
            audit_objection_roots: entry_count,
            reviewer_hold_roots: entry_count,
            dispute_bond_roots: entry_count,
            appeal_deadline_roots: entry_count,
            blocker_count: self.blockers.len() as u64,
            command_hint_count: self.command_hints.len() as u64,
            released_claims: 0,
            raw_payload_records: 0,
        }
    }

    pub fn wave97_seal_root(&self) -> String {
        value_root(
            "WAVE98-SOURCE-WAVE97-SEAL-ROOT",
            &json!({
                "wave97_audit_security_seal_root": self.config.wave97_audit_security_seal_root,
                "wave97_release_claim_root": self.config.wave97_release_claim_root,
                "wave97_anti_equivocation_guard_root": self.config.wave97_anti_equivocation_guard_root,
                "wave97_blocker_root": self.config.wave97_blocker_root,
            }),
        )
    }

    pub fn challenge_window_root(&self) -> String {
        collection_root(
            "WAVE98-CHALLENGE-WINDOW-ROOTS",
            self.holdoff_entries
                .iter()
                .map(|entry| {
                    json!({
                        "slot_kind": entry.slot_kind.as_str(),
                        "challenge_window_root": entry.challenge_window_root,
                    })
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn audit_objection_root(&self) -> String {
        collection_root(
            "WAVE98-AUDIT-OBJECTION-ROOTS",
            self.holdoff_entries
                .iter()
                .map(|entry| {
                    json!({
                        "slot_kind": entry.slot_kind.as_str(),
                        "audit_objection_root": entry.audit_objection_root,
                    })
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn reviewer_hold_root(&self) -> String {
        collection_root(
            "WAVE98-REVIEWER-HOLD-ROOTS",
            self.holdoff_entries
                .iter()
                .map(|entry| {
                    json!({
                        "slot_kind": entry.slot_kind.as_str(),
                        "reviewer_hold_root": entry.reviewer_hold_root,
                    })
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn dispute_bond_root(&self) -> String {
        collection_root(
            "WAVE98-DISPUTE-BOND-ROOTS",
            self.holdoff_entries
                .iter()
                .map(|entry| {
                    json!({
                        "slot_kind": entry.slot_kind.as_str(),
                        "dispute_bond_root": entry.dispute_bond_root,
                    })
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn appeal_deadline_root(&self) -> String {
        collection_root(
            "WAVE98-APPEAL-DEADLINE-ROOTS",
            self.holdoff_entries
                .iter()
                .map(|entry| {
                    json!({
                        "slot_kind": entry.slot_kind.as_str(),
                        "appeal_deadline_root": entry.appeal_deadline_root,
                    })
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn holdoff_ledger_root(&self) -> String {
        collection_root(
            "WAVE98-HOLDOFF-LEDGER",
            self.holdoff_entries
                .iter()
                .map(HoldoffEntry::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn blocker_root(&self) -> String {
        collection_root(
            "WAVE98-HOLDOFF-BLOCKERS",
            self.blockers
                .iter()
                .map(HoldoffBlocker::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn command_hint_root(&self) -> String {
        collection_root(
            "WAVE98-COMMAND-HINTS",
            self.command_hints
                .iter()
                .map(CommandHint::public_record)
                .collect::<Vec<_>>(),
        )
    }

    pub fn state_material_root(&self) -> String {
        value_root(
            "WAVE98-STATE-MATERIAL",
            &json!({
                "config_root": self.config.state_root(),
                "wave97_seal_root": self.wave97_seal_root(),
                "challenge_window_root": self.challenge_window_root(),
                "audit_objection_root": self.audit_objection_root(),
                "reviewer_hold_root": self.reviewer_hold_root(),
                "dispute_bond_root": self.dispute_bond_root(),
                "appeal_deadline_root": self.appeal_deadline_root(),
                "holdoff_ledger_root": self.holdoff_ledger_root(),
                "blocker_root": self.blocker_root(),
                "command_hint_root": self.command_hint_root(),
                "counters": self.counters.public_record(),
                "release_allowed": false,
                "production_allowed": false,
                "heavy_gates_ran": false,
            }),
        )
    }

    pub fn state_root(&self) -> String {
        value_root(
            "WAVE98-STATE",
            &json!({
                "state_material_root": self.state_material_root(),
                "challenge_windows_open": self.config.challenge_windows_open,
                "audit_holds_active": self.config.audit_holds_active,
                "reviewer_holds_active": self.config.reviewer_holds_active,
                "released_claims": 0,
                "release_denied": self.release_denied(),
                "production_denied": self.production_denied(),
                "heavy_gates_ran": false,
            }),
        )
    }

    pub fn release_denied(&self) -> bool {
        !self.config.release_allowed
            || self.config.challenge_windows_open
            || self.config.audit_holds_active
            || self.config.reviewer_holds_active
            || !self.blockers.is_empty()
    }

    pub fn production_denied(&self) -> bool {
        !self.config.production_allowed || self.release_denied()
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        ensure_min_count(
            "slot count",
            self.holdoff_entries.len() as u64,
            self.config.min_slot_count,
        )?;
        for entry in &self.holdoff_entries {
            entry.validate()?;
        }
        for blocker in &self.blockers {
            blocker.validate()?;
        }
        for command in &self.command_hints {
            command.validate()?;
        }
        if self.counters.raw_payload_records > self.config.max_raw_payload_records {
            return Err("wave98 holdoff ledger contains raw payload records".to_string());
        }
        if self.counters.released_claims > self.config.max_released_claims {
            return Err("wave98 released claims above configured limit".to_string());
        }
        if self.counters.released_claims != 0 {
            return Err("devnet wave98 holdoff ledger must not release claims".to_string());
        }
        if self.compute_counters() != self.counters {
            return Err("wave98 holdoff counters do not match state".to_string());
        }
        if !self.release_denied() {
            return Err("wave98 holdoff ledger cannot allow release".to_string());
        }
        if !self.production_denied() {
            return Err("wave98 holdoff ledger cannot allow production".to_string());
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": "wave98_live_heavy_gate_release_claim_challenge_window_holdoff_ledger_audit_security_lane_state",
            "config": self.config.public_record(),
            "wave97_seal_root": self.wave97_seal_root(),
            "challenge_window_root": self.challenge_window_root(),
            "audit_objection_root": self.audit_objection_root(),
            "reviewer_hold_root": self.reviewer_hold_root(),
            "dispute_bond_root": self.dispute_bond_root(),
            "appeal_deadline_root": self.appeal_deadline_root(),
            "holdoff_ledger_root": self.holdoff_ledger_root(),
            "blocker_root": self.blocker_root(),
            "command_hint_root": self.command_hint_root(),
            "state_root": self.state_root(),
            "counters": self.counters.public_record(),
            "release_denied": self.release_denied(),
            "production_denied": self.production_denied(),
            "heavy_gates_ran": false,
            "holdoff_entries": self.holdoff_entries.iter().map(HoldoffEntry::public_record).collect::<Vec<_>>(),
            "blockers": self.blockers.iter().map(HoldoffBlocker::public_record).collect::<Vec<_>>(),
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
    let holdoff_entries = LaneSlotKind::all()
        .into_iter()
        .map(|slot_kind| HoldoffEntry::blocked(slot_kind, &config))
        .collect::<Vec<_>>();
    let mut state = State {
        blockers: vec![HoldoffBlocker::new(
            HoldoffBlockerKind::ProductionDenied,
            LaneSlotKind::OperatorSignoff,
            &value_root(
                "WAVE98-FALLBACK-ERROR",
                &json!({"error_root": stable_id("fallback-error", &error)}),
            ),
        )],
        command_hints: CommandHint::canonical(&config),
        counters: Counters::default(),
        holdoff_entries,
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
        "MONERO-L2-PQ-FORCE-EXIT-WAVE98-STABLE-ID",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(LEDGER_SUITE),
            HashPart::Str(domain),
            HashPart::Str(label),
        ],
        32,
    )
}

fn deterministic_root(label: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WAVE98-DETERMINISTIC-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(LEDGER_SUITE),
            HashPart::Str(label),
        ],
        32,
    )
}

fn bind_root(domain: &str, label: &str, source_root: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-FORCE-EXIT-WAVE98-BIND-ROOT",
        &[
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(LEDGER_SUITE),
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
