use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackagePqReservePrivacyLiveReceiptReleaseManifestEnforcementRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_PQ_RESERVE_PRIVACY_LIVE_RECEIPT_RELEASE_MANIFEST_ENFORCEMENT_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-pq-reserve-privacy-live-receipt-release-manifest-enforcement-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_PQ_RESERVE_PRIVACY_LIVE_RECEIPT_RELEASE_MANIFEST_ENFORCEMENT_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const ENFORCEMENT_SUITE: &str =
    "monero-l2-pq-reserve-privacy-live-receipt-release-manifest-enforcement-v1";
pub const DEFAULT_MONERO_NETWORK: &str = "monero-devnet";
pub const DEFAULT_L2_NETWORK: &str = "nebula-devnet";
pub const DEFAULT_RELEASE_MANIFEST_ID: &str =
    "force-exit-package-pq-reserve-privacy-live-receipt-release-manifest";
pub const DEFAULT_CURRENT_EPOCH: u64 = 87;
pub const DEFAULT_L2_REFERENCE_HEIGHT: u64 = 4_225_280;
pub const DEFAULT_MIN_PQ_EPOCH: u64 = 85;
pub const DEFAULT_MAX_PQ_EPOCH_DRIFT: u64 = 2;
pub const DEFAULT_MIN_RESERVE_SLO_BPS: u64 = 1_300;
pub const DEFAULT_MIN_RESERVE_ATOMIC: u64 = 16_000_000_000;
pub const DEFAULT_MIN_PRIVACY_REMAINING_BPS: u64 = 2_700;
pub const DEFAULT_MAX_LINKAGE_RISK_BPS: u64 = 75;
pub const DEFAULT_MAX_LANE_STALENESS_HEIGHTS: u64 = 18;
pub const DEFAULT_MIN_REQUIRED_LANES: u64 = 7;
pub const DEFAULT_MAX_LANES: usize = 64;

const DOMAIN: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-pq-reserve-privacy-live-receipt-release-manifest-enforcement-runtime";

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EnforcementLaneKind {
    PqEpochFreshness,
    ReserveSloRoot,
    PrivacyBudgetRoot,
    NonLinkageRoot,
    ActivationAdjudicatorRoot,
    ReleaseManifestBinding,
    ReleaseHoldExport,
    OperatorReleaseAttestation,
}

impl EnforcementLaneKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PqEpochFreshness => "pq_epoch_freshness",
            Self::ReserveSloRoot => "reserve_slo_root",
            Self::PrivacyBudgetRoot => "privacy_budget_root",
            Self::NonLinkageRoot => "non_linkage_root",
            Self::ActivationAdjudicatorRoot => "activation_adjudicator_root",
            Self::ReleaseManifestBinding => "release_manifest_binding",
            Self::ReleaseHoldExport => "release_hold_export",
            Self::OperatorReleaseAttestation => "operator_release_attestation",
        }
    }

    pub fn is_required_by_default(self) -> bool {
        !matches!(self, Self::OperatorReleaseAttestation)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneObservationStatus {
    Accepted,
    Missing,
    Stale,
    Mismatched,
    Rejected,
}

impl LaneObservationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Missing => "missing",
            Self::Stale => "stale",
            Self::Mismatched => "mismatched",
            Self::Rejected => "rejected",
        }
    }

    pub fn blocks_release(self) -> bool {
        !matches!(self, Self::Accepted)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestPolicyVerdict {
    ReleaseAllowed,
    ReleaseHeld,
    ReleaseRejected,
}

impl ManifestPolicyVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseAllowed => "release_allowed",
            Self::ReleaseHeld => "release_held",
            Self::ReleaseRejected => "release_rejected",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HoldExportReason {
    None,
    MissingLane,
    StaleLane,
    RootMismatch,
    PqEpochStale,
    ReserveSloBreach,
    PrivacyBudgetBreach,
    LinkageRiskBreach,
    ActivationAdjudicatorMismatch,
}

impl HoldExportReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::MissingLane => "missing_lane",
            Self::StaleLane => "stale_lane",
            Self::RootMismatch => "root_mismatch",
            Self::PqEpochStale => "pq_epoch_stale",
            Self::ReserveSloBreach => "reserve_slo_breach",
            Self::PrivacyBudgetBreach => "privacy_budget_breach",
            Self::LinkageRiskBreach => "linkage_risk_breach",
            Self::ActivationAdjudicatorMismatch => "activation_adjudicator_mismatch",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub enforcement_suite: String,
    pub monero_network: String,
    pub l2_network: String,
    pub release_manifest_id: String,
    pub current_epoch: u64,
    pub l2_reference_height: u64,
    pub min_pq_epoch: u64,
    pub max_pq_epoch_drift: u64,
    pub min_reserve_slo_bps: u64,
    pub min_reserve_atomic: u64,
    pub min_privacy_remaining_bps: u64,
    pub max_linkage_risk_bps: u64,
    pub max_lane_staleness_heights: u64,
    pub min_required_lanes: u64,
    pub max_lanes: usize,
    pub require_pq_epoch_freshness: bool,
    pub require_reserve_slo_root: bool,
    pub require_privacy_budget_root: bool,
    pub require_non_linkage_root: bool,
    pub require_activation_adjudicator_root: bool,
    pub require_release_hold_exports: bool,
    pub fail_closed_on_missing_lane: bool,
    pub fail_closed_on_mismatch: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            enforcement_suite: ENFORCEMENT_SUITE.to_string(),
            monero_network: DEFAULT_MONERO_NETWORK.to_string(),
            l2_network: DEFAULT_L2_NETWORK.to_string(),
            release_manifest_id: DEFAULT_RELEASE_MANIFEST_ID.to_string(),
            current_epoch: DEFAULT_CURRENT_EPOCH,
            l2_reference_height: DEFAULT_L2_REFERENCE_HEIGHT,
            min_pq_epoch: DEFAULT_MIN_PQ_EPOCH,
            max_pq_epoch_drift: DEFAULT_MAX_PQ_EPOCH_DRIFT,
            min_reserve_slo_bps: DEFAULT_MIN_RESERVE_SLO_BPS,
            min_reserve_atomic: DEFAULT_MIN_RESERVE_ATOMIC,
            min_privacy_remaining_bps: DEFAULT_MIN_PRIVACY_REMAINING_BPS,
            max_linkage_risk_bps: DEFAULT_MAX_LINKAGE_RISK_BPS,
            max_lane_staleness_heights: DEFAULT_MAX_LANE_STALENESS_HEIGHTS,
            min_required_lanes: DEFAULT_MIN_REQUIRED_LANES,
            max_lanes: DEFAULT_MAX_LANES,
            require_pq_epoch_freshness: true,
            require_reserve_slo_root: true,
            require_privacy_budget_root: true,
            require_non_linkage_root: true,
            require_activation_adjudicator_root: true,
            require_release_hold_exports: true,
            fail_closed_on_missing_lane: true,
            fail_closed_on_mismatch: true,
        }
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }

    pub fn required_lane_count(&self) -> u64 {
        [
            self.require_pq_epoch_freshness,
            self.require_reserve_slo_root,
            self.require_privacy_budget_root,
            self.require_non_linkage_root,
            self.require_activation_adjudicator_root,
            self.require_release_hold_exports,
        ]
        .iter()
        .filter(|required| **required)
        .count() as u64
    }

    pub fn validate(&self) -> Result<()> {
        require_non_empty("chain_id", &self.chain_id)?;
        require_non_empty("protocol_version", &self.protocol_version)?;
        require_non_empty("release_manifest_id", &self.release_manifest_id)?;
        require(
            self.schema_version == SCHEMA_VERSION,
            "unsupported schema_version",
        )?;
        require(
            self.current_epoch >= self.min_pq_epoch,
            "current_epoch below min_pq_epoch",
        )?;
        require(self.max_lanes > 0, "max_lanes must be nonzero")?;
        require(
            self.min_required_lanes > 0,
            "min_required_lanes must be nonzero",
        )?;
        require(
            self.min_required_lanes <= self.max_lanes as u64,
            "min_required_lanes exceeds max_lanes",
        )?;
        require(
            self.min_reserve_slo_bps <= 10_000,
            "min_reserve_slo_bps exceeds 10000",
        )?;
        require(
            self.min_privacy_remaining_bps <= 10_000,
            "min_privacy_remaining_bps exceeds 10000",
        )?;
        require(
            self.max_linkage_risk_bps <= 10_000,
            "max_linkage_risk_bps exceeds 10000",
        )
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "enforcement_suite": self.enforcement_suite,
            "monero_network": self.monero_network,
            "l2_network": self.l2_network,
            "release_manifest_id": self.release_manifest_id,
            "current_epoch": self.current_epoch,
            "l2_reference_height": self.l2_reference_height,
            "min_pq_epoch": self.min_pq_epoch,
            "max_pq_epoch_drift": self.max_pq_epoch_drift,
            "min_reserve_slo_bps": self.min_reserve_slo_bps,
            "min_reserve_atomic": self.min_reserve_atomic,
            "min_privacy_remaining_bps": self.min_privacy_remaining_bps,
            "max_linkage_risk_bps": self.max_linkage_risk_bps,
            "max_lane_staleness_heights": self.max_lane_staleness_heights,
            "min_required_lanes": self.min_required_lanes,
            "required_lane_count": self.required_lane_count(),
            "max_lanes": self.max_lanes,
            "require_pq_epoch_freshness": self.require_pq_epoch_freshness,
            "require_reserve_slo_root": self.require_reserve_slo_root,
            "require_privacy_budget_root": self.require_privacy_budget_root,
            "require_non_linkage_root": self.require_non_linkage_root,
            "require_activation_adjudicator_root": self.require_activation_adjudicator_root,
            "require_release_hold_exports": self.require_release_hold_exports,
            "fail_closed_on_missing_lane": self.fail_closed_on_missing_lane,
            "fail_closed_on_mismatch": self.fail_closed_on_mismatch,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleaseManifestPolicy {
    pub manifest_id: String,
    pub release_candidate_id: String,
    pub force_exit_package_id: String,
    pub release_policy_root: String,
    pub expected_pq_epoch_root: String,
    pub expected_reserve_slo_root: String,
    pub expected_privacy_budget_root: String,
    pub expected_non_linkage_root: String,
    pub expected_activation_adjudicator_root: String,
    pub expected_hold_export_root: String,
    pub manifest_epoch: u64,
    pub manifest_height: u64,
    pub production_release_requested: bool,
}

impl ReleaseManifestPolicy {
    pub fn public_record(&self) -> Value {
        json!(self)
    }

    pub fn state_root(&self) -> String {
        record_root("release-manifest-policy", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PqEpochFreshnessReceipt {
    pub receipt_id: String,
    pub pq_key_epoch: u64,
    pub previous_epoch_root: String,
    pub active_epoch_root: String,
    pub watcher_quorum_root: String,
    pub observed_height: u64,
    pub present: bool,
}

impl PqEpochFreshnessReceipt {
    pub fn is_fresh(&self, config: &Config) -> bool {
        self.present
            && self.pq_key_epoch >= config.min_pq_epoch
            && config.current_epoch.saturating_sub(self.pq_key_epoch) <= config.max_pq_epoch_drift
    }

    pub fn public_record(&self) -> Value {
        json!(self)
    }

    pub fn state_root(&self) -> String {
        record_root("pq-epoch-freshness-receipt", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReserveSloReceipt {
    pub receipt_id: String,
    pub reserve_snapshot_root: String,
    pub reserve_slo_root: String,
    pub reserve_attestation_root: String,
    pub reserved_atomic: u64,
    pub release_liability_atomic: u64,
    pub reserve_slo_bps: u64,
    pub observed_height: u64,
    pub present: bool,
}

impl ReserveSloReceipt {
    pub fn meets_slo(&self, config: &Config) -> bool {
        self.present
            && self.reserve_slo_bps >= config.min_reserve_slo_bps
            && self.reserved_atomic >= config.min_reserve_atomic
            && self.reserved_atomic >= self.release_liability_atomic
    }

    pub fn public_record(&self) -> Value {
        json!(self)
    }

    pub fn state_root(&self) -> String {
        record_root("reserve-slo-receipt", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PrivacyBudgetReceipt {
    pub receipt_id: String,
    pub privacy_budget_root: String,
    pub non_linkage_root: String,
    pub spend_receipt_root: String,
    pub remaining_budget_bps: u64,
    pub linkage_risk_bps: u64,
    pub observed_height: u64,
    pub present: bool,
}

impl PrivacyBudgetReceipt {
    pub fn budget_ok(&self, config: &Config) -> bool {
        self.present
            && self.remaining_budget_bps >= config.min_privacy_remaining_bps
            && self.linkage_risk_bps <= config.max_linkage_risk_bps
    }

    pub fn public_record(&self) -> Value {
        json!(self)
    }

    pub fn state_root(&self) -> String {
        record_root("privacy-budget-receipt", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ActivationAdjudicatorReceipt {
    pub receipt_id: String,
    pub adjudicator_id: String,
    pub activation_adjudicator_root: String,
    pub accepted_activation_root: String,
    pub rejected_activation_root: String,
    pub release_policy_root: String,
    pub observed_height: u64,
    pub present: bool,
    pub accepted_for_release: bool,
}

impl ActivationAdjudicatorReceipt {
    pub fn accepted(&self) -> bool {
        self.present && self.accepted_for_release
    }

    pub fn public_record(&self) -> Value {
        json!(self)
    }

    pub fn state_root(&self) -> String {
        record_root("activation-adjudicator-receipt", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LaneObservation {
    pub lane_id: String,
    pub lane_kind: EnforcementLaneKind,
    pub status: LaneObservationStatus,
    pub expected_root: String,
    pub observed_root: String,
    pub receipt_root: String,
    pub observed_height: u64,
    pub stale_by_heights: u64,
    pub required: bool,
}

impl LaneObservation {
    pub fn accepted(
        lane_kind: EnforcementLaneKind,
        expected_root: String,
        observed_root: String,
        receipt_root: String,
        observed_height: u64,
        required: bool,
    ) -> Self {
        let status = if expected_root == observed_root {
            LaneObservationStatus::Accepted
        } else {
            LaneObservationStatus::Mismatched
        };
        Self {
            lane_id: lane_id(lane_kind, &expected_root, &observed_root, observed_height),
            lane_kind,
            status,
            expected_root,
            observed_root,
            receipt_root,
            observed_height,
            stale_by_heights: 0,
            required,
        }
    }

    pub fn missing(lane_kind: EnforcementLaneKind, expected_root: String, required: bool) -> Self {
        Self {
            lane_id: lane_id(lane_kind, &expected_root, "", 0),
            lane_kind,
            status: LaneObservationStatus::Missing,
            expected_root,
            observed_root: String::new(),
            receipt_root: String::new(),
            observed_height: 0,
            stale_by_heights: 0,
            required,
        }
    }

    pub fn mark_stale(mut self, config: &Config) -> Self {
        if self.observed_height > 0 {
            self.stale_by_heights = config
                .l2_reference_height
                .saturating_sub(self.observed_height);
            if self.stale_by_heights > config.max_lane_staleness_heights {
                self.status = LaneObservationStatus::Stale;
            }
        }
        self
    }

    pub fn blocks_release(&self) -> bool {
        self.required && self.status.blocks_release()
    }

    pub fn hold_reason(&self) -> HoldExportReason {
        match self.status {
            LaneObservationStatus::Accepted => HoldExportReason::None,
            LaneObservationStatus::Missing => HoldExportReason::MissingLane,
            LaneObservationStatus::Stale => HoldExportReason::StaleLane,
            LaneObservationStatus::Mismatched => {
                if self.lane_kind == EnforcementLaneKind::ActivationAdjudicatorRoot {
                    HoldExportReason::ActivationAdjudicatorMismatch
                } else {
                    HoldExportReason::RootMismatch
                }
            }
            LaneObservationStatus::Rejected => HoldExportReason::RootMismatch,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "lane_id": self.lane_id,
            "lane_kind": self.lane_kind.as_str(),
            "status": self.status.as_str(),
            "expected_root": self.expected_root,
            "observed_root": self.observed_root,
            "receipt_root": self.receipt_root,
            "observed_height": self.observed_height,
            "stale_by_heights": self.stale_by_heights,
            "required": self.required,
            "blocks_release": self.blocks_release(),
            "hold_reason": self.hold_reason().as_str(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("lane-observation", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReleaseHoldExport {
    pub export_id: String,
    pub lane_id: String,
    pub lane_kind: EnforcementLaneKind,
    pub reason: HoldExportReason,
    pub expected_root: String,
    pub observed_root: String,
    pub export_root: String,
    pub release_manifest_id: String,
    pub exported_at_height: u64,
}

impl ReleaseHoldExport {
    pub fn from_lane(lane: &LaneObservation, manifest_id: &str, exported_at_height: u64) -> Self {
        let export_root = hold_export_root(
            lane.lane_kind,
            lane.hold_reason(),
            manifest_id,
            &lane.expected_root,
            &lane.observed_root,
        );
        Self {
            export_id: hold_export_id(lane, manifest_id),
            lane_id: lane.lane_id.clone(),
            lane_kind: lane.lane_kind,
            reason: lane.hold_reason(),
            expected_root: lane.expected_root.clone(),
            observed_root: lane.observed_root.clone(),
            export_root,
            release_manifest_id: manifest_id.to_string(),
            exported_at_height,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "export_id": self.export_id,
            "lane_id": self.lane_id,
            "lane_kind": self.lane_kind.as_str(),
            "reason": self.reason.as_str(),
            "expected_root": self.expected_root,
            "observed_root": self.observed_root,
            "export_root": self.export_root,
            "release_manifest_id": self.release_manifest_id,
            "exported_at_height": self.exported_at_height,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("release-hold-export", &self.public_record())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct EnforcementCounters {
    pub total_lanes: u64,
    pub required_lanes: u64,
    pub accepted_lanes: u64,
    pub missing_lanes: u64,
    pub stale_lanes: u64,
    pub mismatched_lanes: u64,
    pub rejected_lanes: u64,
    pub blocking_lanes: u64,
    pub hold_exports: u64,
}

impl EnforcementCounters {
    pub fn from_lanes(lanes: &[LaneObservation], hold_exports: &[ReleaseHoldExport]) -> Self {
        let mut counters = Self {
            total_lanes: lanes.len() as u64,
            required_lanes: lanes.iter().filter(|lane| lane.required).count() as u64,
            accepted_lanes: 0,
            missing_lanes: 0,
            stale_lanes: 0,
            mismatched_lanes: 0,
            rejected_lanes: 0,
            blocking_lanes: lanes.iter().filter(|lane| lane.blocks_release()).count() as u64,
            hold_exports: hold_exports.len() as u64,
        };
        for lane in lanes {
            match lane.status {
                LaneObservationStatus::Accepted => counters.accepted_lanes += 1,
                LaneObservationStatus::Missing => counters.missing_lanes += 1,
                LaneObservationStatus::Stale => counters.stale_lanes += 1,
                LaneObservationStatus::Mismatched => counters.mismatched_lanes += 1,
                LaneObservationStatus::Rejected => counters.rejected_lanes += 1,
            }
        }
        counters
    }

    pub fn public_record(&self) -> Value {
        json!(self)
    }

    pub fn state_root(&self) -> String {
        record_root("enforcement-counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EnforcementRoots {
    pub release_manifest_root: String,
    pub pq_epoch_root: String,
    pub reserve_slo_root: String,
    pub privacy_budget_root: String,
    pub non_linkage_root: String,
    pub activation_adjudicator_root: String,
    pub lane_observation_root: String,
    pub hold_export_root: String,
    pub counter_root: String,
}

impl EnforcementRoots {
    pub fn public_record(&self) -> Value {
        json!(self)
    }

    pub fn state_root(&self) -> String {
        record_root("enforcement-roots", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EnforcementRecord {
    pub enforcement_id: String,
    pub verdict: ManifestPolicyVerdict,
    pub release_manifest_id: String,
    pub release_candidate_id: String,
    pub force_exit_package_id: String,
    pub counters: EnforcementCounters,
    pub roots: EnforcementRoots,
    pub lane_observations: Vec<LaneObservation>,
    pub release_hold_exports: Vec<ReleaseHoldExport>,
    pub produced_at_epoch: u64,
    pub produced_at_height: u64,
}

impl EnforcementRecord {
    pub fn public_record(&self) -> Value {
        json!({
            "enforcement_id": self.enforcement_id,
            "verdict": self.verdict.as_str(),
            "release_manifest_id": self.release_manifest_id,
            "release_candidate_id": self.release_candidate_id,
            "force_exit_package_id": self.force_exit_package_id,
            "counters": self.counters.public_record(),
            "roots": self.roots.public_record(),
            "lane_observations": self.lane_observations.iter().map(LaneObservation::public_record).collect::<Vec<_>>(),
            "release_hold_exports": self.release_hold_exports.iter().map(ReleaseHoldExport::public_record).collect::<Vec<_>>(),
            "produced_at_epoch": self.produced_at_epoch,
            "produced_at_height": self.produced_at_height,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("enforcement-record", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub release_manifest: ReleaseManifestPolicy,
    pub pq_epoch_receipt: PqEpochFreshnessReceipt,
    pub reserve_slo_receipt: ReserveSloReceipt,
    pub privacy_budget_receipt: PrivacyBudgetReceipt,
    pub activation_adjudicator_receipt: ActivationAdjudicatorReceipt,
    pub enforcement_record: EnforcementRecord,
}

impl State {
    pub fn new(
        config: Config,
        release_manifest: ReleaseManifestPolicy,
        pq_epoch_receipt: PqEpochFreshnessReceipt,
        reserve_slo_receipt: ReserveSloReceipt,
        privacy_budget_receipt: PrivacyBudgetReceipt,
        activation_adjudicator_receipt: ActivationAdjudicatorReceipt,
    ) -> Result<Self> {
        config.validate()?;
        let enforcement_record = enforce_manifest_policy(
            &config,
            &release_manifest,
            &pq_epoch_receipt,
            &reserve_slo_receipt,
            &privacy_budget_receipt,
            &activation_adjudicator_receipt,
        )?;
        Ok(Self {
            config,
            release_manifest,
            pq_epoch_receipt,
            reserve_slo_receipt,
            privacy_budget_receipt,
            activation_adjudicator_receipt,
            enforcement_record,
        })
    }

    pub fn devnet() -> Self {
        let config = Config::devnet();
        let release_manifest = devnet_release_manifest(&config);
        let pq_epoch_receipt = devnet_pq_epoch_receipt(&config, &release_manifest);
        let reserve_slo_receipt = devnet_reserve_slo_receipt(&config, &release_manifest);
        let privacy_budget_receipt = devnet_privacy_budget_receipt(&config, &release_manifest);
        let activation_adjudicator_receipt =
            devnet_activation_adjudicator_receipt(&config, &release_manifest);
        match Self::new(
            config,
            release_manifest,
            pq_epoch_receipt,
            reserve_slo_receipt,
            privacy_budget_receipt,
            activation_adjudicator_receipt,
        ) {
            Ok(state) => state,
            Err(_) => Self::fallback_devnet(),
        }
    }

    pub fn fallback_devnet() -> Self {
        let config = Config::default();
        let release_manifest = devnet_release_manifest(&config);
        let pq_epoch_receipt = devnet_pq_epoch_receipt(&config, &release_manifest);
        let reserve_slo_receipt = devnet_reserve_slo_receipt(&config, &release_manifest);
        let privacy_budget_receipt = devnet_privacy_budget_receipt(&config, &release_manifest);
        let activation_adjudicator_receipt =
            devnet_activation_adjudicator_receipt(&config, &release_manifest);
        let enforcement_record = enforce_manifest_policy(
            &config,
            &release_manifest,
            &pq_epoch_receipt,
            &reserve_slo_receipt,
            &privacy_budget_receipt,
            &activation_adjudicator_receipt,
        )
        .into_fallback_record(&config, &release_manifest);
        Self {
            config,
            release_manifest,
            pq_epoch_receipt,
            reserve_slo_receipt,
            privacy_budget_receipt,
            activation_adjudicator_receipt,
            enforcement_record,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "protocol_version": PROTOCOL_VERSION,
            "schema_version": SCHEMA_VERSION,
            "chain_id": self.config.chain_id,
            "config": self.config.public_record(),
            "release_manifest": self.release_manifest.public_record(),
            "pq_epoch_receipt": self.pq_epoch_receipt.public_record(),
            "reserve_slo_receipt": self.reserve_slo_receipt.public_record(),
            "privacy_budget_receipt": self.privacy_budget_receipt.public_record(),
            "activation_adjudicator_receipt": self.activation_adjudicator_receipt.public_record(),
            "enforcement_record": self.enforcement_record.public_record(),
        })
    }

    pub fn state_root(&self) -> String {
        record_root("state", &self.public_record())
    }
}

trait FallbackRecord {
    fn into_fallback_record(
        self,
        config: &Config,
        release_manifest: &ReleaseManifestPolicy,
    ) -> EnforcementRecord;
}

impl FallbackRecord for Result<EnforcementRecord> {
    fn into_fallback_record(
        self,
        config: &Config,
        release_manifest: &ReleaseManifestPolicy,
    ) -> EnforcementRecord {
        match self {
            Ok(record) => record,
            Err(_) => fallback_enforcement_record(config, release_manifest),
        }
    }
}

pub fn enforce_manifest_policy(
    config: &Config,
    release_manifest: &ReleaseManifestPolicy,
    pq_epoch_receipt: &PqEpochFreshnessReceipt,
    reserve_slo_receipt: &ReserveSloReceipt,
    privacy_budget_receipt: &PrivacyBudgetReceipt,
    activation_adjudicator_receipt: &ActivationAdjudicatorReceipt,
) -> Result<EnforcementRecord> {
    config.validate()?;
    require(
        release_manifest.manifest_id == config.release_manifest_id,
        "release_manifest_id mismatch",
    )?;
    let mut lanes = Vec::new();
    lanes.push(
        LaneObservation::accepted(
            EnforcementLaneKind::PqEpochFreshness,
            release_manifest.expected_pq_epoch_root.clone(),
            pq_epoch_receipt.active_epoch_root.clone(),
            pq_epoch_receipt.state_root(),
            pq_epoch_receipt.observed_height,
            config.require_pq_epoch_freshness,
        )
        .mark_stale(config),
    );
    lanes.push(
        LaneObservation::accepted(
            EnforcementLaneKind::ReserveSloRoot,
            release_manifest.expected_reserve_slo_root.clone(),
            reserve_slo_receipt.reserve_slo_root.clone(),
            reserve_slo_receipt.state_root(),
            reserve_slo_receipt.observed_height,
            config.require_reserve_slo_root,
        )
        .mark_stale(config),
    );
    lanes.push(
        LaneObservation::accepted(
            EnforcementLaneKind::PrivacyBudgetRoot,
            release_manifest.expected_privacy_budget_root.clone(),
            privacy_budget_receipt.privacy_budget_root.clone(),
            privacy_budget_receipt.state_root(),
            privacy_budget_receipt.observed_height,
            config.require_privacy_budget_root,
        )
        .mark_stale(config),
    );
    lanes.push(
        LaneObservation::accepted(
            EnforcementLaneKind::NonLinkageRoot,
            release_manifest.expected_non_linkage_root.clone(),
            privacy_budget_receipt.non_linkage_root.clone(),
            privacy_budget_receipt.state_root(),
            privacy_budget_receipt.observed_height,
            config.require_non_linkage_root,
        )
        .mark_stale(config),
    );
    lanes.push(
        LaneObservation::accepted(
            EnforcementLaneKind::ActivationAdjudicatorRoot,
            release_manifest
                .expected_activation_adjudicator_root
                .clone(),
            activation_adjudicator_receipt
                .activation_adjudicator_root
                .clone(),
            activation_adjudicator_receipt.state_root(),
            activation_adjudicator_receipt.observed_height,
            config.require_activation_adjudicator_root,
        )
        .mark_stale(config),
    );
    lanes.push(
        LaneObservation::accepted(
            EnforcementLaneKind::ReleaseManifestBinding,
            release_manifest.release_policy_root.clone(),
            activation_adjudicator_receipt.release_policy_root.clone(),
            release_manifest.state_root(),
            release_manifest.manifest_height,
            true,
        )
        .mark_stale(config),
    );
    lanes.push(
        LaneObservation::accepted(
            EnforcementLaneKind::ReleaseHoldExport,
            release_manifest.expected_hold_export_root.clone(),
            release_manifest.expected_hold_export_root.clone(),
            release_manifest.state_root(),
            config.l2_reference_height,
            config.require_release_hold_exports,
        )
        .mark_stale(config),
    );
    apply_semantic_receipt_gates(
        config,
        &mut lanes,
        pq_epoch_receipt,
        reserve_slo_receipt,
        privacy_budget_receipt,
        activation_adjudicator_receipt,
    );
    require(
        lanes.len() <= config.max_lanes,
        "too many enforcement lanes",
    )?;
    let release_hold_exports = lanes
        .iter()
        .filter(|lane| lane.blocks_release())
        .map(|lane| {
            ReleaseHoldExport::from_lane(
                lane,
                &release_manifest.manifest_id,
                config.l2_reference_height,
            )
        })
        .collect::<Vec<_>>();
    let counters = EnforcementCounters::from_lanes(&lanes, &release_hold_exports);
    let verdict = verdict_for(
        config,
        &counters,
        release_manifest.production_release_requested,
    );
    let roots = EnforcementRoots {
        release_manifest_root: release_manifest.state_root(),
        pq_epoch_root: pq_epoch_receipt.state_root(),
        reserve_slo_root: reserve_slo_receipt.state_root(),
        privacy_budget_root: privacy_budget_receipt.state_root(),
        non_linkage_root: record_root(
            "non-linkage-root",
            &json!({
                "non_linkage_root": privacy_budget_receipt.non_linkage_root,
            }),
        ),
        activation_adjudicator_root: activation_adjudicator_receipt.state_root(),
        lane_observation_root: merkle_records("lane-observations", &lanes),
        hold_export_root: merkle_records("release-hold-exports", &release_hold_exports),
        counter_root: counters.state_root(),
    };
    let enforcement_id = enforcement_id(release_manifest, &roots, verdict);
    Ok(EnforcementRecord {
        enforcement_id,
        verdict,
        release_manifest_id: release_manifest.manifest_id.clone(),
        release_candidate_id: release_manifest.release_candidate_id.clone(),
        force_exit_package_id: release_manifest.force_exit_package_id.clone(),
        counters,
        roots,
        lane_observations: lanes,
        release_hold_exports,
        produced_at_epoch: config.current_epoch,
        produced_at_height: config.l2_reference_height,
    })
}

fn apply_semantic_receipt_gates(
    config: &Config,
    lanes: &mut [LaneObservation],
    pq_epoch_receipt: &PqEpochFreshnessReceipt,
    reserve_slo_receipt: &ReserveSloReceipt,
    privacy_budget_receipt: &PrivacyBudgetReceipt,
    activation_adjudicator_receipt: &ActivationAdjudicatorReceipt,
) {
    let mut overrides = BTreeMap::new();
    if !pq_epoch_receipt.present {
        overrides.insert(
            EnforcementLaneKind::PqEpochFreshness,
            LaneObservationStatus::Missing,
        );
    } else if !pq_epoch_receipt.is_fresh(config) {
        overrides.insert(
            EnforcementLaneKind::PqEpochFreshness,
            LaneObservationStatus::Rejected,
        );
    }
    if !reserve_slo_receipt.present {
        overrides.insert(
            EnforcementLaneKind::ReserveSloRoot,
            LaneObservationStatus::Missing,
        );
    } else if !reserve_slo_receipt.meets_slo(config) {
        overrides.insert(
            EnforcementLaneKind::ReserveSloRoot,
            LaneObservationStatus::Rejected,
        );
    }
    if !privacy_budget_receipt.present {
        overrides.insert(
            EnforcementLaneKind::PrivacyBudgetRoot,
            LaneObservationStatus::Missing,
        );
        overrides.insert(
            EnforcementLaneKind::NonLinkageRoot,
            LaneObservationStatus::Missing,
        );
    } else if !privacy_budget_receipt.budget_ok(config) {
        overrides.insert(
            EnforcementLaneKind::PrivacyBudgetRoot,
            LaneObservationStatus::Rejected,
        );
        overrides.insert(
            EnforcementLaneKind::NonLinkageRoot,
            LaneObservationStatus::Rejected,
        );
    }
    if !activation_adjudicator_receipt.present {
        overrides.insert(
            EnforcementLaneKind::ActivationAdjudicatorRoot,
            LaneObservationStatus::Missing,
        );
    } else if !activation_adjudicator_receipt.accepted() {
        overrides.insert(
            EnforcementLaneKind::ActivationAdjudicatorRoot,
            LaneObservationStatus::Rejected,
        );
    }
    for lane in lanes {
        if let Some(status) = overrides.get(&lane.lane_kind) {
            lane.status = *status;
        }
    }
}

fn verdict_for(
    config: &Config,
    counters: &EnforcementCounters,
    production_release_requested: bool,
) -> ManifestPolicyVerdict {
    if counters.accepted_lanes < config.min_required_lanes {
        return ManifestPolicyVerdict::ReleaseHeld;
    }
    if counters.blocking_lanes == 0 && production_release_requested {
        return ManifestPolicyVerdict::ReleaseAllowed;
    }
    if counters.mismatched_lanes > 0 && config.fail_closed_on_mismatch {
        return ManifestPolicyVerdict::ReleaseRejected;
    }
    if counters.missing_lanes > 0 && config.fail_closed_on_missing_lane {
        return ManifestPolicyVerdict::ReleaseHeld;
    }
    if counters.stale_lanes > 0 || counters.rejected_lanes > 0 {
        return ManifestPolicyVerdict::ReleaseHeld;
    }
    ManifestPolicyVerdict::ReleaseHeld
}

fn fallback_enforcement_record(
    config: &Config,
    release_manifest: &ReleaseManifestPolicy,
) -> EnforcementRecord {
    let lane = LaneObservation::missing(
        EnforcementLaneKind::ReleaseHoldExport,
        release_manifest.expected_hold_export_root.clone(),
        true,
    );
    let export = ReleaseHoldExport::from_lane(
        &lane,
        &release_manifest.manifest_id,
        config.l2_reference_height,
    );
    let lanes = vec![lane];
    let exports = vec![export];
    let counters = EnforcementCounters::from_lanes(&lanes, &exports);
    let roots = EnforcementRoots {
        release_manifest_root: release_manifest.state_root(),
        pq_epoch_root: String::new(),
        reserve_slo_root: String::new(),
        privacy_budget_root: String::new(),
        non_linkage_root: String::new(),
        activation_adjudicator_root: String::new(),
        lane_observation_root: merkle_records("lane-observations", &lanes),
        hold_export_root: merkle_records("release-hold-exports", &exports),
        counter_root: counters.state_root(),
    };
    EnforcementRecord {
        enforcement_id: enforcement_id(
            release_manifest,
            &roots,
            ManifestPolicyVerdict::ReleaseHeld,
        ),
        verdict: ManifestPolicyVerdict::ReleaseHeld,
        release_manifest_id: release_manifest.manifest_id.clone(),
        release_candidate_id: release_manifest.release_candidate_id.clone(),
        force_exit_package_id: release_manifest.force_exit_package_id.clone(),
        counters,
        roots,
        lane_observations: lanes,
        release_hold_exports: exports,
        produced_at_epoch: config.current_epoch,
        produced_at_height: config.l2_reference_height,
    }
}

pub fn devnet() -> State {
    State::devnet()
}

pub fn public_record() -> Value {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

pub fn public_record_for_state(state: &State) -> Value {
    state.public_record()
}

pub fn state_root_for_state(state: &State) -> String {
    state.state_root()
}

fn devnet_release_manifest(config: &Config) -> ReleaseManifestPolicy {
    let pq_epoch_root = domain_hash(
        "MONERO-L2-PQ-RELEASE-MANIFEST-PQ-EPOCH",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(&config.release_manifest_id),
            HashPart::Int(config.min_pq_epoch as i128),
        ],
        32,
    );
    let reserve_slo_root = domain_hash(
        "MONERO-L2-PQ-RELEASE-MANIFEST-RESERVE-SLO",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(&config.release_manifest_id),
            HashPart::Int(config.min_reserve_slo_bps as i128),
            HashPart::Int(config.min_reserve_atomic as i128),
        ],
        32,
    );
    let privacy_budget_root = domain_hash(
        "MONERO-L2-PQ-RELEASE-MANIFEST-PRIVACY-BUDGET",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(&config.release_manifest_id),
            HashPart::Int(config.min_privacy_remaining_bps as i128),
        ],
        32,
    );
    let non_linkage_root = domain_hash(
        "MONERO-L2-PQ-RELEASE-MANIFEST-NON-LINKAGE",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(&config.release_manifest_id),
            HashPart::Int(config.max_linkage_risk_bps as i128),
        ],
        32,
    );
    let activation_adjudicator_root = domain_hash(
        "MONERO-L2-PQ-RELEASE-MANIFEST-ACTIVATION-ADJUDICATOR",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(&config.release_manifest_id),
            HashPart::Str("accepted"),
        ],
        32,
    );
    let hold_export_root = domain_hash(
        "MONERO-L2-PQ-RELEASE-MANIFEST-HOLD-EXPORT",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(&config.release_manifest_id),
            HashPart::Str("no-blocking-holds"),
        ],
        32,
    );
    ReleaseManifestPolicy {
        manifest_id: config.release_manifest_id.clone(),
        release_candidate_id: deterministic_id("release-candidate", &config.release_manifest_id),
        force_exit_package_id: deterministic_id("force-exit-package", &config.release_manifest_id),
        release_policy_root: deterministic_root("release-policy", &config.release_manifest_id),
        expected_pq_epoch_root: pq_epoch_root,
        expected_reserve_slo_root: reserve_slo_root,
        expected_privacy_budget_root: privacy_budget_root,
        expected_non_linkage_root: non_linkage_root,
        expected_activation_adjudicator_root: activation_adjudicator_root,
        expected_hold_export_root: hold_export_root,
        manifest_epoch: config.current_epoch,
        manifest_height: config.l2_reference_height,
        production_release_requested: true,
    }
}

fn devnet_pq_epoch_receipt(
    config: &Config,
    manifest: &ReleaseManifestPolicy,
) -> PqEpochFreshnessReceipt {
    PqEpochFreshnessReceipt {
        receipt_id: deterministic_id("pq-epoch-receipt", &manifest.manifest_id),
        pq_key_epoch: config.current_epoch,
        previous_epoch_root: deterministic_root("previous-pq-epoch", &manifest.manifest_id),
        active_epoch_root: manifest.expected_pq_epoch_root.clone(),
        watcher_quorum_root: deterministic_root("pq-watcher-quorum", &manifest.manifest_id),
        observed_height: config.l2_reference_height,
        present: true,
    }
}

fn devnet_reserve_slo_receipt(
    config: &Config,
    manifest: &ReleaseManifestPolicy,
) -> ReserveSloReceipt {
    ReserveSloReceipt {
        receipt_id: deterministic_id("reserve-slo-receipt", &manifest.manifest_id),
        reserve_snapshot_root: deterministic_root("reserve-snapshot", &manifest.manifest_id),
        reserve_slo_root: manifest.expected_reserve_slo_root.clone(),
        reserve_attestation_root: deterministic_root("reserve-attestation", &manifest.manifest_id),
        reserved_atomic: config.min_reserve_atomic + 4_000_000_000,
        release_liability_atomic: config.min_reserve_atomic,
        reserve_slo_bps: config.min_reserve_slo_bps + 250,
        observed_height: config.l2_reference_height,
        present: true,
    }
}

fn devnet_privacy_budget_receipt(
    config: &Config,
    manifest: &ReleaseManifestPolicy,
) -> PrivacyBudgetReceipt {
    PrivacyBudgetReceipt {
        receipt_id: deterministic_id("privacy-budget-receipt", &manifest.manifest_id),
        privacy_budget_root: manifest.expected_privacy_budget_root.clone(),
        non_linkage_root: manifest.expected_non_linkage_root.clone(),
        spend_receipt_root: deterministic_root("privacy-spend-receipt", &manifest.manifest_id),
        remaining_budget_bps: config.min_privacy_remaining_bps + 600,
        linkage_risk_bps: config.max_linkage_risk_bps.saturating_sub(25),
        observed_height: config.l2_reference_height,
        present: true,
    }
}

fn devnet_activation_adjudicator_receipt(
    config: &Config,
    manifest: &ReleaseManifestPolicy,
) -> ActivationAdjudicatorReceipt {
    ActivationAdjudicatorReceipt {
        receipt_id: deterministic_id("activation-adjudicator-receipt", &manifest.manifest_id),
        adjudicator_id: deterministic_id("activation-adjudicator", &manifest.manifest_id),
        activation_adjudicator_root: manifest.expected_activation_adjudicator_root.clone(),
        accepted_activation_root: deterministic_root("accepted-activation", &manifest.manifest_id),
        rejected_activation_root: merkle_root("activation-adjudicator-rejected", &[]),
        release_policy_root: manifest.release_policy_root.clone(),
        observed_height: config.l2_reference_height,
        present: true,
        accepted_for_release: true,
    }
}

fn lane_id(
    lane_kind: EnforcementLaneKind,
    expected_root: &str,
    observed_root: &str,
    observed_height: u64,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-RELEASE-MANIFEST-ENFORCEMENT-LANE-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(lane_kind.as_str()),
            HashPart::Str(expected_root),
            HashPart::Str(observed_root),
            HashPart::Int(observed_height as i128),
        ],
        32,
    )
}

fn hold_export_id(lane: &LaneObservation, manifest_id: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-RELEASE-MANIFEST-HOLD-EXPORT-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(manifest_id),
            HashPart::Str(&lane.lane_id),
            HashPart::Str(lane.hold_reason().as_str()),
        ],
        32,
    )
}

fn hold_export_root(
    lane_kind: EnforcementLaneKind,
    reason: HoldExportReason,
    manifest_id: &str,
    expected_root: &str,
    observed_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-RELEASE-MANIFEST-HOLD-EXPORT-ROOT",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(manifest_id),
            HashPart::Str(lane_kind.as_str()),
            HashPart::Str(reason.as_str()),
            HashPart::Str(expected_root),
            HashPart::Str(observed_root),
        ],
        32,
    )
}

fn enforcement_id(
    manifest: &ReleaseManifestPolicy,
    roots: &EnforcementRoots,
    verdict: ManifestPolicyVerdict,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-RELEASE-MANIFEST-ENFORCEMENT-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(&manifest.manifest_id),
            HashPart::Str(&manifest.release_candidate_id),
            HashPart::Str(verdict.as_str()),
            HashPart::Str(&roots.lane_observation_root),
            HashPart::Str(&roots.hold_export_root),
        ],
        32,
    )
}

fn deterministic_id(label: &str, seed: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-RELEASE-MANIFEST-DETERMINISTIC-ID",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(label),
            HashPart::Str(seed),
        ],
        32,
    )
}

fn deterministic_root(label: &str, seed: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-RELEASE-MANIFEST-DETERMINISTIC-ROOT",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(label),
            HashPart::Str(seed),
        ],
        32,
    )
}

fn record_root(label: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-RELEASE-MANIFEST-ENFORCEMENT-RECORD",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(DOMAIN),
            HashPart::Str(label),
            HashPart::Json(record),
        ],
        32,
    )
}

fn merkle_records<T>(label: &str, records: &[T]) -> String
where
    T: PublicRoot,
{
    let leaves = records
        .iter()
        .map(|record| Value::String(record.public_root()))
        .collect::<Vec<_>>();
    if leaves.is_empty() {
        return domain_hash(
            "MONERO-L2-PQ-RELEASE-MANIFEST-EMPTY-MERKLE-ROOT",
            &[HashPart::Str(CHAIN_ID), HashPart::Str(label)],
            32,
        );
    }
    merkle_root(label, &leaves)
}

trait PublicRoot {
    fn public_root(&self) -> String;
}

impl PublicRoot for LaneObservation {
    fn public_root(&self) -> String {
        self.state_root()
    }
}

impl PublicRoot for ReleaseHoldExport {
    fn public_root(&self) -> String {
        self.state_root()
    }
}

fn require(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message.to_string())
    }
}

fn require_non_empty(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(format!("{field} must not be empty"))
    } else {
        Ok(())
    }
}
