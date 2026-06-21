use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::hash::{domain_hash, merkle_root, HashPart};

pub type Result<T> = std::result::Result<T, String>;
pub type Runtime = State;

pub const PROTOCOL_VERSION: &str =
    "monero-l2-pq-bridge-force-exit-wave93-receipt-admission-quarantine-bridge-custody-v1";
pub const DEVNET_CHAIN_ID: &str = "nebula-devnet";
pub const DEVNET_LANE_ID: &str = "bridge-custody-force-exit";
pub const WAVE92_SLOT_ROOT: &str =
    "root:wave92:bridge-custody-receipt-slot-registry-fail-closed-placeholder";
pub const EMPTY_ROOT: &str =
    "root:wave93:0000000000000000000000000000000000000000000000000000000000000000";
pub const DEFAULT_MIN_WATCHER_QUORUM: u16 = 4;
pub const DEFAULT_MIN_SIGNER_QUORUM: u16 = 3;
pub const DEFAULT_MIN_OPERATOR_SIGNOFF: u16 = 2;
pub const DEFAULT_MIN_RESERVE_COVERAGE_BPS: u16 = 10_000;
pub const DEFAULT_CHALLENGE_HOLD_BLOCKS: u64 = 720;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub chain_id: String,
    pub lane_id: String,
    pub protocol_version: String,
    pub wave92_slot_root: String,
    pub accepted_root_domain: String,
    pub quarantine_root_domain: String,
    pub min_watcher_quorum: u16,
    pub min_signer_quorum: u16,
    pub min_operator_signoff: u16,
    pub min_reserve_coverage_bps: u16,
    pub challenge_hold_blocks: u64,
    pub roots_only_public_records: bool,
    pub live_receipts_enabled: bool,
    pub heavy_gates_ran: bool,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: DEVNET_CHAIN_ID.to_string(),
            lane_id: DEVNET_LANE_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            wave92_slot_root: WAVE92_SLOT_ROOT.to_string(),
            accepted_root_domain: "wave93-bridge-custody-admitted-receipt-root".to_string(),
            quarantine_root_domain: "wave93-bridge-custody-quarantined-receipt-root".to_string(),
            min_watcher_quorum: DEFAULT_MIN_WATCHER_QUORUM,
            min_signer_quorum: DEFAULT_MIN_SIGNER_QUORUM,
            min_operator_signoff: DEFAULT_MIN_OPERATOR_SIGNOFF,
            min_reserve_coverage_bps: DEFAULT_MIN_RESERVE_COVERAGE_BPS,
            challenge_hold_blocks: DEFAULT_CHALLENGE_HOLD_BLOCKS,
            roots_only_public_records: true,
            live_receipts_enabled: false,
            heavy_gates_ran: false,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_text("chain_id", &self.chain_id)?;
        ensure_text("lane_id", &self.lane_id)?;
        ensure_text("protocol_version", &self.protocol_version)?;
        ensure_root_like("wave92_slot_root", &self.wave92_slot_root)?;
        ensure_text("accepted_root_domain", &self.accepted_root_domain)?;
        ensure_text("quarantine_root_domain", &self.quarantine_root_domain)?;
        if self.min_watcher_quorum == 0 {
            return Err("watcher quorum must be nonzero".to_string());
        }
        if self.min_signer_quorum == 0 {
            return Err("signer quorum must be nonzero".to_string());
        }
        if self.min_operator_signoff == 0 {
            return Err("operator signoff quorum must be nonzero".to_string());
        }
        if self.min_reserve_coverage_bps > 10_000 {
            return Err("reserve coverage bps exceeds full coverage".to_string());
        }
        if self.challenge_hold_blocks == 0 {
            return Err("challenge hold blocks must be nonzero".to_string());
        }
        if !self.roots_only_public_records {
            return Err("public records must remain roots only".to_string());
        }
        if self.heavy_gates_ran {
            return Err("wave93 admission lane must not claim heavy gates ran".to_string());
        }
        Ok(())
    }

    pub fn root(&self) -> String {
        root_for("config", &self.public_record())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "lane_id": self.lane_id,
            "protocol_version": self.protocol_version,
            "wave92_slot_root": self.wave92_slot_root,
            "accepted_root_domain": self.accepted_root_domain,
            "quarantine_root_domain": self.quarantine_root_domain,
            "min_watcher_quorum": self.min_watcher_quorum,
            "min_signer_quorum": self.min_signer_quorum,
            "min_operator_signoff": self.min_operator_signoff,
            "min_reserve_coverage_bps": self.min_reserve_coverage_bps,
            "challenge_hold_blocks": self.challenge_hold_blocks,
            "roots_only_public_records": self.roots_only_public_records,
            "live_receipts_enabled": self.live_receipts_enabled,
            "heavy_gates_ran": self.heavy_gates_ran,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub config: Config,
    pub admitted_roots: BTreeMap<String, ReceiptRootAdmission>,
    pub quarantined_roots: BTreeMap<String, QuarantinedReceiptRoot>,
    pub admission_rules: Vec<AdmissionRule>,
    pub operator_commands: Vec<OperatorCommand>,
    pub last_verdict: AdmissionVerdict,
}

impl State {
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;
        let admission_rules = AdmissionRule::canonical_rules(&config);
        let operator_commands = OperatorCommand::canonical_commands();
        let mut state = Self {
            config,
            admitted_roots: BTreeMap::new(),
            quarantined_roots: BTreeMap::new(),
            admission_rules,
            operator_commands,
            last_verdict: AdmissionVerdict::fail_closed("no admitted future receipt roots"),
        };
        state.last_verdict = state.evaluate();
        Ok(state)
    }

    pub fn devnet() -> Result<Self> {
        Self::new(Config::devnet())
    }

    pub fn validate(&self) -> Result<()> {
        self.config.validate()?;
        for rule in &self.admission_rules {
            rule.validate()?;
        }
        for command in &self.operator_commands {
            command.validate()?;
        }
        for admission in self.admitted_roots.values() {
            admission.validate(&self.config)?;
        }
        for quarantine in self.quarantined_roots.values() {
            quarantine.validate()?;
        }
        if !self.admitted_roots.is_empty() {
            return Err("live receipt admission is disabled in this fail closed lane".to_string());
        }
        Ok(())
    }

    pub fn submit_future_receipt_root(
        &mut self,
        candidate: FutureReceiptRootCandidate,
    ) -> Result<AdmissionVerdict> {
        candidate.validate()?;
        let blockers = candidate.blockers(&self.config);
        if blockers.is_empty() && self.config.live_receipts_enabled {
            let admission = ReceiptRootAdmission::from_candidate(&self.config, candidate);
            self.admitted_roots
                .insert(admission.admission_root.clone(), admission);
        } else {
            let quarantine = QuarantinedReceiptRoot::from_candidate(candidate, blockers);
            self.quarantined_roots
                .insert(quarantine.quarantine_root.clone(), quarantine);
        }
        self.last_verdict = self.evaluate();
        Ok(self.last_verdict.clone())
    }

    pub fn apply_operator_command(&mut self, command_root: String) -> Result<AdmissionVerdict> {
        ensure_root_like("command_root", &command_root)?;
        let mut found = false;
        for command in &mut self.operator_commands {
            if command.command_root == command_root {
                command.applied = true;
                found = true;
            }
        }
        if !found {
            return Err("operator command root is not registered for this lane".to_string());
        }
        self.last_verdict = self.evaluate();
        Ok(self.last_verdict.clone())
    }

    pub fn evaluate(&self) -> AdmissionVerdict {
        if let Err(reason) = self.validate() {
            return AdmissionVerdict::fail_closed(reason);
        }
        if self.admitted_roots.is_empty() {
            return AdmissionVerdict::fail_closed("empty admitted root set");
        }
        AdmissionVerdict {
            status: AdmissionStatus::Admitted,
            clearable: true,
            blocker_roots: Vec::new(),
            admitted_root_count: self.admitted_roots.len() as u64,
            quarantined_root_count: self.quarantined_roots.len() as u64,
            state_root: self.state_root(),
            public_record_root: self.public_record_root(),
        }
    }

    pub fn public_record(&self) -> Value {
        let mut record = self.public_record_without_state_root();
        match &mut record {
            Value::Object(fields) => {
                fields.insert("state_root".to_string(), Value::String(self.state_root()));
            }
            _ => {}
        }
        record
    }

    pub fn state_root(&self) -> String {
        root_for("state", &self.public_record_without_state_root())
    }

    pub fn public_record_root(&self) -> String {
        root_for("public-record", &self.public_record_without_state_root())
    }

    fn public_record_without_state_root(&self) -> Value {
        json!({
            "kind": "wave93_bridge_custody_receipt_root_admission_quarantine",
            "config": self.config.public_record(),
            "admitted_root_count": self.admitted_roots.len() as u64,
            "admitted_roots_root": merkle_of_roots(
                "wave93-bridge-custody-admitted-roots",
                self.admitted_roots.values().map(|item| item.admission_root.clone()).collect(),
            ),
            "quarantined_root_count": self.quarantined_roots.len() as u64,
            "quarantined_roots_root": merkle_of_roots(
                "wave93-bridge-custody-quarantined-roots",
                self.quarantined_roots.values().map(|item| item.quarantine_root.clone()).collect(),
            ),
            "admission_rule_root": merkle_of_roots(
                "wave93-bridge-custody-admission-rules",
                self.admission_rules.iter().map(AdmissionRule::root).collect(),
            ),
            "operator_command_root": merkle_of_roots(
                "wave93-bridge-custody-operator-commands",
                self.operator_commands.iter().map(OperatorCommand::root).collect(),
            ),
            "last_verdict": self.last_verdict.public_record(),
            "roots_only_public_records": true,
            "live_receipt": false,
            "heavy_gates_ran": false,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FutureReceiptRootCandidate {
    pub receipt_root: String,
    pub wave92_slot_root: String,
    pub watcher_quorum_root: String,
    pub withdrawal_release_root: String,
    pub reserve_coverage_root: String,
    pub signer_quorum_root: String,
    pub challenge_hold_review_root: String,
    pub custody_operator_signoff_root: String,
    pub watcher_votes: u16,
    pub signer_votes: u16,
    pub operator_signoffs: u16,
    pub reserve_coverage_bps: u16,
    pub challenge_hold_blocks_elapsed: u64,
    pub challenge_hold_clear: bool,
}

impl FutureReceiptRootCandidate {
    pub fn validate(&self) -> Result<()> {
        ensure_root_like("receipt_root", &self.receipt_root)?;
        ensure_root_like("wave92_slot_root", &self.wave92_slot_root)?;
        ensure_root_like("watcher_quorum_root", &self.watcher_quorum_root)?;
        ensure_root_like("withdrawal_release_root", &self.withdrawal_release_root)?;
        ensure_root_like("reserve_coverage_root", &self.reserve_coverage_root)?;
        ensure_root_like("signer_quorum_root", &self.signer_quorum_root)?;
        ensure_root_like(
            "challenge_hold_review_root",
            &self.challenge_hold_review_root,
        )?;
        ensure_root_like(
            "custody_operator_signoff_root",
            &self.custody_operator_signoff_root,
        )?;
        Ok(())
    }

    pub fn blockers(&self, config: &Config) -> Vec<QuarantineReason> {
        let mut reasons = Vec::new();
        if self.wave92_slot_root != config.wave92_slot_root {
            reasons.push(QuarantineReason::Wave92SlotRootMismatch);
        }
        if self.watcher_votes < config.min_watcher_quorum {
            reasons.push(QuarantineReason::MoneroWatcherQuorumMissing);
        }
        if self.withdrawal_release_root == EMPTY_ROOT {
            reasons.push(QuarantineReason::WithdrawalReleaseMissing);
        }
        if self.reserve_coverage_root == EMPTY_ROOT
            || self.reserve_coverage_bps < config.min_reserve_coverage_bps
        {
            reasons.push(QuarantineReason::ReserveCoverageMissing);
        }
        if self.signer_votes < config.min_signer_quorum {
            reasons.push(QuarantineReason::SignerQuorumMissing);
        }
        if self.challenge_hold_review_root == EMPTY_ROOT
            || !self.challenge_hold_clear
            || self.challenge_hold_blocks_elapsed < config.challenge_hold_blocks
        {
            reasons.push(QuarantineReason::ChallengeHoldReviewOpen);
        }
        if self.operator_signoffs < config.min_operator_signoff {
            reasons.push(QuarantineReason::CustodyOperatorSignoffMissing);
        }
        if !config.live_receipts_enabled {
            reasons.push(QuarantineReason::FutureLiveReceiptDisabled);
        }
        if config.heavy_gates_ran {
            reasons.push(QuarantineReason::HeavyGateClaimRejected);
        }
        reasons
    }

    pub fn root(&self) -> String {
        root_for("candidate", &self.public_record())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "receipt_root": self.receipt_root,
            "wave92_slot_root": self.wave92_slot_root,
            "watcher_quorum_root": self.watcher_quorum_root,
            "withdrawal_release_root": self.withdrawal_release_root,
            "reserve_coverage_root": self.reserve_coverage_root,
            "signer_quorum_root": self.signer_quorum_root,
            "challenge_hold_review_root": self.challenge_hold_review_root,
            "custody_operator_signoff_root": self.custody_operator_signoff_root,
            "watcher_votes": self.watcher_votes,
            "signer_votes": self.signer_votes,
            "operator_signoffs": self.operator_signoffs,
            "reserve_coverage_bps": self.reserve_coverage_bps,
            "challenge_hold_blocks_elapsed": self.challenge_hold_blocks_elapsed,
            "challenge_hold_clear": self.challenge_hold_clear,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReceiptRootAdmission {
    pub admission_root: String,
    pub receipt_root: String,
    pub custody_lane_root: String,
    pub admitted_by_rule_root: String,
}

impl ReceiptRootAdmission {
    pub fn from_candidate(config: &Config, candidate: FutureReceiptRootCandidate) -> Self {
        let custody_lane_root = root_for("custody-lane", &candidate.public_record());
        let admitted_by_rule_root = root_for("admission-rule-set", &config.public_record());
        let admission_root = domain_hash(
            "WAVE93-BRIDGE-CUSTODY-ADMISSION",
            &[
                HashPart::Str(&candidate.receipt_root),
                HashPart::Str(&custody_lane_root),
                HashPart::Str(&admitted_by_rule_root),
            ],
            32,
        );
        Self {
            admission_root: format!("root:wave93:{admission_root}"),
            receipt_root: candidate.receipt_root,
            custody_lane_root,
            admitted_by_rule_root,
        }
    }

    pub fn validate(&self, config: &Config) -> Result<()> {
        ensure_root_like("admission_root", &self.admission_root)?;
        ensure_root_like("receipt_root", &self.receipt_root)?;
        ensure_root_like("custody_lane_root", &self.custody_lane_root)?;
        ensure_root_like("admitted_by_rule_root", &self.admitted_by_rule_root)?;
        if !config.live_receipts_enabled {
            return Err("admission cannot be live while lane is disabled".to_string());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuarantinedReceiptRoot {
    pub quarantine_root: String,
    pub receipt_root: String,
    pub candidate_root: String,
    pub reason_roots: Vec<String>,
}

impl QuarantinedReceiptRoot {
    pub fn from_candidate(
        candidate: FutureReceiptRootCandidate,
        reasons: Vec<QuarantineReason>,
    ) -> Self {
        let reason_roots = reasons
            .iter()
            .map(QuarantineReason::root)
            .collect::<Vec<String>>();
        let candidate_root = candidate.root();
        let reason_root = merkle_of_roots(
            "wave93-bridge-custody-quarantine-reasons",
            reason_roots.clone(),
        );
        let quarantine_hash = domain_hash(
            "WAVE93-BRIDGE-CUSTODY-QUARANTINE",
            &[HashPart::Str(&candidate_root), HashPart::Str(&reason_root)],
            32,
        );
        Self {
            quarantine_root: format!("root:wave93:{quarantine_hash}"),
            receipt_root: candidate.receipt_root,
            candidate_root,
            reason_roots,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root_like("quarantine_root", &self.quarantine_root)?;
        ensure_root_like("receipt_root", &self.receipt_root)?;
        ensure_root_like("candidate_root", &self.candidate_root)?;
        if self.reason_roots.is_empty() {
            return Err("quarantine requires at least one reason root".to_string());
        }
        for root in &self.reason_roots {
            ensure_root_like("reason_root", root)?;
        }
        Ok(())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "quarantine_root": self.quarantine_root,
            "receipt_root": self.receipt_root,
            "candidate_root": self.candidate_root,
            "reason_root": merkle_of_roots(
                "wave93-bridge-custody-quarantine-reasons",
                self.reason_roots.clone(),
            ),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum QuarantineReason {
    Wave92SlotRootMismatch,
    MoneroWatcherQuorumMissing,
    WithdrawalReleaseMissing,
    ReserveCoverageMissing,
    SignerQuorumMissing,
    ChallengeHoldReviewOpen,
    CustodyOperatorSignoffMissing,
    FutureLiveReceiptDisabled,
    HeavyGateClaimRejected,
}

impl QuarantineReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Wave92SlotRootMismatch => "wave92_slot_root_mismatch",
            Self::MoneroWatcherQuorumMissing => "monero_watcher_quorum_missing",
            Self::WithdrawalReleaseMissing => "withdrawal_release_missing",
            Self::ReserveCoverageMissing => "reserve_coverage_missing",
            Self::SignerQuorumMissing => "signer_quorum_missing",
            Self::ChallengeHoldReviewOpen => "challenge_hold_review_open",
            Self::CustodyOperatorSignoffMissing => "custody_operator_signoff_missing",
            Self::FutureLiveReceiptDisabled => "future_live_receipt_disabled",
            Self::HeavyGateClaimRejected => "heavy_gate_claim_rejected",
        }
    }

    pub fn root(&self) -> String {
        let hash = domain_hash(
            "WAVE93-BRIDGE-CUSTODY-QUARANTINE-REASON",
            &[HashPart::Str(self.as_str())],
            32,
        );
        format!("root:wave93:{hash}")
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdmissionRule {
    pub rule_id: String,
    pub domain: String,
    pub required_root_field: String,
    pub quorum_floor: u16,
    pub fail_closed: bool,
}

impl AdmissionRule {
    pub fn canonical_rules(config: &Config) -> Vec<Self> {
        vec![
            Self::rule(
                "monero_watcher_quorum",
                "monero-watcher-quorum",
                "watcher_quorum_root",
                config.min_watcher_quorum,
            ),
            Self::rule(
                "withdrawal_release",
                "withdrawal-release",
                "withdrawal_release_root",
                1,
            ),
            Self::rule(
                "reserve_coverage",
                "reserve-coverage",
                "reserve_coverage_root",
                config.min_reserve_coverage_bps,
            ),
            Self::rule(
                "signer_quorum",
                "signer-quorum",
                "signer_quorum_root",
                config.min_signer_quorum,
            ),
            Self::rule(
                "challenge_hold_review",
                "challenge-hold-review",
                "challenge_hold_review_root",
                1,
            ),
            Self::rule(
                "custody_operator_signoff",
                "custody-operator-signoff",
                "custody_operator_signoff_root",
                config.min_operator_signoff,
            ),
        ]
    }

    fn rule(rule_id: &str, domain: &str, required_root_field: &str, quorum_floor: u16) -> Self {
        Self {
            rule_id: rule_id.to_string(),
            domain: domain.to_string(),
            required_root_field: required_root_field.to_string(),
            quorum_floor,
            fail_closed: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure_text("rule_id", &self.rule_id)?;
        ensure_text("domain", &self.domain)?;
        ensure_text("required_root_field", &self.required_root_field)?;
        if !self.fail_closed {
            return Err("admission rules must fail closed".to_string());
        }
        Ok(())
    }

    pub fn root(&self) -> String {
        root_for("admission-rule", &self.public_record())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "rule_id": self.rule_id,
            "domain": self.domain,
            "required_root_field": self.required_root_field,
            "quorum_floor": self.quorum_floor,
            "fail_closed": self.fail_closed,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorCommand {
    pub command_root: String,
    pub command: OperatorCommandKind,
    pub applied: bool,
}

impl OperatorCommand {
    pub fn canonical_commands() -> Vec<Self> {
        OperatorCommandKind::all()
            .into_iter()
            .map(|command| {
                let command_hash = domain_hash(
                    "WAVE93-BRIDGE-CUSTODY-OPERATOR-COMMAND",
                    &[HashPart::Str(command.as_str())],
                    32,
                );
                Self {
                    command_root: format!("root:wave93:{command_hash}"),
                    command,
                    applied: false,
                }
            })
            .collect()
    }

    pub fn validate(&self) -> Result<()> {
        ensure_root_like("command_root", &self.command_root)?;
        Ok(())
    }

    pub fn root(&self) -> String {
        root_for("operator-command", &self.public_record())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "command_root": self.command_root,
            "command": self.command.as_str(),
            "applied": self.applied,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperatorCommandKind {
    KeepAdmissionClosed,
    QuarantineFutureReceiptRoots,
    ReviewWatcherQuorum,
    ReviewWithdrawalRelease,
    ReviewReserveCoverage,
    ReviewSignerQuorum,
    ReviewChallengeHold,
    ReviewCustodyOperatorSignoff,
}

impl OperatorCommandKind {
    pub fn all() -> Vec<Self> {
        vec![
            Self::KeepAdmissionClosed,
            Self::QuarantineFutureReceiptRoots,
            Self::ReviewWatcherQuorum,
            Self::ReviewWithdrawalRelease,
            Self::ReviewReserveCoverage,
            Self::ReviewSignerQuorum,
            Self::ReviewChallengeHold,
            Self::ReviewCustodyOperatorSignoff,
        ]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::KeepAdmissionClosed => "keep_admission_closed",
            Self::QuarantineFutureReceiptRoots => "quarantine_future_receipt_roots",
            Self::ReviewWatcherQuorum => "review_watcher_quorum",
            Self::ReviewWithdrawalRelease => "review_withdrawal_release",
            Self::ReviewReserveCoverage => "review_reserve_coverage",
            Self::ReviewSignerQuorum => "review_signer_quorum",
            Self::ReviewChallengeHold => "review_challenge_hold",
            Self::ReviewCustodyOperatorSignoff => "review_custody_operator_signoff",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdmissionVerdict {
    pub status: AdmissionStatus,
    pub clearable: bool,
    pub blocker_roots: Vec<String>,
    pub admitted_root_count: u64,
    pub quarantined_root_count: u64,
    pub state_root: String,
    pub public_record_root: String,
}

impl AdmissionVerdict {
    pub fn fail_closed(reason: impl Into<String>) -> Self {
        let reason = reason.into();
        let reason_hash = domain_hash(
            "WAVE93-BRIDGE-CUSTODY-FAIL-CLOSED-REASON",
            &[HashPart::Str(&reason)],
            32,
        );
        Self {
            status: AdmissionStatus::FailClosed,
            clearable: false,
            blocker_roots: vec![format!("root:wave93:{reason_hash}")],
            admitted_root_count: 0,
            quarantined_root_count: 0,
            state_root: EMPTY_ROOT.to_string(),
            public_record_root: EMPTY_ROOT.to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "status": self.status.as_str(),
            "clearable": self.clearable,
            "blocker_root": merkle_of_roots(
                "wave93-bridge-custody-verdict-blockers",
                self.blocker_roots.clone(),
            ),
            "admitted_root_count": self.admitted_root_count,
            "quarantined_root_count": self.quarantined_root_count,
            "state_root": self.state_root,
            "public_record_root": self.public_record_root,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdmissionStatus {
    FailClosed,
    Admitted,
}

impl AdmissionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FailClosed => "fail_closed",
            Self::Admitted => "admitted",
        }
    }
}

pub fn devnet() -> Result<Runtime> {
    State::devnet()
}

pub fn public_record() -> Value {
    match devnet() {
        Ok(state) => state.public_record(),
        Err(reason) => json!({
            "kind": "wave93_bridge_custody_receipt_root_admission_quarantine",
            "status": "fail_closed",
            "error_root": root_for("devnet-error", &json!({ "reason": reason })),
            "live_receipt": false,
            "heavy_gates_ran": false,
        }),
    }
}

pub fn state_root() -> String {
    match devnet() {
        Ok(state) => state.state_root(),
        Err(reason) => root_for("devnet-error-state", &json!({ "reason": reason })),
    }
}

fn root_for(domain: &str, value: &Value) -> String {
    let hash = domain_hash(
        &format!("WAVE93-BRIDGE-CUSTODY-{domain}"),
        &[HashPart::Json(value)],
        32,
    );
    format!("root:wave93:{hash}")
}

fn merkle_of_roots(domain: &str, roots: Vec<String>) -> String {
    let leaves = roots.into_iter().map(Value::String).collect::<Vec<Value>>();
    format!("root:wave93:{}", merkle_root(domain, &leaves))
}

fn ensure_text(field: &'static str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(format!("{field} must not be empty"));
    }
    Ok(())
}

fn ensure_root_like(field: &'static str, value: &str) -> Result<()> {
    ensure_text(field, value)?;
    if !(value.starts_with("root:") || value.len() >= 32) {
        return Err(format!("{field} must be a root commitment"));
    }
    Ok(())
}
