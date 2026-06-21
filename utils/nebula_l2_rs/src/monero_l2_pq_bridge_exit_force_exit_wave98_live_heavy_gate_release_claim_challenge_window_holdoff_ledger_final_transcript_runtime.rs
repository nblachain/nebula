use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::hash::{domain_hash, merkle_root, HashPart};

const CHAIN_ID: &str = "nebula-monero-private-l2-devnet";
const PROTOCOL_VERSION: &str =
    "wave98-live-heavy-gate-release-claim-challenge-window-holdoff-ledger-v1";
const WAVE: u64 = 98;
const SEAL_WAVE: u64 = 97;
const MIN_HOLDOFF_HEIGHT: u64 = 980_000;
const MIN_CHALLENGE_WINDOW: u64 = 720;
const MIN_APPEAL_WINDOW: u64 = 1_440;

pub type PublicRecord = Value;
pub type Runtime = State;
pub type Result<T> = core::result::Result<T, HoldoffError>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum HoldoffError {
    LaneMissing,
    ClaimMissing,
    ReleaseClaimSealRootMissing,
    ChallengeWindowRootMissing,
    ObjectionRootMissing,
    ReviewerHoldRootMissing,
    DisputeBondRootMissing,
    AppealDeadlineRootMissing,
    HoldoffHeightTooLow,
    ChallengeWindowTooShort,
    AppealWindowTooShort,
    DuplicateChallengeRoot,
    ForkedObjectionRoot,
    AppealStillOpen,
    HoldoffStillBlocked,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum LaneKind {
    Compile,
    RuntimeReplay,
    AuditSecurity,
    BridgeCustody,
    WalletWatchtower,
    PqReservePrivacy,
}

impl LaneKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Compile => "compile",
            Self::RuntimeReplay => "runtime_replay",
            Self::AuditSecurity => "audit_security",
            Self::BridgeCustody => "bridge_custody",
            Self::WalletWatchtower => "wallet_watchtower",
            Self::PqReservePrivacy => "pq_reserve_privacy",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::Compile => "Compile challenge-window holdoff ledger",
            Self::RuntimeReplay => "Runtime replay challenge-window holdoff ledger",
            Self::AuditSecurity => "Audit security challenge-window holdoff ledger",
            Self::BridgeCustody => "Bridge custody challenge-window holdoff ledger",
            Self::WalletWatchtower => "Wallet watchtower challenge-window holdoff ledger",
            Self::PqReservePrivacy => "PQ reserve privacy challenge-window holdoff ledger",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum HoldoffStatus {
    Empty,
    Blocked,
    WindowOpen,
    ChallengeReady,
    HoldoffCleared,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum HoldoffBlockerKind {
    MissingReleaseClaimSealRoot,
    MissingChallengeWindowRoot,
    MissingObjectionRoot,
    MissingReviewerHoldRoot,
    MissingDisputeBondRoot,
    MissingAppealDeadlineRoot,
    HoldoffHeightTooLow,
    ChallengeWindowOpen,
    ChallengeWindowTooShort,
    AppealWindowOpen,
    AppealWindowTooShort,
    DuplicateChallengeRoot,
    ForkedObjectionRoot,
    LiveHeavyGateEvidenceMissing,
}

impl HoldoffBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingReleaseClaimSealRoot => "missing_release_claim_seal_root",
            Self::MissingChallengeWindowRoot => "missing_challenge_window_root",
            Self::MissingObjectionRoot => "missing_objection_root",
            Self::MissingReviewerHoldRoot => "missing_reviewer_hold_root",
            Self::MissingDisputeBondRoot => "missing_dispute_bond_root",
            Self::MissingAppealDeadlineRoot => "missing_appeal_deadline_root",
            Self::HoldoffHeightTooLow => "holdoff_height_too_low",
            Self::ChallengeWindowOpen => "challenge_window_open",
            Self::ChallengeWindowTooShort => "challenge_window_too_short",
            Self::AppealWindowOpen => "appeal_window_open",
            Self::AppealWindowTooShort => "appeal_window_too_short",
            Self::DuplicateChallengeRoot => "duplicate_challenge_root",
            Self::ForkedObjectionRoot => "forked_objection_root",
            Self::LiveHeavyGateEvidenceMissing => "live_heavy_gate_evidence_missing",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub wave: u64,
    pub seal_wave: u64,
    pub min_holdoff_height: u64,
    pub min_challenge_window: u64,
    pub min_appeal_window: u64,
    pub lane_holdoff_threshold: u64,
    pub global_holdoff_threshold: u64,
    pub require_release_claim_seal_root: bool,
    pub require_challenge_window_root: bool,
    pub require_objection_root: bool,
    pub require_reviewer_hold_root: bool,
    pub require_dispute_bond_root: bool,
    pub require_appeal_deadline_root: bool,
    pub require_live_heavy_gate_evidence: bool,
    pub deny_release_when_any_lane_blocked: bool,
    pub roots_only_public_records: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            wave: WAVE,
            seal_wave: SEAL_WAVE,
            min_holdoff_height: MIN_HOLDOFF_HEIGHT,
            min_challenge_window: MIN_CHALLENGE_WINDOW,
            min_appeal_window: MIN_APPEAL_WINDOW,
            lane_holdoff_threshold: 1,
            global_holdoff_threshold: 6,
            require_release_claim_seal_root: true,
            require_challenge_window_root: true,
            require_objection_root: true,
            require_reviewer_hold_root: true,
            require_dispute_bond_root: true,
            require_appeal_deadline_root: true,
            require_live_heavy_gate_evidence: true,
            deny_release_when_any_lane_blocked: true,
            roots_only_public_records: true,
        }
    }
}

impl Config {
    pub fn public_record(&self) -> PublicRecord {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "wave": self.wave,
            "seal_wave": self.seal_wave,
            "min_holdoff_height": self.min_holdoff_height,
            "min_challenge_window": self.min_challenge_window,
            "min_appeal_window": self.min_appeal_window,
            "lane_holdoff_threshold": self.lane_holdoff_threshold,
            "global_holdoff_threshold": self.global_holdoff_threshold,
            "require_release_claim_seal_root": self.require_release_claim_seal_root,
            "require_challenge_window_root": self.require_challenge_window_root,
            "require_objection_root": self.require_objection_root,
            "require_reviewer_hold_root": self.require_reviewer_hold_root,
            "require_dispute_bond_root": self.require_dispute_bond_root,
            "require_appeal_deadline_root": self.require_appeal_deadline_root,
            "require_live_heavy_gate_evidence": self.require_live_heavy_gate_evidence,
            "deny_release_when_any_lane_blocked": self.deny_release_when_any_lane_blocked,
            "roots_only_public_records": self.roots_only_public_records,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReleaseClaimHoldoff {
    pub lane: LaneKind,
    pub slot_label: String,
    pub ordinal: u64,
    pub wave97_anti_equivocation_seal_root: String,
    pub release_claim_seal_root: Option<String>,
    pub challenge_window_root: Option<String>,
    pub objection_root: Option<String>,
    pub reviewer_hold_root: Option<String>,
    pub dispute_bond_root: Option<String>,
    pub appeal_deadline_root: Option<String>,
    pub holdoff_height: Option<u64>,
    pub challenge_window: Option<u64>,
    pub appeal_window: Option<u64>,
    pub challenge_guard_root: String,
    pub objection_guard_root: String,
    pub appeal_guard_root: String,
    pub command_hint_root: String,
    pub blockers: Vec<HoldoffBlockerKind>,
    pub status: HoldoffStatus,
}

impl ReleaseClaimHoldoff {
    pub fn empty(lane: LaneKind, slot_label: &str, ordinal: u64, config: &Config) -> Self {
        Self {
            lane,
            slot_label: slot_label.to_string(),
            ordinal,
            wave97_anti_equivocation_seal_root: label_root(
                "wave97_anti_equivocation_seal",
                lane.as_str(),
                slot_label,
                ordinal,
            ),
            release_claim_seal_root: None,
            challenge_window_root: None,
            objection_root: None,
            reviewer_hold_root: None,
            dispute_bond_root: None,
            appeal_deadline_root: None,
            holdoff_height: None,
            challenge_window: None,
            appeal_window: None,
            challenge_guard_root: label_root("challenge_guard", lane.as_str(), slot_label, ordinal),
            objection_guard_root: label_root("objection_guard", lane.as_str(), slot_label, ordinal),
            appeal_guard_root: label_root("appeal_guard", lane.as_str(), slot_label, ordinal),
            command_hint_root: label_root(
                "release_claim_challenge_holdoff_command",
                lane.as_str(),
                slot_label,
                ordinal,
            ),
            blockers: initial_blockers(config),
            status: HoldoffStatus::Blocked,
        }
    }

    pub fn stage_holdoff(
        &self,
        release_claim_seal_root: &str,
        challenge_window_root: &str,
        objection_root: &str,
        reviewer_hold_root: &str,
        dispute_bond_root: &str,
        appeal_deadline_root: &str,
        holdoff_height: u64,
        challenge_window: u64,
        appeal_window: u64,
        config: &Config,
        duplicate_challenge: bool,
        forked_objection: bool,
    ) -> Result<Self> {
        if release_claim_seal_root.is_empty() {
            return Err(HoldoffError::ReleaseClaimSealRootMissing);
        }
        if challenge_window_root.is_empty() {
            return Err(HoldoffError::ChallengeWindowRootMissing);
        }
        if objection_root.is_empty() {
            return Err(HoldoffError::ObjectionRootMissing);
        }
        if reviewer_hold_root.is_empty() {
            return Err(HoldoffError::ReviewerHoldRootMissing);
        }
        if dispute_bond_root.is_empty() {
            return Err(HoldoffError::DisputeBondRootMissing);
        }
        if appeal_deadline_root.is_empty() {
            return Err(HoldoffError::AppealDeadlineRootMissing);
        }
        if holdoff_height < config.min_holdoff_height {
            return Err(HoldoffError::HoldoffHeightTooLow);
        }
        if challenge_window < config.min_challenge_window {
            return Err(HoldoffError::ChallengeWindowTooShort);
        }
        if appeal_window < config.min_appeal_window {
            return Err(HoldoffError::AppealWindowTooShort);
        }
        if duplicate_challenge {
            return Err(HoldoffError::DuplicateChallengeRoot);
        }
        if forked_objection {
            return Err(HoldoffError::ForkedObjectionRoot);
        }

        let mut next = self.clone();
        next.release_claim_seal_root = Some(release_claim_seal_root.to_string());
        next.challenge_window_root = Some(challenge_window_root.to_string());
        next.objection_root = Some(objection_root.to_string());
        next.reviewer_hold_root = Some(reviewer_hold_root.to_string());
        next.dispute_bond_root = Some(dispute_bond_root.to_string());
        next.appeal_deadline_root = Some(appeal_deadline_root.to_string());
        next.holdoff_height = Some(holdoff_height);
        next.challenge_window = Some(challenge_window);
        next.appeal_window = Some(appeal_window);
        next.challenge_guard_root = holdoff_guard_root(
            "challenge",
            self.lane,
            &self.slot_label,
            release_claim_seal_root,
            challenge_window_root,
            objection_root,
            holdoff_height,
        );
        next.objection_guard_root = holdoff_guard_root(
            "objection",
            self.lane,
            &self.slot_label,
            release_claim_seal_root,
            challenge_window_root,
            objection_root,
            holdoff_height,
        );
        next.appeal_guard_root = holdoff_guard_root(
            "appeal",
            self.lane,
            &self.slot_label,
            release_claim_seal_root,
            appeal_deadline_root,
            reviewer_hold_root,
            holdoff_height,
        );
        next.blockers = next.active_blockers(config, duplicate_challenge, forked_objection, true);
        next.status = if next
            .blockers
            .iter()
            .any(|blocker| *blocker == HoldoffBlockerKind::ChallengeWindowOpen)
        {
            HoldoffStatus::WindowOpen
        } else if next.blockers.is_empty() {
            HoldoffStatus::ChallengeReady
        } else {
            HoldoffStatus::Blocked
        };
        Ok(next)
    }

    pub fn clear_holdoff(&self) -> Result<Self> {
        if !self.blockers.is_empty() {
            return Err(HoldoffError::HoldoffStillBlocked);
        }
        let mut next = self.clone();
        next.status = HoldoffStatus::HoldoffCleared;
        Ok(next)
    }

    fn active_blockers(
        &self,
        config: &Config,
        duplicate_challenge: bool,
        forked_objection: bool,
        challenge_open: bool,
    ) -> Vec<HoldoffBlockerKind> {
        let mut blockers = Vec::new();
        if config.require_release_claim_seal_root && self.release_claim_seal_root.is_none() {
            blockers.push(HoldoffBlockerKind::MissingReleaseClaimSealRoot);
        }
        if config.require_challenge_window_root && self.challenge_window_root.is_none() {
            blockers.push(HoldoffBlockerKind::MissingChallengeWindowRoot);
        }
        if config.require_objection_root && self.objection_root.is_none() {
            blockers.push(HoldoffBlockerKind::MissingObjectionRoot);
        }
        if config.require_reviewer_hold_root && self.reviewer_hold_root.is_none() {
            blockers.push(HoldoffBlockerKind::MissingReviewerHoldRoot);
        }
        if config.require_dispute_bond_root && self.dispute_bond_root.is_none() {
            blockers.push(HoldoffBlockerKind::MissingDisputeBondRoot);
        }
        if config.require_appeal_deadline_root && self.appeal_deadline_root.is_none() {
            blockers.push(HoldoffBlockerKind::MissingAppealDeadlineRoot);
        }
        match self.holdoff_height {
            Some(height) if height >= config.min_holdoff_height => {}
            _ => blockers.push(HoldoffBlockerKind::HoldoffHeightTooLow),
        }
        match self.challenge_window {
            Some(window) if window >= config.min_challenge_window => {}
            _ => blockers.push(HoldoffBlockerKind::ChallengeWindowTooShort),
        }
        match self.appeal_window {
            Some(window) if window >= config.min_appeal_window => {}
            _ => blockers.push(HoldoffBlockerKind::AppealWindowTooShort),
        }
        if challenge_open {
            blockers.push(HoldoffBlockerKind::ChallengeWindowOpen);
        }
        blockers.push(HoldoffBlockerKind::AppealWindowOpen);
        if duplicate_challenge {
            blockers.push(HoldoffBlockerKind::DuplicateChallengeRoot);
        }
        if forked_objection {
            blockers.push(HoldoffBlockerKind::ForkedObjectionRoot);
        }
        if config.require_live_heavy_gate_evidence {
            blockers.push(HoldoffBlockerKind::LiveHeavyGateEvidenceMissing);
        }
        blockers
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "slot_label": self.slot_label,
            "ordinal": self.ordinal,
            "wave97_anti_equivocation_seal_root": self.wave97_anti_equivocation_seal_root,
            "release_claim_seal_root": self.release_claim_seal_root,
            "challenge_window_root": self.challenge_window_root,
            "objection_root": self.objection_root,
            "reviewer_hold_root": self.reviewer_hold_root,
            "dispute_bond_root": self.dispute_bond_root,
            "appeal_deadline_root": self.appeal_deadline_root,
            "holdoff_height": self.holdoff_height,
            "challenge_window": self.challenge_window,
            "appeal_window": self.appeal_window,
            "challenge_guard_root": self.challenge_guard_root,
            "objection_guard_root": self.objection_guard_root,
            "appeal_guard_root": self.appeal_guard_root,
            "command_hint_root": self.command_hint_root,
            "blockers": self.blockers.iter().map(|blocker| blocker.as_str()).collect::<Vec<_>>(),
            "status": format!("{:?}", self.status),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("release_claim_holdoff", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LaneChallengeHoldoffLedger {
    pub lane: LaneKind,
    pub lane_title_root: String,
    pub wave97_anti_equivocation_transcript_root: String,
    pub holdoffs: Vec<ReleaseClaimHoldoff>,
    pub blocked_root: String,
    pub window_open_root: String,
    pub challenge_ready_root: String,
    pub holdoff_cleared_root: String,
    pub challenge_guard_root: String,
    pub objection_guard_root: String,
    pub appeal_guard_root: String,
    pub command_root: String,
    pub lane_status: HoldoffStatus,
}

impl LaneChallengeHoldoffLedger {
    pub fn new(lane: LaneKind, slot_labels: &[&str], config: &Config) -> Self {
        let holdoffs = slot_labels
            .iter()
            .enumerate()
            .map(|(index, label)| ReleaseClaimHoldoff::empty(lane, label, index as u64, config))
            .collect::<Vec<_>>();
        Self::from_holdoffs(
            lane,
            label_root("lane_title", lane.as_str(), lane.title(), WAVE),
            label_root(
                "wave97_anti_equivocation_transcript",
                lane.as_str(),
                "source",
                WAVE,
            ),
            holdoffs,
        )
    }

    pub fn stage_holdoff(
        &self,
        slot_label: &str,
        release_claim_seal_root: &str,
        challenge_window_root: &str,
        objection_root: &str,
        reviewer_hold_root: &str,
        dispute_bond_root: &str,
        appeal_deadline_root: &str,
        holdoff_height: u64,
        challenge_window: u64,
        appeal_window: u64,
        config: &Config,
        known_challenge_roots: &[String],
        known_objection_roots: &[String],
    ) -> Result<Self> {
        let duplicate_challenge = known_challenge_roots
            .iter()
            .any(|root| root.as_str() == challenge_window_root);
        let forked_objection = known_objection_roots
            .iter()
            .any(|root| root.as_str() == objection_root);
        let mut found = false;
        let mut holdoffs = Vec::with_capacity(self.holdoffs.len());
        for holdoff in &self.holdoffs {
            if holdoff.slot_label == slot_label {
                holdoffs.push(holdoff.stage_holdoff(
                    release_claim_seal_root,
                    challenge_window_root,
                    objection_root,
                    reviewer_hold_root,
                    dispute_bond_root,
                    appeal_deadline_root,
                    holdoff_height,
                    challenge_window,
                    appeal_window,
                    config,
                    duplicate_challenge,
                    forked_objection,
                )?);
                found = true;
            } else {
                holdoffs.push(holdoff.clone());
            }
        }
        if !found {
            return Err(HoldoffError::ClaimMissing);
        }
        Ok(Self::from_holdoffs(
            self.lane,
            self.lane_title_root.clone(),
            self.wave97_anti_equivocation_transcript_root.clone(),
            holdoffs,
        ))
    }

    fn from_holdoffs(
        lane: LaneKind,
        lane_title_root: String,
        wave97_anti_equivocation_transcript_root: String,
        holdoffs: Vec<ReleaseClaimHoldoff>,
    ) -> Self {
        let blocked_root = blocked_root(&holdoffs);
        let window_open_root = root_from_strings(
            "wave98_window_open_holdoffs",
            holdoffs.iter().filter_map(|holdoff| {
                if holdoff.status == HoldoffStatus::WindowOpen {
                    Some(holdoff.state_root())
                } else {
                    None
                }
            }),
        );
        let challenge_ready_root = root_from_strings(
            "wave98_challenge_ready_holdoffs",
            holdoffs.iter().filter_map(|holdoff| {
                if holdoff.status == HoldoffStatus::ChallengeReady {
                    Some(holdoff.state_root())
                } else {
                    None
                }
            }),
        );
        let holdoff_cleared_root = root_from_strings(
            "wave98_holdoff_cleared_claims",
            holdoffs.iter().filter_map(|holdoff| {
                if holdoff.status == HoldoffStatus::HoldoffCleared {
                    Some(holdoff.state_root())
                } else {
                    None
                }
            }),
        );
        let challenge_guard_root = root_from_strings(
            "wave98_challenge_guard_root",
            holdoffs
                .iter()
                .map(|holdoff| holdoff.challenge_guard_root.clone()),
        );
        let objection_guard_root = root_from_strings(
            "wave98_objection_guard_root",
            holdoffs
                .iter()
                .map(|holdoff| holdoff.objection_guard_root.clone()),
        );
        let appeal_guard_root = root_from_strings(
            "wave98_appeal_guard_root",
            holdoffs
                .iter()
                .map(|holdoff| holdoff.appeal_guard_root.clone()),
        );
        let command_root = root_from_strings(
            "wave98_challenge_holdoff_commands",
            holdoffs
                .iter()
                .map(|holdoff| holdoff.command_hint_root.clone()),
        );
        let lane_status = if holdoffs
            .iter()
            .all(|holdoff| holdoff.status == HoldoffStatus::HoldoffCleared)
        {
            HoldoffStatus::HoldoffCleared
        } else if holdoffs
            .iter()
            .any(|holdoff| holdoff.status == HoldoffStatus::ChallengeReady)
        {
            HoldoffStatus::ChallengeReady
        } else if holdoffs
            .iter()
            .any(|holdoff| holdoff.status == HoldoffStatus::WindowOpen)
        {
            HoldoffStatus::WindowOpen
        } else {
            HoldoffStatus::Blocked
        };
        Self {
            lane,
            lane_title_root,
            wave97_anti_equivocation_transcript_root,
            holdoffs,
            blocked_root,
            window_open_root,
            challenge_ready_root,
            holdoff_cleared_root,
            challenge_guard_root,
            objection_guard_root,
            appeal_guard_root,
            command_root,
            lane_status,
        }
    }

    pub fn blocked_count(&self) -> usize {
        self.holdoffs
            .iter()
            .filter(|holdoff| !holdoff.blockers.is_empty())
            .count()
    }

    pub fn window_open_count(&self) -> usize {
        self.holdoffs
            .iter()
            .filter(|holdoff| holdoff.status == HoldoffStatus::WindowOpen)
            .count()
    }

    pub fn challenge_ready_count(&self) -> usize {
        self.holdoffs
            .iter()
            .filter(|holdoff| holdoff.status == HoldoffStatus::ChallengeReady)
            .count()
    }

    pub fn holdoff_cleared_count(&self) -> usize {
        self.holdoffs
            .iter()
            .filter(|holdoff| holdoff.status == HoldoffStatus::HoldoffCleared)
            .count()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "lane_title_root": self.lane_title_root,
            "wave97_anti_equivocation_transcript_root": self.wave97_anti_equivocation_transcript_root,
            "holdoff_roots": self.holdoffs.iter().map(ReleaseClaimHoldoff::state_root).collect::<Vec<_>>(),
            "blocked_root": self.blocked_root,
            "window_open_root": self.window_open_root,
            "challenge_ready_root": self.challenge_ready_root,
            "holdoff_cleared_root": self.holdoff_cleared_root,
            "challenge_guard_root": self.challenge_guard_root,
            "objection_guard_root": self.objection_guard_root,
            "appeal_guard_root": self.appeal_guard_root,
            "command_root": self.command_root,
            "claim_count": self.holdoffs.len(),
            "blocked_count": self.blocked_count(),
            "window_open_count": self.window_open_count(),
            "challenge_ready_count": self.challenge_ready_count(),
            "holdoff_cleared_count": self.holdoff_cleared_count(),
            "lane_status": format!("{:?}", self.lane_status),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("lane_challenge_holdoff_ledger", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HoldoffSummary {
    pub holdoff_root: String,
    pub blocked_root: String,
    pub window_open_root: String,
    pub challenge_ready_root: String,
    pub holdoff_cleared_root: String,
    pub challenge_guard_root: String,
    pub objection_guard_root: String,
    pub appeal_guard_root: String,
    pub command_root: String,
    pub release_denial_root: String,
    pub lane_count: usize,
    pub claim_count: usize,
    pub blocked_count: usize,
    pub window_open_count: usize,
    pub challenge_ready_count: usize,
    pub holdoff_cleared_count: usize,
    pub all_lanes_cleared: bool,
    pub live_heavy_gates_ran: bool,
    pub production_release_denied: bool,
}

impl HoldoffSummary {
    pub fn from_ledgers(config: &Config, ledgers: &[LaneChallengeHoldoffLedger]) -> Self {
        let holdoff_root = root_from_strings(
            "wave98_holdoff_root",
            ledgers.iter().map(LaneChallengeHoldoffLedger::state_root),
        );
        let blocked_root = root_from_strings(
            "wave98_blocked_root",
            ledgers.iter().map(|ledger| ledger.blocked_root.clone()),
        );
        let window_open_root = root_from_strings(
            "wave98_window_open_root",
            ledgers.iter().map(|ledger| ledger.window_open_root.clone()),
        );
        let challenge_ready_root = root_from_strings(
            "wave98_challenge_ready_root",
            ledgers
                .iter()
                .map(|ledger| ledger.challenge_ready_root.clone()),
        );
        let holdoff_cleared_root = root_from_strings(
            "wave98_holdoff_cleared_root",
            ledgers
                .iter()
                .map(|ledger| ledger.holdoff_cleared_root.clone()),
        );
        let challenge_guard_root = root_from_strings(
            "wave98_challenge_guard_summary_root",
            ledgers
                .iter()
                .map(|ledger| ledger.challenge_guard_root.clone()),
        );
        let objection_guard_root = root_from_strings(
            "wave98_objection_guard_summary_root",
            ledgers
                .iter()
                .map(|ledger| ledger.objection_guard_root.clone()),
        );
        let appeal_guard_root = root_from_strings(
            "wave98_appeal_guard_summary_root",
            ledgers
                .iter()
                .map(|ledger| ledger.appeal_guard_root.clone()),
        );
        let command_root = root_from_strings(
            "wave98_command_root",
            ledgers.iter().map(|ledger| ledger.command_root.clone()),
        );
        let claim_count = ledgers
            .iter()
            .map(|ledger| ledger.holdoffs.len())
            .sum::<usize>();
        let blocked_count = ledgers
            .iter()
            .map(LaneChallengeHoldoffLedger::blocked_count)
            .sum::<usize>();
        let window_open_count = ledgers
            .iter()
            .map(LaneChallengeHoldoffLedger::window_open_count)
            .sum::<usize>();
        let challenge_ready_count = ledgers
            .iter()
            .map(LaneChallengeHoldoffLedger::challenge_ready_count)
            .sum::<usize>();
        let holdoff_cleared_count = ledgers
            .iter()
            .map(LaneChallengeHoldoffLedger::holdoff_cleared_count)
            .sum::<usize>();
        let all_lanes_cleared = ledgers.len() as u64 >= config.global_holdoff_threshold
            && ledgers
                .iter()
                .all(|ledger| ledger.lane_status == HoldoffStatus::HoldoffCleared);
        let live_heavy_gates_ran = false;
        let production_release_denied = config.deny_release_when_any_lane_blocked
            && (!all_lanes_cleared || !live_heavy_gates_ran);
        let denial_record = json!({
            "chain_id": config.chain_id,
            "protocol_version": config.protocol_version,
            "wave": config.wave,
            "seal_wave": config.seal_wave,
            "blocked_count": blocked_count,
            "window_open_count": window_open_count,
            "challenge_ready_count": challenge_ready_count,
            "holdoff_cleared_count": holdoff_cleared_count,
            "all_lanes_cleared": all_lanes_cleared,
            "live_heavy_gates_ran": live_heavy_gates_ran,
            "production_release_denied": production_release_denied,
        });
        let release_denial_root = record_root("release_denial", &denial_record);
        Self {
            holdoff_root,
            blocked_root,
            window_open_root,
            challenge_ready_root,
            holdoff_cleared_root,
            challenge_guard_root,
            objection_guard_root,
            appeal_guard_root,
            command_root,
            release_denial_root,
            lane_count: ledgers.len(),
            claim_count,
            blocked_count,
            window_open_count,
            challenge_ready_count,
            holdoff_cleared_count,
            all_lanes_cleared,
            live_heavy_gates_ran,
            production_release_denied,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "holdoff_root": self.holdoff_root,
            "blocked_root": self.blocked_root,
            "window_open_root": self.window_open_root,
            "challenge_ready_root": self.challenge_ready_root,
            "holdoff_cleared_root": self.holdoff_cleared_root,
            "challenge_guard_root": self.challenge_guard_root,
            "objection_guard_root": self.objection_guard_root,
            "appeal_guard_root": self.appeal_guard_root,
            "command_root": self.command_root,
            "release_denial_root": self.release_denial_root,
            "lane_count": self.lane_count,
            "claim_count": self.claim_count,
            "blocked_count": self.blocked_count,
            "window_open_count": self.window_open_count,
            "challenge_ready_count": self.challenge_ready_count,
            "holdoff_cleared_count": self.holdoff_cleared_count,
            "all_lanes_cleared": self.all_lanes_cleared,
            "live_heavy_gates_ran": self.live_heavy_gates_ran,
            "production_release_denied": self.production_release_denied,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("holdoff_summary", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub config: Config,
    pub wave97_anti_equivocation_transcript_root: String,
    pub ledgers: Vec<LaneChallengeHoldoffLedger>,
    pub summary: HoldoffSummary,
}

impl State {
    pub fn new(config: Config, ledgers: Vec<LaneChallengeHoldoffLedger>) -> Self {
        let wave97_anti_equivocation_transcript_root = label_root(
            "wave97_anti_equivocation_transcript",
            "all_lanes",
            "source",
            WAVE,
        );
        let summary = HoldoffSummary::from_ledgers(&config, &ledgers);
        Self {
            config,
            wave97_anti_equivocation_transcript_root,
            ledgers,
            summary,
        }
    }

    pub fn stage_release_claim_holdoff(
        &self,
        lane: LaneKind,
        slot_label: &str,
        release_claim_seal_root: &str,
        challenge_window_root: &str,
        objection_root: &str,
        reviewer_hold_root: &str,
        dispute_bond_root: &str,
        appeal_deadline_root: &str,
        holdoff_height: u64,
        challenge_window: u64,
        appeal_window: u64,
    ) -> Result<Self> {
        let known_challenge_roots = self.known_challenge_roots();
        let known_objection_roots = self.known_objection_roots();
        let mut found = false;
        let mut ledgers = Vec::with_capacity(self.ledgers.len());
        for ledger in &self.ledgers {
            if ledger.lane == lane {
                ledgers.push(ledger.stage_holdoff(
                    slot_label,
                    release_claim_seal_root,
                    challenge_window_root,
                    objection_root,
                    reviewer_hold_root,
                    dispute_bond_root,
                    appeal_deadline_root,
                    holdoff_height,
                    challenge_window,
                    appeal_window,
                    &self.config,
                    &known_challenge_roots,
                    &known_objection_roots,
                )?);
                found = true;
            } else {
                ledgers.push(ledger.clone());
            }
        }
        if !found {
            return Err(HoldoffError::LaneMissing);
        }
        Ok(Self::new(self.config.clone(), ledgers))
    }

    pub fn known_challenge_roots(&self) -> Vec<String> {
        self.ledgers
            .iter()
            .flat_map(|ledger| {
                ledger
                    .holdoffs
                    .iter()
                    .filter_map(|holdoff| holdoff.challenge_window_root.clone())
            })
            .collect::<Vec<_>>()
    }

    pub fn known_objection_roots(&self) -> Vec<String> {
        self.ledgers
            .iter()
            .flat_map(|ledger| {
                ledger
                    .holdoffs
                    .iter()
                    .filter_map(|holdoff| holdoff.objection_root.clone())
            })
            .collect::<Vec<_>>()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "config_root": self.config.state_root(),
            "wave97_anti_equivocation_transcript_root": self.wave97_anti_equivocation_transcript_root,
            "ledger_roots": self.ledgers.iter().map(LaneChallengeHoldoffLedger::state_root).collect::<Vec<_>>(),
            "summary": self.summary.public_record(),
            "roots_only": self.config.roots_only_public_records,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("state", &self.public_record())
    }
}

pub fn devnet() -> Runtime {
    let config = Config::default();
    let ledgers = vec![
        LaneChallengeHoldoffLedger::new(
            LaneKind::Compile,
            &[
                "cargo_check",
                "cargo_test",
                "clippy",
                "rustfmt",
                "rustc",
                "build_metadata",
                "operator_signoff",
            ],
            &config,
        ),
        LaneChallengeHoldoffLedger::new(
            LaneKind::RuntimeReplay,
            &[
                "replay_run",
                "rollback_drill",
                "adversarial_replay",
                "stale_archive_replacement",
                "live_execution_receipt",
                "operator_signoff",
            ],
            &config,
        ),
        LaneChallengeHoldoffLedger::new(
            LaneKind::AuditSecurity,
            &[
                "audit_review",
                "adversarial_scenario",
                "threat_model",
                "privacy_review",
                "reviewer_signoff",
                "operator_signoff",
            ],
            &config,
        ),
        LaneChallengeHoldoffLedger::new(
            LaneKind::BridgeCustody,
            &[
                "monero_watcher_quorum",
                "withdrawal_release",
                "reserve_coverage",
                "signer_quorum",
                "challenge_hold_review",
                "custody_operator_signoff",
            ],
            &config,
        ),
        LaneChallengeHoldoffLedger::new(
            LaneKind::WalletWatchtower,
            &[
                "wallet_escape_dry_run",
                "watchtower_quorum",
                "user_runbook_replay",
                "redacted_recovery_proof",
                "wallet_visible_receipt",
                "operator_signoff",
            ],
            &config,
        ),
        LaneChallengeHoldoffLedger::new(
            LaneKind::PqReservePrivacy,
            &[
                "ml_dsa_slh_dsa_authority_epoch",
                "pq_quorum",
                "reserve_coverage",
                "privacy_linkage",
                "metadata_redaction",
                "nullifier_separation",
                "operator_signoff",
            ],
            &config,
        ),
    ];
    State::new(config, ledgers)
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn initial_blockers(config: &Config) -> Vec<HoldoffBlockerKind> {
    let mut blockers = Vec::new();
    if config.require_release_claim_seal_root {
        blockers.push(HoldoffBlockerKind::MissingReleaseClaimSealRoot);
    }
    if config.require_challenge_window_root {
        blockers.push(HoldoffBlockerKind::MissingChallengeWindowRoot);
    }
    if config.require_objection_root {
        blockers.push(HoldoffBlockerKind::MissingObjectionRoot);
    }
    if config.require_reviewer_hold_root {
        blockers.push(HoldoffBlockerKind::MissingReviewerHoldRoot);
    }
    if config.require_dispute_bond_root {
        blockers.push(HoldoffBlockerKind::MissingDisputeBondRoot);
    }
    if config.require_appeal_deadline_root {
        blockers.push(HoldoffBlockerKind::MissingAppealDeadlineRoot);
    }
    blockers.push(HoldoffBlockerKind::HoldoffHeightTooLow);
    blockers.push(HoldoffBlockerKind::ChallengeWindowOpen);
    blockers.push(HoldoffBlockerKind::ChallengeWindowTooShort);
    blockers.push(HoldoffBlockerKind::AppealWindowOpen);
    blockers.push(HoldoffBlockerKind::AppealWindowTooShort);
    if config.require_live_heavy_gate_evidence {
        blockers.push(HoldoffBlockerKind::LiveHeavyGateEvidenceMissing);
    }
    blockers
}

fn blocked_root(holdoffs: &[ReleaseClaimHoldoff]) -> String {
    let leaves = holdoffs
        .iter()
        .flat_map(|holdoff| {
            holdoff.blockers.iter().map(move |blocker| {
                json!({
                    "lane": holdoff.lane.as_str(),
                    "slot_label": holdoff.slot_label,
                    "blocker": blocker.as_str(),
                    "claim_root": holdoff.state_root(),
                })
            })
        })
        .collect::<Vec<_>>();
    merkle_root("wave98_blocked_release_claim_holdoffs", &leaves)
}

fn root_from_strings<I>(domain: &str, values: I) -> String
where
    I: IntoIterator<Item = String>,
{
    let leaves = values.into_iter().map(Value::String).collect::<Vec<_>>();
    merkle_root(domain, &leaves)
}

fn holdoff_guard_root(
    guard_kind: &str,
    lane: LaneKind,
    slot_label: &str,
    release_claim_seal_root: &str,
    first_guard_root: &str,
    second_guard_root: &str,
    holdoff_height: u64,
) -> String {
    domain_hash(
        "wave98-live-heavy-gate-release-claim-challenge-window-holdoff-guard",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(guard_kind),
            HashPart::Str(lane.as_str()),
            HashPart::Str(slot_label),
            HashPart::Str(release_claim_seal_root),
            HashPart::Str(first_guard_root),
            HashPart::Str(second_guard_root),
            HashPart::U64(holdoff_height),
        ],
        32,
    )
}

fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "wave98-live-heavy-gate-release-claim-challenge-window-holdoff-record",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Json(record),
        ],
        32,
    )
}

fn label_root(kind: &str, lane: &str, label: &str, ordinal: u64) -> String {
    domain_hash(
        "wave98-live-heavy-gate-release-claim-challenge-window-holdoff-label",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(kind),
            HashPart::Str(lane),
            HashPart::Str(label),
            HashPart::U64(ordinal),
        ],
        32,
    )
}
