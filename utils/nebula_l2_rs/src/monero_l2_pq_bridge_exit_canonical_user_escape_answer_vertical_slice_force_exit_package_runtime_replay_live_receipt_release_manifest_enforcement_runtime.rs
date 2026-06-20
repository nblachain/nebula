use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageRuntimeReplayLiveReceiptReleaseManifestEnforcementRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;
pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RUNTIME_REPLAY_LIVE_RECEIPT_RELEASE_MANIFEST_ENFORCEMENT_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-runtime-replay-live-receipt-release-manifest-enforcement-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_RUNTIME_REPLAY_LIVE_RECEIPT_RELEASE_MANIFEST_ENFORCEMENT_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const ENFORCEMENT_SUITE: &str =
    "force-exit-package-runtime-replay-live-receipt-release-manifest-enforcement-v1";
pub const DEFAULT_VERTICAL_SLICE_ID: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-devnet-v1";
pub const DEFAULT_FORCE_EXIT_PACKAGE_ID: &str =
    "force-exit-package-runtime-replay-live-receipt-release-manifest-enforcement-devnet-0001";
pub const DEFAULT_RELEASE_MANIFEST_ID: &str =
    "release-manifest-runtime-replay-live-receipt-enforcement-devnet-0001";
pub const DEFAULT_RELEASE_EPOCH: u64 = 79;
pub const DEFAULT_L2_HEIGHT: u64 = 893_079;
pub const DEFAULT_MONERO_HEIGHT: u64 = 3_079_893;
pub const DEFAULT_MAX_L2_FRESHNESS_DELTA: u64 = 9;
pub const DEFAULT_MAX_MONERO_FRESHNESS_DELTA: u64 = 6;
pub const DEFAULT_MIN_COMMAND_ROOTS: u64 = 8;
pub const DEFAULT_MIN_ACCEPTED_ENFORCEMENTS: u64 = 8;
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub enforcement_suite: String,
    pub vertical_slice_id: String,
    pub force_exit_package_id: String,
    pub release_manifest_id: String,
    pub release_epoch: u64,
    pub enforcement_l2_height: u64,
    pub enforcement_monero_height: u64,
    pub max_l2_freshness_delta: u64,
    pub max_monero_freshness_delta: u64,
    pub min_command_roots: u64,
    pub min_accepted_enforcements: u64,
    pub require_observed_replay_command_roots: bool,
    pub require_expected_observed_root_match: bool,
    pub require_activation_adjudicator_root: bool,
    pub require_manifest_binding_root: bool,
    pub require_freshness_windows: bool,
    pub export_fail_closed_holds: bool,
    pub fail_closed_on_any_violation: bool,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            enforcement_suite: ENFORCEMENT_SUITE.to_string(),
            vertical_slice_id: DEFAULT_VERTICAL_SLICE_ID.to_string(),
            force_exit_package_id: DEFAULT_FORCE_EXIT_PACKAGE_ID.to_string(),
            release_manifest_id: DEFAULT_RELEASE_MANIFEST_ID.to_string(),
            release_epoch: DEFAULT_RELEASE_EPOCH,
            enforcement_l2_height: DEFAULT_L2_HEIGHT,
            enforcement_monero_height: DEFAULT_MONERO_HEIGHT,
            max_l2_freshness_delta: DEFAULT_MAX_L2_FRESHNESS_DELTA,
            max_monero_freshness_delta: DEFAULT_MAX_MONERO_FRESHNESS_DELTA,
            min_command_roots: DEFAULT_MIN_COMMAND_ROOTS,
            min_accepted_enforcements: DEFAULT_MIN_ACCEPTED_ENFORCEMENTS,
            require_observed_replay_command_roots: true,
            require_expected_observed_root_match: true,
            require_activation_adjudicator_root: true,
            require_manifest_binding_root: true,
            require_freshness_windows: true,
            export_fail_closed_holds: true,
            fail_closed_on_any_violation: true,
        }
    }
}
impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }
    pub fn required_gate_count(&self) -> u64 {
        [
            self.require_observed_replay_command_roots,
            self.require_expected_observed_root_match,
            self.require_activation_adjudicator_root,
            self.require_manifest_binding_root,
            self.require_freshness_windows,
            self.export_fail_closed_holds,
        ]
        .iter()
        .filter(|required| **required)
        .count() as u64
    }
    pub fn public_record(&self) -> Value {
        json!({"chain_id": self.chain_id, "protocol_version": self.protocol_version, "schema_version": self.schema_version, "hash_suite": self.hash_suite, "enforcement_suite": self.enforcement_suite, "vertical_slice_id": self.vertical_slice_id, "force_exit_package_id": self.force_exit_package_id, "release_manifest_id": self.release_manifest_id, "release_epoch": self.release_epoch, "enforcement_l2_height": self.enforcement_l2_height, "enforcement_monero_height": self.enforcement_monero_height, "max_l2_freshness_delta": self.max_l2_freshness_delta, "max_monero_freshness_delta": self.max_monero_freshness_delta, "min_command_roots": self.min_command_roots, "min_accepted_enforcements": self.min_accepted_enforcements, "required_gate_count": self.required_gate_count(), "require_observed_replay_command_roots": self.require_observed_replay_command_roots, "require_expected_observed_root_match": self.require_expected_observed_root_match, "require_activation_adjudicator_root": self.require_activation_adjudicator_root, "require_manifest_binding_root": self.require_manifest_binding_root, "require_freshness_windows": self.require_freshness_windows, "export_fail_closed_holds": self.export_fail_closed_holds, "fail_closed_on_any_violation": self.fail_closed_on_any_violation})
    }
    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EnforcementLane {
    RuntimeExecutionReplay,
    WalletEscapeCommand,
    WatchtowerChallengeCommand,
    PqAuthorityActivation,
    ReserveReleaseCommand,
    SettlementAnchorCommand,
    PrivacyBudgetCommand,
    ClosureManifestCommand,
}
impl EnforcementLane {
    pub fn ordered() -> &'static [Self] {
        &[
            Self::RuntimeExecutionReplay,
            Self::WalletEscapeCommand,
            Self::WatchtowerChallengeCommand,
            Self::PqAuthorityActivation,
            Self::ReserveReleaseCommand,
            Self::SettlementAnchorCommand,
            Self::PrivacyBudgetCommand,
            Self::ClosureManifestCommand,
        ]
    }
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeExecutionReplay => "runtime_execution_replay",
            Self::WalletEscapeCommand => "wallet_escape_command",
            Self::WatchtowerChallengeCommand => "watchtower_challenge_command",
            Self::PqAuthorityActivation => "pq_authority_activation",
            Self::ReserveReleaseCommand => "reserve_release_command",
            Self::SettlementAnchorCommand => "settlement_anchor_command",
            Self::PrivacyBudgetCommand => "privacy_budget_command",
            Self::ClosureManifestCommand => "closure_manifest_command",
        }
    }
    pub fn command_name(self) -> &'static str {
        match self {
            Self::RuntimeExecutionReplay => "force_exit_runtime_replay_live_receipt",
            Self::WalletEscapeCommand => "force_exit_wallet_escape_live_receipt",
            Self::WatchtowerChallengeCommand => "force_exit_watchtower_challenge_live_receipt",
            Self::PqAuthorityActivation => "force_exit_pq_authority_activation_live_receipt",
            Self::ReserveReleaseCommand => "force_exit_reserve_release_live_receipt",
            Self::SettlementAnchorCommand => "force_exit_settlement_anchor_live_receipt",
            Self::PrivacyBudgetCommand => "force_exit_privacy_budget_live_receipt",
            Self::ClosureManifestCommand => "force_exit_closure_manifest_live_receipt",
        }
    }
    pub fn release_weight(self) -> u64 {
        match self {
            Self::RuntimeExecutionReplay => 19,
            Self::WalletEscapeCommand => 14,
            Self::WatchtowerChallengeCommand => 13,
            Self::PqAuthorityActivation => 18,
            Self::ReserveReleaseCommand => 16,
            Self::SettlementAnchorCommand => 15,
            Self::PrivacyBudgetCommand => 12,
            Self::ClosureManifestCommand => 11,
        }
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EnforcementStatus {
    Enforced,
    MissingObservedCommandRoot,
    ExpectedObservedRootMismatch,
    MissingActivationAdjudicatorRoot,
    MissingManifestBindingRoot,
    StaleFreshnessWindow,
    FailClosedHoldExported,
}
impl EnforcementStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Enforced => "enforced",
            Self::MissingObservedCommandRoot => "missing_observed_command_root",
            Self::ExpectedObservedRootMismatch => "expected_observed_root_mismatch",
            Self::MissingActivationAdjudicatorRoot => "missing_activation_adjudicator_root",
            Self::MissingManifestBindingRoot => "missing_manifest_binding_root",
            Self::StaleFreshnessWindow => "stale_freshness_window",
            Self::FailClosedHoldExported => "fail_closed_hold_exported",
        }
    }
    pub fn permits_release(self) -> bool {
        matches!(self, Self::Enforced)
    }
    pub fn requires_hold(self) -> bool {
        !self.permits_release()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseManifestDecision {
    ManifestEnforced,
    HoldForObservedCommandRoot,
    HoldForRootMismatch,
    HoldForAdjudicatorBinding,
    HoldForManifestBinding,
    HoldForFreshness,
    FailClosedHold,
}
impl ReleaseManifestDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ManifestEnforced => "manifest_enforced",
            Self::HoldForObservedCommandRoot => "hold_for_observed_command_root",
            Self::HoldForRootMismatch => "hold_for_root_mismatch",
            Self::HoldForAdjudicatorBinding => "hold_for_adjudicator_binding",
            Self::HoldForManifestBinding => "hold_for_manifest_binding",
            Self::HoldForFreshness => "hold_for_freshness",
            Self::FailClosedHold => "fail_closed_hold",
        }
    }
    pub fn permits_release(self) -> bool {
        matches!(self, Self::ManifestEnforced)
    }
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceRoots {
    pub runtime_replay_activation_root: String,
    pub release_policy_adjudicator_root: String,
    pub release_policy_manifest_binding_root: String,
    pub live_receipt_cross_domain_root: String,
    pub replacement_manifest_root: String,
    pub package_closure_root: String,
    pub hold_export_bus_root: String,
}
impl SourceRoots {
    pub fn devnet(config: &Config) -> Self {
        Self {
            runtime_replay_activation_root: deterministic_root(
                "runtime-replay-activation",
                &config.force_exit_package_id,
            ),
            release_policy_adjudicator_root: deterministic_root(
                "release-policy-live-receipt-adjudicator",
                &config.force_exit_package_id,
            ),
            release_policy_manifest_binding_root: deterministic_root(
                "release-policy-manifest-binding",
                &config.release_manifest_id,
            ),
            live_receipt_cross_domain_root: deterministic_root(
                "live-receipt-cross-domain",
                &config.release_manifest_id,
            ),
            replacement_manifest_root: deterministic_root(
                "heavy-gate-replacement-manifest",
                &config.release_manifest_id,
            ),
            package_closure_root: deterministic_root(
                "package-closure",
                &config.force_exit_package_id,
            ),
            hold_export_bus_root: deterministic_root(
                "fail-closed-hold-export-bus",
                &config.release_manifest_id,
            ),
        }
    }
    pub fn public_record(&self) -> Value {
        json!({"runtime_replay_activation_root": self.runtime_replay_activation_root, "release_policy_adjudicator_root": self.release_policy_adjudicator_root, "release_policy_manifest_binding_root": self.release_policy_manifest_binding_root, "live_receipt_cross_domain_root": self.live_receipt_cross_domain_root, "replacement_manifest_root": self.replacement_manifest_root, "package_closure_root": self.package_closure_root, "hold_export_bus_root": self.hold_export_bus_root})
    }
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReplayCommandObservation {
    pub command_id: String,
    pub lane: EnforcementLane,
    pub command_name: String,
    pub expected_command_root: String,
    pub observed_command_root: String,
    pub command_receipt_root: String,
    pub witness_quorum_root: String,
    pub observed_l2_height: u64,
    pub observed_monero_height: u64,
    pub witness_count: u64,
}
impl ReplayCommandObservation {
    pub fn devnet(
        config: &Config,
        source_roots: &SourceRoots,
        lane: EnforcementLane,
        index: u64,
    ) -> Self {
        let expected_command_root = expected_command_root(config, source_roots, lane);
        let observed_command_root = expected_command_root.clone();
        let command_receipt_root = command_receipt_root(config, lane, &observed_command_root);
        let witness_quorum_root = witness_quorum_root(config, lane, index);
        let observed_l2_height = config.enforcement_l2_height.saturating_sub(index);
        let observed_monero_height = config.enforcement_monero_height.saturating_sub(index / 2);
        let command_id = replay_command_id(
            lane,
            &expected_command_root,
            &observed_command_root,
            &command_receipt_root,
            &witness_quorum_root,
        );
        Self {
            command_id,
            lane,
            command_name: lane.command_name().to_string(),
            expected_command_root,
            observed_command_root,
            command_receipt_root,
            witness_quorum_root,
            observed_l2_height,
            observed_monero_height,
            witness_count: 5 + index,
        }
    }
    pub fn roots_match(&self) -> bool {
        self.expected_command_root == self.observed_command_root
    }
    pub fn public_record(&self) -> Value {
        json!({"command_id": self.command_id, "lane": self.lane.as_str(), "command_name": self.command_name, "expected_command_root": self.expected_command_root, "observed_command_root": self.observed_command_root, "command_receipt_root": self.command_receipt_root, "witness_quorum_root": self.witness_quorum_root, "observed_l2_height": self.observed_l2_height, "observed_monero_height": self.observed_monero_height, "witness_count": self.witness_count, "roots_match": self.roots_match()})
    }
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FreshnessWindow {
    pub freshness_id: String,
    pub lane: EnforcementLane,
    pub observed_l2_height: u64,
    pub observed_monero_height: u64,
    pub enforcement_l2_height: u64,
    pub enforcement_monero_height: u64,
    pub l2_delta: u64,
    pub monero_delta: u64,
    pub max_l2_delta: u64,
    pub max_monero_delta: u64,
    pub accepted: bool,
}
impl FreshnessWindow {
    pub fn from_observation(config: &Config, observation: &ReplayCommandObservation) -> Self {
        let l2_delta = config
            .enforcement_l2_height
            .saturating_sub(observation.observed_l2_height);
        let monero_delta = config
            .enforcement_monero_height
            .saturating_sub(observation.observed_monero_height);
        let accepted = l2_delta <= config.max_l2_freshness_delta
            && monero_delta <= config.max_monero_freshness_delta;
        let freshness_id = freshness_window_id(observation.lane, l2_delta, monero_delta, accepted);
        Self {
            freshness_id,
            lane: observation.lane,
            observed_l2_height: observation.observed_l2_height,
            observed_monero_height: observation.observed_monero_height,
            enforcement_l2_height: config.enforcement_l2_height,
            enforcement_monero_height: config.enforcement_monero_height,
            l2_delta,
            monero_delta,
            max_l2_delta: config.max_l2_freshness_delta,
            max_monero_delta: config.max_monero_freshness_delta,
            accepted,
        }
    }
    pub fn public_record(&self) -> Value {
        json!({
            "freshness_id": self.freshness_id,
            "lane": self.lane.as_str(),
            "observed_l2_height": self.observed_l2_height,
            "observed_monero_height": self.observed_monero_height,
            "enforcement_l2_height": self.enforcement_l2_height,
            "enforcement_monero_height": self.enforcement_monero_height,
            "l2_delta": self.l2_delta,
            "monero_delta": self.monero_delta,
            "max_l2_delta": self.max_l2_delta,
            "max_monero_delta": self.max_monero_delta,
            "accepted": self.accepted,
        })
    }
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ManifestBinding {
    pub binding_id: String,
    pub lane: EnforcementLane,
    pub release_manifest_id: String,
    pub manifest_binding_root: String,
    pub manifest_expected_command_root: String,
    pub manifest_observed_command_root: String,
    pub activation_adjudicator_root: String,
    pub release_policy_manifest_root: String,
    pub replacement_manifest_root: String,
    pub bound: bool,
}
impl ManifestBinding {
    pub fn from_observation(
        config: &Config,
        source_roots: &SourceRoots,
        observation: &ReplayCommandObservation,
    ) -> Self {
        let manifest_binding_root = manifest_binding_root(config, source_roots, observation);
        let activation_adjudicator_root =
            activation_adjudicator_root(config, source_roots, observation.lane);
        let binding_id = manifest_binding_id(
            observation.lane,
            &manifest_binding_root,
            &activation_adjudicator_root,
            &source_roots.release_policy_manifest_binding_root,
        );
        Self {
            binding_id,
            lane: observation.lane,
            release_manifest_id: config.release_manifest_id.clone(),
            manifest_binding_root,
            manifest_expected_command_root: observation.expected_command_root.clone(),
            manifest_observed_command_root: observation.observed_command_root.clone(),
            activation_adjudicator_root,
            release_policy_manifest_root: source_roots.release_policy_manifest_binding_root.clone(),
            replacement_manifest_root: source_roots.replacement_manifest_root.clone(),
            bound: observation.roots_match(),
        }
    }
    pub fn public_record(&self) -> Value {
        json!({
            "binding_id": self.binding_id,
            "lane": self.lane.as_str(),
            "release_manifest_id": self.release_manifest_id,
            "manifest_binding_root": self.manifest_binding_root,
            "manifest_expected_command_root": self.manifest_expected_command_root,
            "manifest_observed_command_root": self.manifest_observed_command_root,
            "activation_adjudicator_root": self.activation_adjudicator_root,
            "release_policy_manifest_root": self.release_policy_manifest_root,
            "replacement_manifest_root": self.replacement_manifest_root,
            "bound": self.bound,
        })
    }
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EnforcementRecord {
    pub enforcement_id: String,
    pub lane: EnforcementLane,
    pub command_observation: ReplayCommandObservation,
    pub freshness_window: FreshnessWindow,
    pub manifest_binding: ManifestBinding,
    pub expected_observed_match: bool,
    pub observed_command_root_present: bool,
    pub activation_adjudicator_root_present: bool,
    pub manifest_binding_root_present: bool,
    pub status: EnforcementStatus,
    pub decision: ReleaseManifestDecision,
    pub fail_closed_export_id: String,
    pub fail_closed_export_root: String,
    pub fail_closed_hold_reason: String,
    pub fail_closed_operator_action: String,
    pub fail_closed_exported: bool,
    pub release_weight: u64,
}
impl EnforcementRecord {
    pub fn from_observation(
        config: &Config,
        source_roots: &SourceRoots,
        observation: ReplayCommandObservation,
    ) -> Self {
        let freshness_window = FreshnessWindow::from_observation(config, &observation);
        let manifest_binding =
            ManifestBinding::from_observation(config, source_roots, &observation);
        let observed_command_root_present = !observation.observed_command_root.is_empty();
        let expected_observed_match = observation.roots_match();
        let activation_adjudicator_root_present =
            !manifest_binding.activation_adjudicator_root.is_empty();
        let manifest_binding_root_present = !manifest_binding.manifest_binding_root.is_empty();
        let status = classify_enforcement_status(
            config,
            observed_command_root_present,
            expected_observed_match,
            activation_adjudicator_root_present,
            manifest_binding_root_present,
            freshness_window.accepted,
        );
        let decision = decision_for_status(status);
        let hold_reason = hold_reason_for_status(status);
        let fail_closed_export_root =
            hold_export_root(config, source_roots, observation.lane, decision, status);
        let fail_closed_export_id = hold_export_id(
            observation.lane,
            decision,
            status,
            &fail_closed_export_root,
            hold_reason,
        );
        let enforcement_id = enforcement_record_id(
            observation.lane,
            &observation.command_id,
            &freshness_window.freshness_id,
            &manifest_binding.binding_id,
            status,
            decision,
        );
        Self {
            enforcement_id,
            lane: observation.lane,
            command_observation: observation,
            freshness_window,
            manifest_binding,
            expected_observed_match,
            observed_command_root_present,
            activation_adjudicator_root_present,
            manifest_binding_root_present,
            status,
            decision,
            fail_closed_export_id,
            fail_closed_export_root,
            fail_closed_hold_reason: hold_reason.to_string(),
            fail_closed_operator_action: "block_release_manifest_activation".to_string(),
            fail_closed_exported: status.requires_hold(),
            release_weight: observation.lane.release_weight(),
        }
    }
    pub fn permits_release(&self) -> bool {
        self.status.permits_release() && self.decision.permits_release()
    }
    pub fn public_record(&self) -> Value {
        json!({
            "enforcement_id": self.enforcement_id,
            "lane": self.lane.as_str(),
            "command_observation": self.command_observation.public_record(),
            "freshness_window": self.freshness_window.public_record(),
            "manifest_binding": self.manifest_binding.public_record(),
            "expected_observed_match": self.expected_observed_match,
            "observed_command_root_present": self.observed_command_root_present,
            "activation_adjudicator_root_present": self.activation_adjudicator_root_present,
            "manifest_binding_root_present": self.manifest_binding_root_present,
            "status": self.status.as_str(),
            "decision": self.decision.as_str(),
            "fail_closed_export_id": self.fail_closed_export_id,
            "fail_closed_export_root": self.fail_closed_export_root,
            "fail_closed_hold_reason": self.fail_closed_hold_reason,
            "fail_closed_operator_action": self.fail_closed_operator_action,
            "fail_closed_exported": self.fail_closed_exported,
            "release_weight": self.release_weight,
            "permits_release": self.permits_release(),
        })
    }
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct EnforcementCounters {
    pub command_roots_observed: u64,
    pub expected_observed_matches: u64,
    pub activation_adjudicator_roots: u64,
    pub manifest_binding_roots: u64,
    pub freshness_windows_accepted: u64,
    pub enforced_records: u64,
    pub fail_closed_holds_exported: u64,
    pub release_weight_accepted: u64,
}
impl EnforcementCounters {
    pub fn from_records(records: &[EnforcementRecord]) -> Self {
        let mut counters = Self::default();
        for record in records {
            if record.observed_command_root_present {
                counters.command_roots_observed += 1;
            }
            if record.expected_observed_match {
                counters.expected_observed_matches += 1;
            }
            if record.activation_adjudicator_root_present {
                counters.activation_adjudicator_roots += 1;
            }
            if record.manifest_binding_root_present {
                counters.manifest_binding_roots += 1;
            }
            if record.freshness_window.accepted {
                counters.freshness_windows_accepted += 1;
            }
            if record.permits_release() {
                counters.enforced_records += 1;
                counters.release_weight_accepted += record.release_weight;
            }
            if record.fail_closed_exported {
                counters.fail_closed_holds_exported += 1;
            }
        }
        counters
    }
    pub fn public_record(&self) -> Value {
        json!({
            "command_roots_observed": self.command_roots_observed,
            "expected_observed_matches": self.expected_observed_matches,
            "activation_adjudicator_roots": self.activation_adjudicator_roots,
            "manifest_binding_roots": self.manifest_binding_roots,
            "freshness_windows_accepted": self.freshness_windows_accepted,
            "enforced_records": self.enforced_records,
            "fail_closed_holds_exported": self.fail_closed_holds_exported,
            "release_weight_accepted": self.release_weight_accepted,
        })
    }
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EnforcementRoots {
    pub config_root: String,
    pub source_roots_root: String,
    pub command_observation_root: String,
    pub freshness_window_root: String,
    pub manifest_binding_root: String,
    pub fail_closed_export_root: String,
    pub enforcement_record_root: String,
    pub counters_root: String,
    pub release_manifest_enforcement_root: String,
}
impl EnforcementRoots {
    pub fn from_parts(
        config: &Config,
        source_roots: &SourceRoots,
        records: &[EnforcementRecord],
        counters: &EnforcementCounters,
    ) -> Self {
        let command_observation_root = merkle_root(
            "MONERO-RELEASE-MANIFEST-ENFORCEMENT-COMMAND-OBSERVATIONS",
            &records
                .iter()
                .map(|record| record.command_observation.public_record())
                .collect::<Vec<_>>(),
        );
        let freshness_window_root = merkle_root(
            "MONERO-RELEASE-MANIFEST-ENFORCEMENT-FRESHNESS-WINDOWS",
            &records
                .iter()
                .map(|record| record.freshness_window.public_record())
                .collect::<Vec<_>>(),
        );
        let manifest_binding_root = merkle_root(
            "MONERO-RELEASE-MANIFEST-ENFORCEMENT-MANIFEST-BINDINGS",
            &records
                .iter()
                .map(|record| record.manifest_binding.public_record())
                .collect::<Vec<_>>(),
        );
        let fail_closed_export_root = merkle_root(
            "MONERO-RELEASE-MANIFEST-ENFORCEMENT-FAIL-CLOSED-EXPORTS",
            &records
                .iter()
                .map(|record| {
                    json!({
                        "fail_closed_export_id": record.fail_closed_export_id,
                        "fail_closed_export_root": record.fail_closed_export_root,
                        "fail_closed_hold_reason": record.fail_closed_hold_reason,
                        "fail_closed_operator_action": record.fail_closed_operator_action,
                        "fail_closed_exported": record.fail_closed_exported,
                        "lane": record.lane.as_str(),
                        "status": record.status.as_str(),
                        "decision": record.decision.as_str(),
                    })
                })
                .collect::<Vec<_>>(),
        );
        let enforcement_record_root = merkle_root(
            "MONERO-RELEASE-MANIFEST-ENFORCEMENT-RECORDS",
            &records
                .iter()
                .map(EnforcementRecord::public_record)
                .collect::<Vec<_>>(),
        );
        let config_root = config.state_root();
        let source_roots_root = record_root("source-roots", &source_roots.public_record());
        let counters_root = record_root("enforcement-counters", &counters.public_record());
        let release_manifest_enforcement_root = domain_hash(
            "MONERO-RELEASE-MANIFEST-ENFORCEMENT-ROOT",
            &[
                HashPart::Str(&config_root),
                HashPart::Str(&source_roots_root),
                HashPart::Str(&command_observation_root),
                HashPart::Str(&freshness_window_root),
                HashPart::Str(&manifest_binding_root),
                HashPart::Str(&fail_closed_export_root),
                HashPart::Str(&enforcement_record_root),
                HashPart::Str(&counters_root),
            ],
            32,
        );
        Self {
            config_root,
            source_roots_root,
            command_observation_root,
            freshness_window_root,
            manifest_binding_root,
            fail_closed_export_root,
            enforcement_record_root,
            counters_root,
            release_manifest_enforcement_root,
        }
    }
    pub fn public_record(&self) -> Value {
        json!({
            "config_root": self.config_root,
            "source_roots_root": self.source_roots_root,
            "command_observation_root": self.command_observation_root,
            "freshness_window_root": self.freshness_window_root,
            "manifest_binding_root": self.manifest_binding_root,
            "fail_closed_export_root": self.fail_closed_export_root,
            "enforcement_record_root": self.enforcement_record_root,
            "counters_root": self.counters_root,
            "release_manifest_enforcement_root": self.release_manifest_enforcement_root,
        })
    }
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub source_roots: SourceRoots,
    pub records: Vec<EnforcementRecord>,
    pub counters: EnforcementCounters,
    pub roots: EnforcementRoots,
    pub release_manifest_decision: ReleaseManifestDecision,
}
impl State {
    pub fn new(
        config: Config,
        source_roots: SourceRoots,
        records: Vec<EnforcementRecord>,
    ) -> Result<Self> {
        let counters = EnforcementCounters::from_records(&records);
        if config.require_observed_replay_command_roots
            && counters.command_roots_observed < config.min_command_roots
        {
            return Err(
                "observed replay command roots below release manifest threshold".to_string(),
            );
        }
        if config.require_expected_observed_root_match
            && counters.expected_observed_matches < config.min_accepted_enforcements
        {
            return Err("expected and observed replay command roots do not match release manifest threshold".to_string());
        }
        if config.require_activation_adjudicator_root
            && counters.activation_adjudicator_roots < config.min_accepted_enforcements
        {
            return Err(
                "activation adjudicator roots below release manifest threshold".to_string(),
            );
        }
        if config.require_manifest_binding_root
            && counters.manifest_binding_roots < config.min_accepted_enforcements
        {
            return Err("manifest binding roots below release manifest threshold".to_string());
        }
        if config.require_freshness_windows
            && counters.freshness_windows_accepted < config.min_accepted_enforcements
        {
            return Err("freshness windows below release manifest threshold".to_string());
        }
        if config.fail_closed_on_any_violation && counters.fail_closed_holds_exported > 0 {
            return Err(
                "fail closed hold exports present for release manifest enforcement".to_string(),
            );
        }
        let release_manifest_decision =
            if counters.enforced_records >= config.min_accepted_enforcements {
                ReleaseManifestDecision::ManifestEnforced
            } else {
                ReleaseManifestDecision::FailClosedHold
            };
        let roots = EnforcementRoots::from_parts(&config, &source_roots, &records, &counters);
        Ok(Self {
            config,
            source_roots,
            records,
            counters,
            roots,
            release_manifest_decision,
        })
    }
    pub fn devnet() -> Self {
        let config = Config::devnet();
        let source_roots = SourceRoots::devnet(&config);
        let records = EnforcementLane::ordered()
            .iter()
            .enumerate()
            .map(|(index, lane)| {
                let observation =
                    ReplayCommandObservation::devnet(&config, &source_roots, *lane, index as u64);
                EnforcementRecord::from_observation(&config, &source_roots, observation)
            })
            .collect::<Vec<_>>();
        match Self::new(config, source_roots, records) {
            Ok(state) => state,
            Err(error) => {
                let fallback_config = Config {
                    fail_closed_on_any_violation: false,
                    ..Config::devnet()
                };
                let mut fallback_source_roots = SourceRoots::devnet(&fallback_config);
                fallback_source_roots.hold_export_bus_root =
                    deterministic_root("devnet-construction-error", &error);
                let fallback_records = Vec::new();
                let fallback_counters = EnforcementCounters::default();
                let fallback_roots = EnforcementRoots::from_parts(
                    &fallback_config,
                    &fallback_source_roots,
                    &fallback_records,
                    &fallback_counters,
                );
                Self {
                    config: fallback_config,
                    source_roots: fallback_source_roots,
                    records: fallback_records,
                    counters: fallback_counters,
                    roots: fallback_roots,
                    release_manifest_decision: ReleaseManifestDecision::FailClosedHold,
                }
            }
        }
    }
    pub fn release_permitted(&self) -> bool {
        self.release_manifest_decision.permits_release()
    }
    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.config.chain_id,
            "protocol_version": self.config.protocol_version,
            "schema_version": self.config.schema_version,
            "config": self.config.public_record(),
            "source_roots": self.source_roots.public_record(),
            "records": self.records.iter().map(EnforcementRecord::public_record).collect::<Vec<_>>(),
            "counters": self.counters.public_record(),
            "roots": self.roots.public_record(),
            "release_manifest_decision": self.release_manifest_decision.as_str(),
            "release_permitted": self.release_permitted(),
        })
    }
    pub fn state_root(&self) -> String {
        domain_hash(
            "MONERO-RELEASE-MANIFEST-ENFORCEMENT-STATE",
            &[HashPart::Json(&self.public_record())],
            32,
        )
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

fn classify_enforcement_status(
    config: &Config,
    observed_command_root_present: bool,
    expected_observed_match: bool,
    activation_adjudicator_root_present: bool,
    manifest_binding_root_present: bool,
    freshness_window_accepted: bool,
) -> EnforcementStatus {
    if config.require_observed_replay_command_roots && !observed_command_root_present {
        EnforcementStatus::MissingObservedCommandRoot
    } else if config.require_expected_observed_root_match && !expected_observed_match {
        EnforcementStatus::ExpectedObservedRootMismatch
    } else if config.require_activation_adjudicator_root && !activation_adjudicator_root_present {
        EnforcementStatus::MissingActivationAdjudicatorRoot
    } else if config.require_manifest_binding_root && !manifest_binding_root_present {
        EnforcementStatus::MissingManifestBindingRoot
    } else if config.require_freshness_windows && !freshness_window_accepted {
        EnforcementStatus::StaleFreshnessWindow
    } else {
        EnforcementStatus::Enforced
    }
}
fn decision_for_status(status: EnforcementStatus) -> ReleaseManifestDecision {
    match status {
        EnforcementStatus::Enforced => ReleaseManifestDecision::ManifestEnforced,
        EnforcementStatus::MissingObservedCommandRoot => {
            ReleaseManifestDecision::HoldForObservedCommandRoot
        }
        EnforcementStatus::ExpectedObservedRootMismatch => {
            ReleaseManifestDecision::HoldForRootMismatch
        }
        EnforcementStatus::MissingActivationAdjudicatorRoot => {
            ReleaseManifestDecision::HoldForAdjudicatorBinding
        }
        EnforcementStatus::MissingManifestBindingRoot => {
            ReleaseManifestDecision::HoldForManifestBinding
        }
        EnforcementStatus::StaleFreshnessWindow => ReleaseManifestDecision::HoldForFreshness,
        EnforcementStatus::FailClosedHoldExported => ReleaseManifestDecision::FailClosedHold,
    }
}
fn hold_reason_for_status(status: EnforcementStatus) -> &'static str {
    match status {
        EnforcementStatus::Enforced => "release_manifest_enforced",
        EnforcementStatus::MissingObservedCommandRoot => "observed_replay_command_root_missing",
        EnforcementStatus::ExpectedObservedRootMismatch => {
            "expected_observed_replay_command_root_mismatch"
        }
        EnforcementStatus::MissingActivationAdjudicatorRoot => {
            "activation_adjudicator_root_missing"
        }
        EnforcementStatus::MissingManifestBindingRoot => "release_manifest_binding_root_missing",
        EnforcementStatus::StaleFreshnessWindow => "live_receipt_freshness_window_stale",
        EnforcementStatus::FailClosedHoldExported => "fail_closed_hold_exported",
    }
}
fn record_root(label: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-RELEASE-MANIFEST-ENFORCEMENT-RECORD-ROOT",
        &[HashPart::Str(label), HashPart::Json(record)],
        32,
    )
}
fn deterministic_root(label: &str, seed: &str) -> String {
    domain_hash(
        "MONERO-RELEASE-MANIFEST-ENFORCEMENT-DETERMINISTIC-ROOT",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(label),
            HashPart::Str(seed),
        ],
        32,
    )
}
fn expected_command_root(
    config: &Config,
    source_roots: &SourceRoots,
    lane: EnforcementLane,
) -> String {
    domain_hash(
        "MONERO-RELEASE-MANIFEST-ENFORCEMENT-EXPECTED-COMMAND-ROOT",
        &[
            HashPart::Str(&config.release_manifest_id),
            HashPart::Str(&source_roots.runtime_replay_activation_root),
            HashPart::Str(lane.as_str()),
            HashPart::Str(lane.command_name()),
        ],
        32,
    )
}
fn command_receipt_root(
    config: &Config,
    lane: EnforcementLane,
    observed_command_root: &str,
) -> String {
    domain_hash(
        "MONERO-RELEASE-MANIFEST-ENFORCEMENT-COMMAND-RECEIPT-ROOT",
        &[
            HashPart::Str(&config.force_exit_package_id),
            HashPart::Str(&config.release_manifest_id),
            HashPart::Str(lane.as_str()),
            HashPart::Str(observed_command_root),
        ],
        32,
    )
}
fn witness_quorum_root(config: &Config, lane: EnforcementLane, index: u64) -> String {
    domain_hash(
        "MONERO-RELEASE-MANIFEST-ENFORCEMENT-WITNESS-QUORUM-ROOT",
        &[
            HashPart::Str(&config.vertical_slice_id),
            HashPart::Str(&config.release_manifest_id),
            HashPart::Str(lane.as_str()),
            HashPart::U64(index),
        ],
        32,
    )
}
fn replay_command_id(
    lane: EnforcementLane,
    expected_command_root: &str,
    observed_command_root: &str,
    command_receipt_root: &str,
    witness_quorum_root: &str,
) -> String {
    domain_hash(
        "MONERO-RELEASE-MANIFEST-ENFORCEMENT-REPLAY-COMMAND-ID",
        &[
            HashPart::Str(lane.as_str()),
            HashPart::Str(expected_command_root),
            HashPart::Str(observed_command_root),
            HashPart::Str(command_receipt_root),
            HashPart::Str(witness_quorum_root),
        ],
        24,
    )
}
fn freshness_window_id(
    lane: EnforcementLane,
    l2_delta: u64,
    monero_delta: u64,
    accepted: bool,
) -> String {
    domain_hash(
        "MONERO-RELEASE-MANIFEST-ENFORCEMENT-FRESHNESS-WINDOW-ID",
        &[
            HashPart::Str(lane.as_str()),
            HashPart::U64(l2_delta),
            HashPart::U64(monero_delta),
            HashPart::Str(if accepted { "accepted" } else { "rejected" }),
        ],
        24,
    )
}
fn manifest_binding_root(
    config: &Config,
    source_roots: &SourceRoots,
    observation: &ReplayCommandObservation,
) -> String {
    domain_hash(
        "MONERO-RELEASE-MANIFEST-ENFORCEMENT-MANIFEST-BINDING-ROOT",
        &[
            HashPart::Str(&config.release_manifest_id),
            HashPart::Str(&source_roots.release_policy_manifest_binding_root),
            HashPart::Str(&source_roots.replacement_manifest_root),
            HashPart::Str(observation.lane.as_str()),
            HashPart::Str(&observation.expected_command_root),
            HashPart::Str(&observation.observed_command_root),
        ],
        32,
    )
}
fn activation_adjudicator_root(
    config: &Config,
    source_roots: &SourceRoots,
    lane: EnforcementLane,
) -> String {
    domain_hash(
        "MONERO-RELEASE-MANIFEST-ENFORCEMENT-ACTIVATION-ADJUDICATOR-ROOT",
        &[
            HashPart::Str(&config.release_manifest_id),
            HashPart::Str(&source_roots.release_policy_adjudicator_root),
            HashPart::Str(&source_roots.live_receipt_cross_domain_root),
            HashPart::Str(lane.as_str()),
        ],
        32,
    )
}
fn manifest_binding_id(
    lane: EnforcementLane,
    manifest_binding_root: &str,
    activation_adjudicator_root: &str,
    release_policy_manifest_root: &str,
) -> String {
    domain_hash(
        "MONERO-RELEASE-MANIFEST-ENFORCEMENT-MANIFEST-BINDING-ID",
        &[
            HashPart::Str(lane.as_str()),
            HashPart::Str(manifest_binding_root),
            HashPart::Str(activation_adjudicator_root),
            HashPart::Str(release_policy_manifest_root),
        ],
        24,
    )
}
fn hold_export_root(
    config: &Config,
    source_roots: &SourceRoots,
    lane: EnforcementLane,
    decision: ReleaseManifestDecision,
    status: EnforcementStatus,
) -> String {
    domain_hash(
        "MONERO-RELEASE-MANIFEST-ENFORCEMENT-HOLD-EXPORT-ROOT",
        &[
            HashPart::Str(&config.release_manifest_id),
            HashPart::Str(&source_roots.hold_export_bus_root),
            HashPart::Str(lane.as_str()),
            HashPart::Str(decision.as_str()),
            HashPart::Str(status.as_str()),
        ],
        32,
    )
}
fn hold_export_id(
    lane: EnforcementLane,
    decision: ReleaseManifestDecision,
    status: EnforcementStatus,
    hold_export_root: &str,
    hold_reason: &str,
) -> String {
    domain_hash(
        "MONERO-RELEASE-MANIFEST-ENFORCEMENT-HOLD-EXPORT-ID",
        &[
            HashPart::Str(lane.as_str()),
            HashPart::Str(decision.as_str()),
            HashPart::Str(status.as_str()),
            HashPart::Str(hold_export_root),
            HashPart::Str(hold_reason),
        ],
        24,
    )
}
fn enforcement_record_id(
    lane: EnforcementLane,
    command_id: &str,
    freshness_id: &str,
    binding_id: &str,
    status: EnforcementStatus,
    decision: ReleaseManifestDecision,
) -> String {
    domain_hash(
        "MONERO-RELEASE-MANIFEST-ENFORCEMENT-RECORD-ID",
        &[
            HashPart::Str(lane.as_str()),
            HashPart::Str(command_id),
            HashPart::Str(freshness_id),
            HashPart::Str(binding_id),
            HashPart::Str(status.as_str()),
            HashPart::Str(decision.as_str()),
        ],
        24,
    )
}
