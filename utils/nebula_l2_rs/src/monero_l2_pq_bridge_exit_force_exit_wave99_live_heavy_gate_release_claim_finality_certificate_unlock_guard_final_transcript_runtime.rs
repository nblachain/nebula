use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::hash::{domain_hash, merkle_root, HashPart};

const CHAIN_ID: &str = "nebula-monero-private-l2-devnet";
const PROTOCOL_VERSION: &str =
    "wave99-live-heavy-gate-release-claim-finality-certificate-unlock-guard-v1";
const WAVE: u64 = 99;
const HOLDOFF_WAVE: u64 = 98;
const MIN_FINALITY_HEIGHT: u64 = 990_000;
const MIN_CONFIRMATION_DEPTH: u64 = 2_160;

pub type PublicRecord = Value;
pub type Runtime = State;
pub type Result<T> = core::result::Result<T, FinalityError>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FinalityError {
    LaneMissing,
    ClaimMissing,
    ReleaseClaimHoldoffRootMissing,
    FinalityCertificateRootMissing,
    EscrowAccountingRootMissing,
    RollbackGuardRootMissing,
    CircuitBreakerRootMissing,
    OperatorSignoffRootMissing,
    ReviewerSignoffRootMissing,
    FinalityHeightTooLow,
    ConfirmationDepthTooLow,
    EscrowUnlockStillLocked,
    CircuitBreakerActive,
    RollbackWindowOpen,
    UnlockStillBlocked,
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
            Self::Compile => "Compile finality certificate unlock guard",
            Self::RuntimeReplay => "Runtime replay finality certificate unlock guard",
            Self::AuditSecurity => "Audit security finality certificate unlock guard",
            Self::BridgeCustody => "Bridge custody finality certificate unlock guard",
            Self::WalletWatchtower => "Wallet watchtower finality certificate unlock guard",
            Self::PqReservePrivacy => "PQ reserve privacy finality certificate unlock guard",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificateStatus {
    Empty,
    Blocked,
    Candidate,
    UnlockGuarded,
    UnlockReady,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FinalityBlockerKind {
    MissingReleaseClaimHoldoffRoot,
    MissingFinalityCertificateRoot,
    MissingEscrowAccountingRoot,
    MissingRollbackGuardRoot,
    MissingCircuitBreakerRoot,
    MissingOperatorSignoffRoot,
    MissingReviewerSignoffRoot,
    FinalityHeightTooLow,
    ConfirmationDepthTooLow,
    EscrowUnlockStillLocked,
    CircuitBreakerActive,
    RollbackWindowOpen,
    LiveHeavyGateEvidenceMissing,
}

impl FinalityBlockerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingReleaseClaimHoldoffRoot => "missing_release_claim_holdoff_root",
            Self::MissingFinalityCertificateRoot => "missing_finality_certificate_root",
            Self::MissingEscrowAccountingRoot => "missing_escrow_accounting_root",
            Self::MissingRollbackGuardRoot => "missing_rollback_guard_root",
            Self::MissingCircuitBreakerRoot => "missing_circuit_breaker_root",
            Self::MissingOperatorSignoffRoot => "missing_operator_signoff_root",
            Self::MissingReviewerSignoffRoot => "missing_reviewer_signoff_root",
            Self::FinalityHeightTooLow => "finality_height_too_low",
            Self::ConfirmationDepthTooLow => "confirmation_depth_too_low",
            Self::EscrowUnlockStillLocked => "escrow_unlock_still_locked",
            Self::CircuitBreakerActive => "circuit_breaker_active",
            Self::RollbackWindowOpen => "rollback_window_open",
            Self::LiveHeavyGateEvidenceMissing => "live_heavy_gate_evidence_missing",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub wave: u64,
    pub holdoff_wave: u64,
    pub min_finality_height: u64,
    pub min_confirmation_depth: u64,
    pub lane_certificate_threshold: u64,
    pub global_certificate_threshold: u64,
    pub require_release_claim_holdoff_root: bool,
    pub require_finality_certificate_root: bool,
    pub require_escrow_accounting_root: bool,
    pub require_rollback_guard_root: bool,
    pub require_circuit_breaker_root: bool,
    pub require_operator_signoff_root: bool,
    pub require_reviewer_signoff_root: bool,
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
            holdoff_wave: HOLDOFF_WAVE,
            min_finality_height: MIN_FINALITY_HEIGHT,
            min_confirmation_depth: MIN_CONFIRMATION_DEPTH,
            lane_certificate_threshold: 1,
            global_certificate_threshold: 6,
            require_release_claim_holdoff_root: true,
            require_finality_certificate_root: true,
            require_escrow_accounting_root: true,
            require_rollback_guard_root: true,
            require_circuit_breaker_root: true,
            require_operator_signoff_root: true,
            require_reviewer_signoff_root: true,
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
            "holdoff_wave": self.holdoff_wave,
            "min_finality_height": self.min_finality_height,
            "min_confirmation_depth": self.min_confirmation_depth,
            "lane_certificate_threshold": self.lane_certificate_threshold,
            "global_certificate_threshold": self.global_certificate_threshold,
            "require_release_claim_holdoff_root": self.require_release_claim_holdoff_root,
            "require_finality_certificate_root": self.require_finality_certificate_root,
            "require_escrow_accounting_root": self.require_escrow_accounting_root,
            "require_rollback_guard_root": self.require_rollback_guard_root,
            "require_circuit_breaker_root": self.require_circuit_breaker_root,
            "require_operator_signoff_root": self.require_operator_signoff_root,
            "require_reviewer_signoff_root": self.require_reviewer_signoff_root,
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
pub struct ReleaseClaimFinalityCertificate {
    pub lane: LaneKind,
    pub slot_label: String,
    pub ordinal: u64,
    pub wave98_holdoff_ledger_root: String,
    pub release_claim_holdoff_root: Option<String>,
    pub finality_certificate_root: Option<String>,
    pub escrow_accounting_root: Option<String>,
    pub rollback_guard_root: Option<String>,
    pub circuit_breaker_root: Option<String>,
    pub operator_signoff_root: Option<String>,
    pub reviewer_signoff_root: Option<String>,
    pub finality_height: Option<u64>,
    pub confirmation_depth: Option<u64>,
    pub unlock_guard_root: String,
    pub accounting_guard_root: String,
    pub rollback_path_root: String,
    pub command_hint_root: String,
    pub blockers: Vec<FinalityBlockerKind>,
    pub status: CertificateStatus,
}

impl ReleaseClaimFinalityCertificate {
    pub fn empty(lane: LaneKind, slot_label: &str, ordinal: u64, config: &Config) -> Self {
        Self {
            lane,
            slot_label: slot_label.to_string(),
            ordinal,
            wave98_holdoff_ledger_root: label_root(
                "wave98_holdoff_ledger",
                lane.as_str(),
                slot_label,
                ordinal,
            ),
            release_claim_holdoff_root: None,
            finality_certificate_root: None,
            escrow_accounting_root: None,
            rollback_guard_root: None,
            circuit_breaker_root: None,
            operator_signoff_root: None,
            reviewer_signoff_root: None,
            finality_height: None,
            confirmation_depth: None,
            unlock_guard_root: label_root("unlock_guard", lane.as_str(), slot_label, ordinal),
            accounting_guard_root: label_root(
                "accounting_guard",
                lane.as_str(),
                slot_label,
                ordinal,
            ),
            rollback_path_root: label_root("rollback_path", lane.as_str(), slot_label, ordinal),
            command_hint_root: label_root(
                "finality_certificate_unlock_command",
                lane.as_str(),
                slot_label,
                ordinal,
            ),
            blockers: initial_blockers(config),
            status: CertificateStatus::Blocked,
        }
    }

    pub fn stage_certificate(
        &self,
        release_claim_holdoff_root: &str,
        finality_certificate_root: &str,
        escrow_accounting_root: &str,
        rollback_guard_root: &str,
        circuit_breaker_root: &str,
        operator_signoff_root: &str,
        reviewer_signoff_root: &str,
        finality_height: u64,
        confirmation_depth: u64,
        config: &Config,
        escrow_locked: bool,
        circuit_breaker_active: bool,
        rollback_open: bool,
    ) -> Result<Self> {
        if release_claim_holdoff_root.is_empty() {
            return Err(FinalityError::ReleaseClaimHoldoffRootMissing);
        }
        if finality_certificate_root.is_empty() {
            return Err(FinalityError::FinalityCertificateRootMissing);
        }
        if escrow_accounting_root.is_empty() {
            return Err(FinalityError::EscrowAccountingRootMissing);
        }
        if rollback_guard_root.is_empty() {
            return Err(FinalityError::RollbackGuardRootMissing);
        }
        if circuit_breaker_root.is_empty() {
            return Err(FinalityError::CircuitBreakerRootMissing);
        }
        if operator_signoff_root.is_empty() {
            return Err(FinalityError::OperatorSignoffRootMissing);
        }
        if reviewer_signoff_root.is_empty() {
            return Err(FinalityError::ReviewerSignoffRootMissing);
        }
        if finality_height < config.min_finality_height {
            return Err(FinalityError::FinalityHeightTooLow);
        }
        if confirmation_depth < config.min_confirmation_depth {
            return Err(FinalityError::ConfirmationDepthTooLow);
        }
        if escrow_locked {
            return Err(FinalityError::EscrowUnlockStillLocked);
        }
        if circuit_breaker_active {
            return Err(FinalityError::CircuitBreakerActive);
        }
        if rollback_open {
            return Err(FinalityError::RollbackWindowOpen);
        }

        let mut next = self.clone();
        next.release_claim_holdoff_root = Some(release_claim_holdoff_root.to_string());
        next.finality_certificate_root = Some(finality_certificate_root.to_string());
        next.escrow_accounting_root = Some(escrow_accounting_root.to_string());
        next.rollback_guard_root = Some(rollback_guard_root.to_string());
        next.circuit_breaker_root = Some(circuit_breaker_root.to_string());
        next.operator_signoff_root = Some(operator_signoff_root.to_string());
        next.reviewer_signoff_root = Some(reviewer_signoff_root.to_string());
        next.finality_height = Some(finality_height);
        next.confirmation_depth = Some(confirmation_depth);
        next.unlock_guard_root = unlock_guard_root(
            "unlock",
            self.lane,
            &self.slot_label,
            release_claim_holdoff_root,
            finality_certificate_root,
            escrow_accounting_root,
            finality_height,
            confirmation_depth,
        );
        next.accounting_guard_root = unlock_guard_root(
            "accounting",
            self.lane,
            &self.slot_label,
            release_claim_holdoff_root,
            finality_certificate_root,
            escrow_accounting_root,
            finality_height,
            confirmation_depth,
        );
        next.rollback_path_root = unlock_guard_root(
            "rollback",
            self.lane,
            &self.slot_label,
            release_claim_holdoff_root,
            rollback_guard_root,
            circuit_breaker_root,
            finality_height,
            confirmation_depth,
        );
        next.blockers =
            next.active_blockers(config, escrow_locked, circuit_breaker_active, rollback_open);
        next.status = if next.blockers.is_empty() {
            CertificateStatus::UnlockGuarded
        } else {
            CertificateStatus::Blocked
        };
        Ok(next)
    }

    pub fn mark_unlock_ready(&self) -> Result<Self> {
        if !self.blockers.is_empty() {
            return Err(FinalityError::UnlockStillBlocked);
        }
        let mut next = self.clone();
        next.status = CertificateStatus::UnlockReady;
        Ok(next)
    }

    fn active_blockers(
        &self,
        config: &Config,
        escrow_locked: bool,
        circuit_breaker_active: bool,
        rollback_open: bool,
    ) -> Vec<FinalityBlockerKind> {
        let mut blockers = Vec::new();
        if config.require_release_claim_holdoff_root && self.release_claim_holdoff_root.is_none() {
            blockers.push(FinalityBlockerKind::MissingReleaseClaimHoldoffRoot);
        }
        if config.require_finality_certificate_root && self.finality_certificate_root.is_none() {
            blockers.push(FinalityBlockerKind::MissingFinalityCertificateRoot);
        }
        if config.require_escrow_accounting_root && self.escrow_accounting_root.is_none() {
            blockers.push(FinalityBlockerKind::MissingEscrowAccountingRoot);
        }
        if config.require_rollback_guard_root && self.rollback_guard_root.is_none() {
            blockers.push(FinalityBlockerKind::MissingRollbackGuardRoot);
        }
        if config.require_circuit_breaker_root && self.circuit_breaker_root.is_none() {
            blockers.push(FinalityBlockerKind::MissingCircuitBreakerRoot);
        }
        if config.require_operator_signoff_root && self.operator_signoff_root.is_none() {
            blockers.push(FinalityBlockerKind::MissingOperatorSignoffRoot);
        }
        if config.require_reviewer_signoff_root && self.reviewer_signoff_root.is_none() {
            blockers.push(FinalityBlockerKind::MissingReviewerSignoffRoot);
        }
        match self.finality_height {
            Some(height) if height >= config.min_finality_height => {}
            _ => blockers.push(FinalityBlockerKind::FinalityHeightTooLow),
        }
        match self.confirmation_depth {
            Some(depth) if depth >= config.min_confirmation_depth => {}
            _ => blockers.push(FinalityBlockerKind::ConfirmationDepthTooLow),
        }
        if escrow_locked {
            blockers.push(FinalityBlockerKind::EscrowUnlockStillLocked);
        }
        if circuit_breaker_active {
            blockers.push(FinalityBlockerKind::CircuitBreakerActive);
        }
        if rollback_open {
            blockers.push(FinalityBlockerKind::RollbackWindowOpen);
        }
        if config.require_live_heavy_gate_evidence {
            blockers.push(FinalityBlockerKind::LiveHeavyGateEvidenceMissing);
        }
        blockers
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "slot_label": self.slot_label,
            "ordinal": self.ordinal,
            "wave98_holdoff_ledger_root": self.wave98_holdoff_ledger_root,
            "release_claim_holdoff_root": self.release_claim_holdoff_root,
            "finality_certificate_root": self.finality_certificate_root,
            "escrow_accounting_root": self.escrow_accounting_root,
            "rollback_guard_root": self.rollback_guard_root,
            "circuit_breaker_root": self.circuit_breaker_root,
            "operator_signoff_root": self.operator_signoff_root,
            "reviewer_signoff_root": self.reviewer_signoff_root,
            "finality_height": self.finality_height,
            "confirmation_depth": self.confirmation_depth,
            "unlock_guard_root": self.unlock_guard_root,
            "accounting_guard_root": self.accounting_guard_root,
            "rollback_path_root": self.rollback_path_root,
            "command_hint_root": self.command_hint_root,
            "blockers": self.blockers.iter().map(|blocker| blocker.as_str()).collect::<Vec<_>>(),
            "status": format!("{:?}", self.status),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("release_claim_finality_certificate", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LaneFinalityUnlockGuard {
    pub lane: LaneKind,
    pub lane_title_root: String,
    pub wave98_holdoff_transcript_root: String,
    pub certificates: Vec<ReleaseClaimFinalityCertificate>,
    pub blocked_root: String,
    pub candidate_root: String,
    pub unlock_guarded_root: String,
    pub unlock_ready_root: String,
    pub unlock_guard_root: String,
    pub accounting_guard_root: String,
    pub rollback_path_root: String,
    pub command_root: String,
    pub lane_status: CertificateStatus,
}

impl LaneFinalityUnlockGuard {
    pub fn new(lane: LaneKind, slot_labels: &[&str], config: &Config) -> Self {
        let certificates = slot_labels
            .iter()
            .enumerate()
            .map(|(index, label)| {
                ReleaseClaimFinalityCertificate::empty(lane, label, index as u64, config)
            })
            .collect::<Vec<_>>();
        Self::from_certificates(
            lane,
            label_root("lane_title", lane.as_str(), lane.title(), WAVE),
            label_root("wave98_holdoff_transcript", lane.as_str(), "source", WAVE),
            certificates,
        )
    }

    pub fn stage_certificate(
        &self,
        slot_label: &str,
        release_claim_holdoff_root: &str,
        finality_certificate_root: &str,
        escrow_accounting_root: &str,
        rollback_guard_root: &str,
        circuit_breaker_root: &str,
        operator_signoff_root: &str,
        reviewer_signoff_root: &str,
        finality_height: u64,
        confirmation_depth: u64,
        config: &Config,
        escrow_locked: bool,
        circuit_breaker_active: bool,
        rollback_open: bool,
    ) -> Result<Self> {
        let mut found = false;
        let mut certificates = Vec::with_capacity(self.certificates.len());
        for certificate in &self.certificates {
            if certificate.slot_label == slot_label {
                certificates.push(certificate.stage_certificate(
                    release_claim_holdoff_root,
                    finality_certificate_root,
                    escrow_accounting_root,
                    rollback_guard_root,
                    circuit_breaker_root,
                    operator_signoff_root,
                    reviewer_signoff_root,
                    finality_height,
                    confirmation_depth,
                    config,
                    escrow_locked,
                    circuit_breaker_active,
                    rollback_open,
                )?);
                found = true;
            } else {
                certificates.push(certificate.clone());
            }
        }
        if !found {
            return Err(FinalityError::ClaimMissing);
        }
        Ok(Self::from_certificates(
            self.lane,
            self.lane_title_root.clone(),
            self.wave98_holdoff_transcript_root.clone(),
            certificates,
        ))
    }

    fn from_certificates(
        lane: LaneKind,
        lane_title_root: String,
        wave98_holdoff_transcript_root: String,
        certificates: Vec<ReleaseClaimFinalityCertificate>,
    ) -> Self {
        let blocked_root = blocked_root(&certificates);
        let candidate_root = root_from_strings(
            "wave99_candidate_finality_certificates",
            certificates.iter().filter_map(|certificate| {
                if certificate.status == CertificateStatus::Candidate {
                    Some(certificate.state_root())
                } else {
                    None
                }
            }),
        );
        let unlock_guarded_root = root_from_strings(
            "wave99_unlock_guarded_certificates",
            certificates.iter().filter_map(|certificate| {
                if certificate.status == CertificateStatus::UnlockGuarded {
                    Some(certificate.state_root())
                } else {
                    None
                }
            }),
        );
        let unlock_ready_root = root_from_strings(
            "wave99_unlock_ready_certificates",
            certificates.iter().filter_map(|certificate| {
                if certificate.status == CertificateStatus::UnlockReady {
                    Some(certificate.state_root())
                } else {
                    None
                }
            }),
        );
        let unlock_guard_root = root_from_strings(
            "wave99_unlock_guards",
            certificates
                .iter()
                .map(|certificate| certificate.unlock_guard_root.clone()),
        );
        let accounting_guard_root = root_from_strings(
            "wave99_accounting_guards",
            certificates
                .iter()
                .map(|certificate| certificate.accounting_guard_root.clone()),
        );
        let rollback_path_root = root_from_strings(
            "wave99_rollback_paths",
            certificates
                .iter()
                .map(|certificate| certificate.rollback_path_root.clone()),
        );
        let command_root = root_from_strings(
            "wave99_finality_unlock_commands",
            certificates
                .iter()
                .map(|certificate| certificate.command_hint_root.clone()),
        );
        let lane_status = if certificates
            .iter()
            .all(|certificate| certificate.status == CertificateStatus::UnlockReady)
        {
            CertificateStatus::UnlockReady
        } else if certificates
            .iter()
            .any(|certificate| certificate.status == CertificateStatus::UnlockGuarded)
        {
            CertificateStatus::UnlockGuarded
        } else if certificates
            .iter()
            .any(|certificate| certificate.status == CertificateStatus::Candidate)
        {
            CertificateStatus::Candidate
        } else {
            CertificateStatus::Blocked
        };
        Self {
            lane,
            lane_title_root,
            wave98_holdoff_transcript_root,
            certificates,
            blocked_root,
            candidate_root,
            unlock_guarded_root,
            unlock_ready_root,
            unlock_guard_root,
            accounting_guard_root,
            rollback_path_root,
            command_root,
            lane_status,
        }
    }

    pub fn blocked_count(&self) -> usize {
        self.certificates
            .iter()
            .filter(|certificate| !certificate.blockers.is_empty())
            .count()
    }

    pub fn candidate_count(&self) -> usize {
        self.certificates
            .iter()
            .filter(|certificate| certificate.status == CertificateStatus::Candidate)
            .count()
    }

    pub fn unlock_guarded_count(&self) -> usize {
        self.certificates
            .iter()
            .filter(|certificate| certificate.status == CertificateStatus::UnlockGuarded)
            .count()
    }

    pub fn unlock_ready_count(&self) -> usize {
        self.certificates
            .iter()
            .filter(|certificate| certificate.status == CertificateStatus::UnlockReady)
            .count()
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "lane": self.lane.as_str(),
            "lane_title_root": self.lane_title_root,
            "wave98_holdoff_transcript_root": self.wave98_holdoff_transcript_root,
            "certificate_roots": self.certificates.iter().map(ReleaseClaimFinalityCertificate::state_root).collect::<Vec<_>>(),
            "blocked_root": self.blocked_root,
            "candidate_root": self.candidate_root,
            "unlock_guarded_root": self.unlock_guarded_root,
            "unlock_ready_root": self.unlock_ready_root,
            "unlock_guard_root": self.unlock_guard_root,
            "accounting_guard_root": self.accounting_guard_root,
            "rollback_path_root": self.rollback_path_root,
            "command_root": self.command_root,
            "claim_count": self.certificates.len(),
            "blocked_count": self.blocked_count(),
            "candidate_count": self.candidate_count(),
            "unlock_guarded_count": self.unlock_guarded_count(),
            "unlock_ready_count": self.unlock_ready_count(),
            "lane_status": format!("{:?}", self.lane_status),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("lane_finality_unlock_guard", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FinalitySummary {
    pub finality_root: String,
    pub blocked_root: String,
    pub candidate_root: String,
    pub unlock_guarded_root: String,
    pub unlock_ready_root: String,
    pub unlock_guard_root: String,
    pub accounting_guard_root: String,
    pub rollback_path_root: String,
    pub command_root: String,
    pub release_denial_root: String,
    pub lane_count: usize,
    pub claim_count: usize,
    pub blocked_count: usize,
    pub candidate_count: usize,
    pub unlock_guarded_count: usize,
    pub unlock_ready_count: usize,
    pub all_lanes_unlock_ready: bool,
    pub live_heavy_gates_ran: bool,
    pub production_release_denied: bool,
}

impl FinalitySummary {
    pub fn from_guards(config: &Config, guards: &[LaneFinalityUnlockGuard]) -> Self {
        let finality_root = root_from_strings(
            "wave99_finality_root",
            guards.iter().map(LaneFinalityUnlockGuard::state_root),
        );
        let blocked_root = root_from_strings(
            "wave99_blocked_root",
            guards.iter().map(|guard| guard.blocked_root.clone()),
        );
        let candidate_root = root_from_strings(
            "wave99_candidate_root",
            guards.iter().map(|guard| guard.candidate_root.clone()),
        );
        let unlock_guarded_root = root_from_strings(
            "wave99_unlock_guarded_root",
            guards.iter().map(|guard| guard.unlock_guarded_root.clone()),
        );
        let unlock_ready_root = root_from_strings(
            "wave99_unlock_ready_root",
            guards.iter().map(|guard| guard.unlock_ready_root.clone()),
        );
        let unlock_guard_root = root_from_strings(
            "wave99_unlock_guard_root",
            guards.iter().map(|guard| guard.unlock_guard_root.clone()),
        );
        let accounting_guard_root = root_from_strings(
            "wave99_accounting_guard_root",
            guards
                .iter()
                .map(|guard| guard.accounting_guard_root.clone()),
        );
        let rollback_path_root = root_from_strings(
            "wave99_rollback_path_root",
            guards.iter().map(|guard| guard.rollback_path_root.clone()),
        );
        let command_root = root_from_strings(
            "wave99_command_root",
            guards.iter().map(|guard| guard.command_root.clone()),
        );
        let claim_count = guards
            .iter()
            .map(|guard| guard.certificates.len())
            .sum::<usize>();
        let blocked_count = guards
            .iter()
            .map(LaneFinalityUnlockGuard::blocked_count)
            .sum::<usize>();
        let candidate_count = guards
            .iter()
            .map(LaneFinalityUnlockGuard::candidate_count)
            .sum::<usize>();
        let unlock_guarded_count = guards
            .iter()
            .map(LaneFinalityUnlockGuard::unlock_guarded_count)
            .sum::<usize>();
        let unlock_ready_count = guards
            .iter()
            .map(LaneFinalityUnlockGuard::unlock_ready_count)
            .sum::<usize>();
        let all_lanes_unlock_ready = guards.len() as u64 >= config.global_certificate_threshold
            && guards
                .iter()
                .all(|guard| guard.lane_status == CertificateStatus::UnlockReady);
        let live_heavy_gates_ran = false;
        let production_release_denied = config.deny_release_when_any_lane_blocked
            && (!all_lanes_unlock_ready || !live_heavy_gates_ran);
        let denial_record = json!({
            "chain_id": config.chain_id,
            "protocol_version": config.protocol_version,
            "wave": config.wave,
            "holdoff_wave": config.holdoff_wave,
            "blocked_count": blocked_count,
            "candidate_count": candidate_count,
            "unlock_guarded_count": unlock_guarded_count,
            "unlock_ready_count": unlock_ready_count,
            "all_lanes_unlock_ready": all_lanes_unlock_ready,
            "live_heavy_gates_ran": live_heavy_gates_ran,
            "production_release_denied": production_release_denied,
        });
        let release_denial_root = record_root("release_denial", &denial_record);
        Self {
            finality_root,
            blocked_root,
            candidate_root,
            unlock_guarded_root,
            unlock_ready_root,
            unlock_guard_root,
            accounting_guard_root,
            rollback_path_root,
            command_root,
            release_denial_root,
            lane_count: guards.len(),
            claim_count,
            blocked_count,
            candidate_count,
            unlock_guarded_count,
            unlock_ready_count,
            all_lanes_unlock_ready,
            live_heavy_gates_ran,
            production_release_denied,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "finality_root": self.finality_root,
            "blocked_root": self.blocked_root,
            "candidate_root": self.candidate_root,
            "unlock_guarded_root": self.unlock_guarded_root,
            "unlock_ready_root": self.unlock_ready_root,
            "unlock_guard_root": self.unlock_guard_root,
            "accounting_guard_root": self.accounting_guard_root,
            "rollback_path_root": self.rollback_path_root,
            "command_root": self.command_root,
            "release_denial_root": self.release_denial_root,
            "lane_count": self.lane_count,
            "claim_count": self.claim_count,
            "blocked_count": self.blocked_count,
            "candidate_count": self.candidate_count,
            "unlock_guarded_count": self.unlock_guarded_count,
            "unlock_ready_count": self.unlock_ready_count,
            "all_lanes_unlock_ready": self.all_lanes_unlock_ready,
            "live_heavy_gates_ran": self.live_heavy_gates_ran,
            "production_release_denied": self.production_release_denied,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("finality_summary", &self.public_record())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub config: Config,
    pub wave98_holdoff_transcript_root: String,
    pub guards: Vec<LaneFinalityUnlockGuard>,
    pub summary: FinalitySummary,
}

impl State {
    pub fn new(config: Config, guards: Vec<LaneFinalityUnlockGuard>) -> Self {
        let wave98_holdoff_transcript_root =
            label_root("wave98_holdoff_transcript", "all_lanes", "source", WAVE);
        let summary = FinalitySummary::from_guards(&config, &guards);
        Self {
            config,
            wave98_holdoff_transcript_root,
            guards,
            summary,
        }
    }

    pub fn stage_release_claim_finality_certificate(
        &self,
        lane: LaneKind,
        slot_label: &str,
        release_claim_holdoff_root: &str,
        finality_certificate_root: &str,
        escrow_accounting_root: &str,
        rollback_guard_root: &str,
        circuit_breaker_root: &str,
        operator_signoff_root: &str,
        reviewer_signoff_root: &str,
        finality_height: u64,
        confirmation_depth: u64,
        escrow_locked: bool,
        circuit_breaker_active: bool,
        rollback_open: bool,
    ) -> Result<Self> {
        let mut found = false;
        let mut guards = Vec::with_capacity(self.guards.len());
        for guard in &self.guards {
            if guard.lane == lane {
                guards.push(guard.stage_certificate(
                    slot_label,
                    release_claim_holdoff_root,
                    finality_certificate_root,
                    escrow_accounting_root,
                    rollback_guard_root,
                    circuit_breaker_root,
                    operator_signoff_root,
                    reviewer_signoff_root,
                    finality_height,
                    confirmation_depth,
                    &self.config,
                    escrow_locked,
                    circuit_breaker_active,
                    rollback_open,
                )?);
                found = true;
            } else {
                guards.push(guard.clone());
            }
        }
        if !found {
            return Err(FinalityError::LaneMissing);
        }
        Ok(Self::new(self.config.clone(), guards))
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "config_root": self.config.state_root(),
            "wave98_holdoff_transcript_root": self.wave98_holdoff_transcript_root,
            "guard_roots": self.guards.iter().map(LaneFinalityUnlockGuard::state_root).collect::<Vec<_>>(),
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
    let guards = vec![
        LaneFinalityUnlockGuard::new(
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
        LaneFinalityUnlockGuard::new(
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
        LaneFinalityUnlockGuard::new(
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
        LaneFinalityUnlockGuard::new(
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
        LaneFinalityUnlockGuard::new(
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
        LaneFinalityUnlockGuard::new(
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
    State::new(config, guards)
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn initial_blockers(config: &Config) -> Vec<FinalityBlockerKind> {
    let mut blockers = Vec::new();
    if config.require_release_claim_holdoff_root {
        blockers.push(FinalityBlockerKind::MissingReleaseClaimHoldoffRoot);
    }
    if config.require_finality_certificate_root {
        blockers.push(FinalityBlockerKind::MissingFinalityCertificateRoot);
    }
    if config.require_escrow_accounting_root {
        blockers.push(FinalityBlockerKind::MissingEscrowAccountingRoot);
    }
    if config.require_rollback_guard_root {
        blockers.push(FinalityBlockerKind::MissingRollbackGuardRoot);
    }
    if config.require_circuit_breaker_root {
        blockers.push(FinalityBlockerKind::MissingCircuitBreakerRoot);
    }
    if config.require_operator_signoff_root {
        blockers.push(FinalityBlockerKind::MissingOperatorSignoffRoot);
    }
    if config.require_reviewer_signoff_root {
        blockers.push(FinalityBlockerKind::MissingReviewerSignoffRoot);
    }
    blockers.push(FinalityBlockerKind::FinalityHeightTooLow);
    blockers.push(FinalityBlockerKind::ConfirmationDepthTooLow);
    blockers.push(FinalityBlockerKind::EscrowUnlockStillLocked);
    blockers.push(FinalityBlockerKind::CircuitBreakerActive);
    blockers.push(FinalityBlockerKind::RollbackWindowOpen);
    if config.require_live_heavy_gate_evidence {
        blockers.push(FinalityBlockerKind::LiveHeavyGateEvidenceMissing);
    }
    blockers
}

fn blocked_root(certificates: &[ReleaseClaimFinalityCertificate]) -> String {
    let leaves = certificates
        .iter()
        .flat_map(|certificate| {
            certificate.blockers.iter().map(move |blocker| {
                json!({
                    "lane": certificate.lane.as_str(),
                    "slot_label": certificate.slot_label,
                    "blocker": blocker.as_str(),
                    "certificate_root": certificate.state_root(),
                })
            })
        })
        .collect::<Vec<_>>();
    merkle_root("wave99_blocked_finality_certificates", &leaves)
}

fn root_from_strings<I>(domain: &str, values: I) -> String
where
    I: IntoIterator<Item = String>,
{
    let leaves = values.into_iter().map(Value::String).collect::<Vec<_>>();
    merkle_root(domain, &leaves)
}

fn unlock_guard_root(
    guard_kind: &str,
    lane: LaneKind,
    slot_label: &str,
    release_claim_holdoff_root: &str,
    first_guard_root: &str,
    second_guard_root: &str,
    finality_height: u64,
    confirmation_depth: u64,
) -> String {
    domain_hash(
        "wave99-live-heavy-gate-release-claim-finality-certificate-unlock-guard",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(guard_kind),
            HashPart::Str(lane.as_str()),
            HashPart::Str(slot_label),
            HashPart::Str(release_claim_holdoff_root),
            HashPart::Str(first_guard_root),
            HashPart::Str(second_guard_root),
            HashPart::U64(finality_height),
            HashPart::U64(confirmation_depth),
        ],
        32,
    )
}

fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "wave99-live-heavy-gate-release-claim-finality-certificate-record",
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
        "wave99-live-heavy-gate-release-claim-finality-certificate-label",
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
