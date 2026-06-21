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
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave93-live-heavy-gate-receipt-admission-quarantine-wallet-watchtower-lane-runtime-v1";
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const WAVE_LABEL: &str = "wave93";
pub const SOURCE_WAVE_LABEL: &str = "wave92";
pub const SOURCE_LANE: &str =
    "force-exit-live-heavy-gate-receipt-slot-registry-wallet-watchtower-lane";
pub const ADMISSION_LANE: &str =
    "force-exit-live-heavy-gate-receipt-admission-quarantine-wallet-watchtower-lane";
pub const EMPTY_ROOT_MARKER: &str = "empty-wave93-future-receipt-root";
pub const DEFAULT_MIN_WATCHTOWER_QUORUM_ROOTS: u64 = 4;
pub const DEFAULT_MIN_OPERATOR_SIGNOFF_ROOTS: u64 = 3;
pub const DEFAULT_MIN_USER_REPLAY_ROOTS: u64 = 2;
pub const DEFAULT_ADMISSION_EPOCH: u64 = 93;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub wave_label: String,
    pub source_wave_label: String,
    pub source_lane: String,
    pub admission_lane: String,
    pub empty_root_marker: String,
    pub admission_epoch: u64,
    pub min_watchtower_quorum_roots: u64,
    pub min_operator_signoff_roots: u64,
    pub min_user_replay_roots: u64,
    pub require_wallet_escape_dry_run: bool,
    pub require_watchtower_quorum: bool,
    pub require_user_runbook_replay: bool,
    pub require_redacted_recovery_proof: bool,
    pub require_wallet_visible_receipt: bool,
    pub require_operator_signoff: bool,
    pub require_roots_only_public_record: bool,
    pub fail_closed_on_empty_future_roots: bool,
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
            source_lane: SOURCE_LANE.to_string(),
            admission_lane: ADMISSION_LANE.to_string(),
            empty_root_marker: EMPTY_ROOT_MARKER.to_string(),
            admission_epoch: DEFAULT_ADMISSION_EPOCH,
            min_watchtower_quorum_roots: DEFAULT_MIN_WATCHTOWER_QUORUM_ROOTS,
            min_operator_signoff_roots: DEFAULT_MIN_OPERATOR_SIGNOFF_ROOTS,
            min_user_replay_roots: DEFAULT_MIN_USER_REPLAY_ROOTS,
            require_wallet_escape_dry_run: true,
            require_watchtower_quorum: true,
            require_user_runbook_replay: true,
            require_redacted_recovery_proof: true,
            require_wallet_visible_receipt: true,
            require_operator_signoff: true,
            require_roots_only_public_record: true,
            fail_closed_on_empty_future_roots: true,
            heavy_gate_execution_allowed: false,
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
            "admission_lane": self.admission_lane,
            "empty_root_marker": self.empty_root_marker,
            "admission_epoch": self.admission_epoch,
            "min_watchtower_quorum_roots": self.min_watchtower_quorum_roots,
            "min_operator_signoff_roots": self.min_operator_signoff_roots,
            "min_user_replay_roots": self.min_user_replay_roots,
            "require_wallet_escape_dry_run": self.require_wallet_escape_dry_run,
            "require_watchtower_quorum": self.require_watchtower_quorum,
            "require_user_runbook_replay": self.require_user_runbook_replay,
            "require_redacted_recovery_proof": self.require_redacted_recovery_proof,
            "require_wallet_visible_receipt": self.require_wallet_visible_receipt,
            "require_operator_signoff": self.require_operator_signoff,
            "require_roots_only_public_record": self.require_roots_only_public_record,
            "fail_closed_on_empty_future_roots": self.fail_closed_on_empty_future_roots,
            "heavy_gate_execution_allowed": self.heavy_gate_execution_allowed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneKind {
    WalletEscapeDryRun,
    WatchtowerQuorum,
    UserRunbookReplay,
    RedactedRecoveryProof,
    WalletVisibleReceipt,
    OperatorSignoff,
}

impl LaneKind {
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

    pub fn minimum_roots(self, config: &Config) -> u64 {
        match self {
            Self::WatchtowerQuorum => config.min_watchtower_quorum_roots,
            Self::UserRunbookReplay => config.min_user_replay_roots,
            Self::OperatorSignoff => config.min_operator_signoff_roots,
            Self::WalletEscapeDryRun | Self::RedactedRecoveryProof | Self::WalletVisibleReceipt => {
                1
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdmissionStatus {
    Quarantined,
    Admissible,
    Denied,
}

impl AdmissionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Quarantined => "quarantined",
            Self::Admissible => "admissible",
            Self::Denied => "denied",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdmissionRuleKind {
    FutureReceiptRootPresent,
    RootShape,
    SourceSlotBinding,
    RootsOnlyPublicRecord,
    WalletEscapeDryRun,
    WatchtowerQuorum,
    UserRunbookReplay,
    RedactedRecoveryProof,
    WalletVisibleReceipt,
    OperatorSignoff,
    HeavyGateNotRun,
}

impl AdmissionRuleKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FutureReceiptRootPresent => "future_receipt_root_present",
            Self::RootShape => "root_shape",
            Self::SourceSlotBinding => "source_slot_binding",
            Self::RootsOnlyPublicRecord => "roots_only_public_record",
            Self::WalletEscapeDryRun => "wallet_escape_dry_run",
            Self::WatchtowerQuorum => "watchtower_quorum",
            Self::UserRunbookReplay => "user_runbook_replay",
            Self::RedactedRecoveryProof => "redacted_recovery_proof",
            Self::WalletVisibleReceipt => "wallet_visible_receipt",
            Self::OperatorSignoff => "operator_signoff",
            Self::HeavyGateNotRun => "heavy_gate_not_run",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QuarantineReason {
    EmptyFutureReceiptRoot,
    RootShapeInvalid,
    MissingWalletEscapeDryRunRoot,
    MissingWatchtowerQuorumRoot,
    MissingUserRunbookReplayRoot,
    MissingRedactedRecoveryProofRoot,
    MissingWalletVisibleReceiptRoot,
    MissingOperatorSignoffRoot,
    RootsOnlyRuleUnclear,
    HeavyGateExecutionClaim,
}

impl QuarantineReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EmptyFutureReceiptRoot => "empty_future_receipt_root",
            Self::RootShapeInvalid => "root_shape_invalid",
            Self::MissingWalletEscapeDryRunRoot => "missing_wallet_escape_dry_run_root",
            Self::MissingWatchtowerQuorumRoot => "missing_watchtower_quorum_root",
            Self::MissingUserRunbookReplayRoot => "missing_user_runbook_replay_root",
            Self::MissingRedactedRecoveryProofRoot => "missing_redacted_recovery_proof_root",
            Self::MissingWalletVisibleReceiptRoot => "missing_wallet_visible_receipt_root",
            Self::MissingOperatorSignoffRoot => "missing_operator_signoff_root",
            Self::RootsOnlyRuleUnclear => "roots_only_rule_unclear",
            Self::HeavyGateExecutionClaim => "heavy_gate_execution_claim",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorCommandKind {
    HoldQuarantine,
    RequestWalletEscapeDryRunRoot,
    RequestWatchtowerQuorumRoot,
    RequestUserRunbookReplayRoot,
    RequestRedactedRecoveryProofRoot,
    RequestWalletVisibleReceiptRoot,
    RequestOperatorSignoffRoot,
    AdmitFutureReceiptRoot,
}

impl OperatorCommandKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HoldQuarantine => "hold_quarantine",
            Self::RequestWalletEscapeDryRunRoot => "request_wallet_escape_dry_run_root",
            Self::RequestWatchtowerQuorumRoot => "request_watchtower_quorum_root",
            Self::RequestUserRunbookReplayRoot => "request_user_runbook_replay_root",
            Self::RequestRedactedRecoveryProofRoot => "request_redacted_recovery_proof_root",
            Self::RequestWalletVisibleReceiptRoot => "request_wallet_visible_receipt_root",
            Self::RequestOperatorSignoffRoot => "request_operator_signoff_root",
            Self::AdmitFutureReceiptRoot => "admit_future_receipt_root",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdmissionRule {
    pub kind: AdmissionRuleKind,
    pub satisfied: bool,
    pub rule_root: String,
    pub quarantine_reason: Option<QuarantineReason>,
}

impl AdmissionRule {
    pub fn new(
        kind: AdmissionRuleKind,
        satisfied: bool,
        quarantine_reason: Option<QuarantineReason>,
        rule_subject_root: &str,
    ) -> Self {
        let reason = match quarantine_reason {
            Some(value) => value.as_str(),
            None => "none",
        };
        let rule_root = record_root(
            "admission-rule",
            &json!({
                "kind": kind.as_str(),
                "satisfied": satisfied,
                "reason": reason,
                "rule_subject_root": rule_subject_root,
            }),
        );
        Self {
            kind,
            satisfied,
            rule_root,
            quarantine_reason,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": self.kind.as_str(),
            "satisfied": self.satisfied,
            "rule_root": self.rule_root,
            "quarantine_reason": self.quarantine_reason.map(|reason| reason.as_str()),
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OperatorCommand {
    pub kind: OperatorCommandKind,
    pub command_root: String,
    pub lane_root: String,
}

impl OperatorCommand {
    pub fn new(kind: OperatorCommandKind, lane_root: &str) -> Self {
        let command_root = record_root(
            "operator-command",
            &json!({
                "kind": kind.as_str(),
                "lane_root": lane_root,
            }),
        );
        Self {
            kind,
            command_root,
            lane_root: lane_root.to_string(),
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "kind": self.kind.as_str(),
            "command_root": self.command_root,
            "lane_root": self.lane_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LaneReceipt {
    pub lane_kind: LaneKind,
    pub future_receipt_root: String,
    pub source_slot_root: String,
    pub admission_input_root: String,
    pub lane_evidence_roots: Vec<String>,
    pub rules: Vec<AdmissionRule>,
    pub quarantine_reasons: Vec<QuarantineReason>,
    pub operator_command: OperatorCommand,
    pub status: AdmissionStatus,
    pub heavy_gate_ran: bool,
}

impl LaneReceipt {
    pub fn quarantined_empty(lane_kind: LaneKind, config: &Config) -> Self {
        let future_receipt_root = empty_root(lane_kind.as_str());
        let source_slot_root = empty_root(&format!("{}-source-slot", lane_kind.as_str()));
        let admission_input_root = admission_input_root(
            lane_kind,
            &future_receipt_root,
            &source_slot_root,
            &Vec::new(),
        );
        Self::from_roots(
            lane_kind,
            future_receipt_root,
            source_slot_root,
            admission_input_root,
            Vec::new(),
            false,
            config,
        )
    }

    pub fn from_roots(
        lane_kind: LaneKind,
        future_receipt_root: impl Into<String>,
        source_slot_root: impl Into<String>,
        admission_input_root: impl Into<String>,
        lane_evidence_roots: Vec<String>,
        heavy_gate_ran: bool,
        config: &Config,
    ) -> Self {
        let future_receipt_root = future_receipt_root.into();
        let source_slot_root = source_slot_root.into();
        let admission_input_root = admission_input_root.into();
        let mut lane = Self {
            lane_kind,
            future_receipt_root,
            source_slot_root,
            admission_input_root,
            lane_evidence_roots,
            rules: Vec::new(),
            quarantine_reasons: Vec::new(),
            operator_command: OperatorCommand::new(
                OperatorCommandKind::HoldQuarantine,
                &empty_root("pending-lane-root"),
            ),
            status: AdmissionStatus::Quarantined,
            heavy_gate_ran,
        };
        lane.recompute(config);
        lane
    }

    pub fn recompute(&mut self, config: &Config) {
        self.lane_evidence_roots.sort();
        self.lane_evidence_roots.dedup();
        self.rules = self.derive_rules(config);
        self.quarantine_reasons = self
            .rules
            .iter()
            .filter_map(|rule| rule.quarantine_reason)
            .collect::<Vec<_>>();
        self.quarantine_reasons.sort();
        self.quarantine_reasons.dedup();
        self.status = if self.quarantine_reasons.is_empty() {
            AdmissionStatus::Admissible
        } else if self.heavy_gate_ran {
            AdmissionStatus::Denied
        } else {
            AdmissionStatus::Quarantined
        };
        let lane_root = self.lane_root_without_command();
        self.operator_command = OperatorCommand::new(self.next_command_kind(config), &lane_root);
    }

    fn derive_rules(&self, config: &Config) -> Vec<AdmissionRule> {
        let lane_root = self.admission_input_root.clone();
        let root_present = !is_empty_root(&self.future_receipt_root);
        let source_present = !is_empty_root(&self.source_slot_root);
        let root_shape = is_root_like(&self.future_receipt_root)
            && is_root_like(&self.source_slot_root)
            && self
                .lane_evidence_roots
                .iter()
                .all(|root| is_root_like(root));
        let minimum_roots_met =
            self.live_evidence_root_count() >= self.lane_kind.minimum_roots(config);
        let lane_reason = missing_lane_reason(self.lane_kind);
        vec![
            AdmissionRule::new(
                AdmissionRuleKind::FutureReceiptRootPresent,
                root_present,
                if root_present {
                    None
                } else {
                    Some(QuarantineReason::EmptyFutureReceiptRoot)
                },
                &lane_root,
            ),
            AdmissionRule::new(
                AdmissionRuleKind::RootShape,
                root_shape,
                if root_shape {
                    None
                } else {
                    Some(QuarantineReason::RootShapeInvalid)
                },
                &lane_root,
            ),
            AdmissionRule::new(
                AdmissionRuleKind::SourceSlotBinding,
                source_present,
                if source_present {
                    None
                } else {
                    Some(QuarantineReason::EmptyFutureReceiptRoot)
                },
                &lane_root,
            ),
            AdmissionRule::new(
                AdmissionRuleKind::RootsOnlyPublicRecord,
                config.require_roots_only_public_record,
                if config.require_roots_only_public_record {
                    None
                } else {
                    Some(QuarantineReason::RootsOnlyRuleUnclear)
                },
                &lane_root,
            ),
            AdmissionRule::new(
                rule_kind_for_lane(self.lane_kind),
                minimum_roots_met,
                if minimum_roots_met {
                    None
                } else {
                    Some(lane_reason)
                },
                &lane_root,
            ),
            AdmissionRule::new(
                AdmissionRuleKind::HeavyGateNotRun,
                !self.heavy_gate_ran && !config.heavy_gate_execution_allowed,
                if !self.heavy_gate_ran && !config.heavy_gate_execution_allowed {
                    None
                } else {
                    Some(QuarantineReason::HeavyGateExecutionClaim)
                },
                &lane_root,
            ),
        ]
    }

    fn next_command_kind(&self, config: &Config) -> OperatorCommandKind {
        if self.status == AdmissionStatus::Admissible {
            return OperatorCommandKind::AdmitFutureReceiptRoot;
        }
        if self.heavy_gate_ran || self.future_receipt_root.is_empty() {
            return OperatorCommandKind::HoldQuarantine;
        }
        if self.live_evidence_root_count() < self.lane_kind.minimum_roots(config) {
            match self.lane_kind {
                LaneKind::WalletEscapeDryRun => OperatorCommandKind::RequestWalletEscapeDryRunRoot,
                LaneKind::WatchtowerQuorum => OperatorCommandKind::RequestWatchtowerQuorumRoot,
                LaneKind::UserRunbookReplay => OperatorCommandKind::RequestUserRunbookReplayRoot,
                LaneKind::RedactedRecoveryProof => {
                    OperatorCommandKind::RequestRedactedRecoveryProofRoot
                }
                LaneKind::WalletVisibleReceipt => {
                    OperatorCommandKind::RequestWalletVisibleReceiptRoot
                }
                LaneKind::OperatorSignoff => OperatorCommandKind::RequestOperatorSignoffRoot,
            }
        } else {
            OperatorCommandKind::HoldQuarantine
        }
    }

    pub fn live_evidence_root_count(&self) -> u64 {
        self.lane_evidence_roots
            .iter()
            .filter(|root| !is_empty_root(root))
            .count() as u64
    }

    pub fn lane_root_without_command(&self) -> String {
        record_root(
            "lane-receipt",
            &self.public_record_without_root_or_command(),
        )
    }

    pub fn public_record_without_root_or_command(&self) -> PublicRecord {
        json!({
            "lane_kind": self.lane_kind.as_str(),
            "future_receipt_root": self.future_receipt_root,
            "source_slot_root": self.source_slot_root,
            "admission_input_root": self.admission_input_root,
            "lane_evidence_roots": self.lane_evidence_roots,
            "rules": self.rules.iter().map(AdmissionRule::public_record).collect::<Vec<_>>(),
            "quarantine_reasons": self.quarantine_reasons.iter().map(|reason| reason.as_str()).collect::<Vec<_>>(),
            "status": self.status.as_str(),
            "heavy_gate_ran": self.heavy_gate_ran,
            "live_evidence_root_count": self.live_evidence_root_count(),
        })
    }

    pub fn public_record(&self) -> PublicRecord {
        let mut record = self.public_record_without_root_or_command();
        if let Some(map) = record.as_object_mut() {
            map.insert(
                "operator_command".to_string(),
                self.operator_command.public_record(),
            );
            map.insert(
                "lane_root".to_string(),
                Value::String(self.lane_root_without_command()),
            );
        }
        record
    }

    pub fn state_root(&self) -> String {
        self.lane_root_without_command()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdmissionSummary {
    pub status: AdmissionStatus,
    pub fail_closed: bool,
    pub admissible_lanes: u64,
    pub quarantined_lanes: u64,
    pub denied_lanes: u64,
    pub quarantine_root: String,
    pub command_root: String,
    pub admitted_root: String,
    pub heavy_gate_ran: bool,
}

impl AdmissionSummary {
    pub fn from_lanes(lanes: &BTreeMap<String, LaneReceipt>) -> Self {
        let admissible_lanes = count_status(lanes, AdmissionStatus::Admissible);
        let quarantined_lanes = count_status(lanes, AdmissionStatus::Quarantined);
        let denied_lanes = count_status(lanes, AdmissionStatus::Denied);
        let heavy_gate_ran = lanes.values().any(|lane| lane.heavy_gate_ran);
        let status = if denied_lanes > 0 {
            AdmissionStatus::Denied
        } else if quarantined_lanes > 0 || lanes.is_empty() {
            AdmissionStatus::Quarantined
        } else {
            AdmissionStatus::Admissible
        };
        let quarantine_root = merkle_root(
            "WAVE93-WALLET-WATCHTOWER-QUARANTINE-REASONS",
            &lanes
                .values()
                .flat_map(|lane| {
                    lane.quarantine_reasons.iter().map(|reason| {
                        json!({
                            "lane_kind": lane.lane_kind.as_str(),
                            "reason": reason.as_str(),
                            "lane_root": lane.state_root(),
                        })
                    })
                })
                .collect::<Vec<_>>(),
        );
        let command_root = merkle_root(
            "WAVE93-WALLET-WATCHTOWER-OPERATOR-COMMANDS",
            &lanes
                .values()
                .map(|lane| lane.operator_command.public_record())
                .collect::<Vec<_>>(),
        );
        let admitted_root = merkle_root(
            "WAVE93-WALLET-WATCHTOWER-ADMITTED-FUTURE-RECEIPTS",
            &lanes
                .values()
                .filter(|lane| lane.status == AdmissionStatus::Admissible)
                .map(|lane| {
                    json!({
                        "lane_kind": lane.lane_kind.as_str(),
                        "future_receipt_root": lane.future_receipt_root,
                        "lane_root": lane.state_root(),
                    })
                })
                .collect::<Vec<_>>(),
        );
        Self {
            status,
            fail_closed: status != AdmissionStatus::Admissible,
            admissible_lanes,
            quarantined_lanes,
            denied_lanes,
            quarantine_root,
            command_root,
            admitted_root,
            heavy_gate_ran,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "status": self.status.as_str(),
            "fail_closed": self.fail_closed,
            "admissible_lanes": self.admissible_lanes,
            "quarantined_lanes": self.quarantined_lanes,
            "denied_lanes": self.denied_lanes,
            "quarantine_root": self.quarantine_root,
            "command_root": self.command_root,
            "admitted_root": self.admitted_root,
            "heavy_gate_ran": self.heavy_gate_ran,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("admission-summary", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub lanes: BTreeMap<String, LaneReceipt>,
    pub summary: AdmissionSummary,
}

impl Default for State {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

impl State {
    pub fn new(config: Config) -> Self {
        let lanes = LaneKind::all()
            .iter()
            .map(|lane_kind| {
                let receipt = LaneReceipt::quarantined_empty(*lane_kind, &config);
                (lane_kind.as_str().to_string(), receipt)
            })
            .collect::<BTreeMap<_, _>>();
        let summary = AdmissionSummary::from_lanes(&lanes);
        Self {
            config,
            lanes,
            summary,
        }
    }

    pub fn admit_future_receipt(
        mut self,
        lane_kind: LaneKind,
        future_receipt_root: impl Into<String>,
        source_slot_root: impl Into<String>,
        lane_evidence_roots: Vec<String>,
    ) -> Result<Self> {
        let future_receipt_root = future_receipt_root.into();
        let source_slot_root = source_slot_root.into();
        let admission_input_root = admission_input_root(
            lane_kind,
            &future_receipt_root,
            &source_slot_root,
            &lane_evidence_roots,
        );
        let receipt = LaneReceipt::from_roots(
            lane_kind,
            future_receipt_root,
            source_slot_root,
            admission_input_root,
            lane_evidence_roots,
            false,
            &self.config,
        );
        self.lanes.insert(lane_kind.as_str().to_string(), receipt);
        self.recompute();
        Ok(self)
    }

    pub fn quarantine_heavy_gate_claim(mut self, lane_kind: LaneKind) -> Result<Self> {
        let key = lane_kind.as_str().to_string();
        match self.lanes.remove(&key) {
            Some(mut lane) => {
                lane.heavy_gate_ran = true;
                lane.recompute(&self.config);
                self.lanes.insert(key, lane);
                self.recompute();
                Ok(self)
            }
            None => Err(format!("missing lane: {}", lane_kind.as_str())),
        }
    }

    pub fn recompute(&mut self) {
        for lane in self.lanes.values_mut() {
            lane.recompute(&self.config);
        }
        self.summary = AdmissionSummary::from_lanes(&self.lanes);
    }

    pub fn lane_roots(&self) -> BTreeMap<String, String> {
        self.lanes
            .iter()
            .map(|(key, lane)| (key.clone(), lane.state_root()))
            .collect::<BTreeMap<_, _>>()
    }

    pub fn lanes_root(&self) -> String {
        merkle_root(
            "WAVE93-WALLET-WATCHTOWER-LANE-ROOTS",
            &self
                .lane_roots()
                .values()
                .cloned()
                .map(Value::String)
                .collect::<Vec<_>>(),
        )
    }

    pub fn public_record_without_state_root(&self) -> PublicRecord {
        json!({
            "config": self.config.public_record(),
            "lanes": self.lanes.iter().map(|(key, lane)| (key.clone(), lane.public_record())).collect::<BTreeMap<_, _>>(),
            "lane_roots": self.lane_roots(),
            "lanes_root": self.lanes_root(),
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

pub fn empty_quarantine_runtime() -> Runtime {
    devnet()
}

pub fn admission_input_root(
    lane_kind: LaneKind,
    future_receipt_root: &str,
    source_slot_root: &str,
    lane_evidence_roots: &[String],
) -> String {
    record_root(
        "admission-input",
        &json!({
            "lane_kind": lane_kind.as_str(),
            "future_receipt_root": future_receipt_root,
            "source_slot_root": source_slot_root,
            "lane_evidence_roots": lane_evidence_roots,
            "wallet_material_absent": true,
            "heavy_gate_ran": false,
        }),
    )
}

fn count_status(lanes: &BTreeMap<String, LaneReceipt>, status: AdmissionStatus) -> u64 {
    lanes.values().filter(|lane| lane.status == status).count() as u64
}

fn rule_kind_for_lane(lane_kind: LaneKind) -> AdmissionRuleKind {
    match lane_kind {
        LaneKind::WalletEscapeDryRun => AdmissionRuleKind::WalletEscapeDryRun,
        LaneKind::WatchtowerQuorum => AdmissionRuleKind::WatchtowerQuorum,
        LaneKind::UserRunbookReplay => AdmissionRuleKind::UserRunbookReplay,
        LaneKind::RedactedRecoveryProof => AdmissionRuleKind::RedactedRecoveryProof,
        LaneKind::WalletVisibleReceipt => AdmissionRuleKind::WalletVisibleReceipt,
        LaneKind::OperatorSignoff => AdmissionRuleKind::OperatorSignoff,
    }
}

fn missing_lane_reason(lane_kind: LaneKind) -> QuarantineReason {
    match lane_kind {
        LaneKind::WalletEscapeDryRun => QuarantineReason::MissingWalletEscapeDryRunRoot,
        LaneKind::WatchtowerQuorum => QuarantineReason::MissingWatchtowerQuorumRoot,
        LaneKind::UserRunbookReplay => QuarantineReason::MissingUserRunbookReplayRoot,
        LaneKind::RedactedRecoveryProof => QuarantineReason::MissingRedactedRecoveryProofRoot,
        LaneKind::WalletVisibleReceipt => QuarantineReason::MissingWalletVisibleReceiptRoot,
        LaneKind::OperatorSignoff => QuarantineReason::MissingOperatorSignoffRoot,
    }
}

fn empty_root(label: &str) -> String {
    let root = record_root(
        "empty-future-receipt-root",
        &json!({
            "marker": EMPTY_ROOT_MARKER,
            "label": label,
        }),
    );
    format!("{EMPTY_ROOT_MARKER}:{root}")
}

fn is_empty_root(root: &str) -> bool {
    root.is_empty() || root.contains(EMPTY_ROOT_MARKER)
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
        "WAVE93-WALLET-WATCHTOWER-RECEIPT-ADMISSION-QUARANTINE",
        &[
            HashPart::Str(domain),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Json(record),
        ],
        32,
    )
}
