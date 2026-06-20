use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_production_hold_go_no_go_readiness_runtime as readiness,
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageCompileHeavyGateReceiptAdapterRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_COMPILE_HEAVY_GATE_RECEIPT_ADAPTER_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-compile-heavy-gate-receipt-adapter-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_COMPILE_HEAVY_GATE_RECEIPT_ADAPTER_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const COMPILE_HEAVY_GATE_RECEIPT_ADAPTER_SUITE: &str =
    "monero-l2-pq-force-exit-package-compile-heavy-gate-receipt-adapter-v1";
pub const DEFAULT_MIN_RECEIPT_FAMILIES: u64 = 6;
pub const DEFAULT_MIN_LIVE_REPLACEMENTS: u64 = 6;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub adapter_suite: String,
    pub min_receipt_families: u64,
    pub min_live_replacements: u64,
    pub require_cargo_check_receipts: bool,
    pub require_cargo_test_receipts: bool,
    pub require_clippy_receipts: bool,
    pub require_rustfmt_confirmation: bool,
    pub require_dependency_lock_receipts: bool,
    pub require_toolchain_pin_receipts: bool,
    pub production_hold_on_missing_receipts: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            adapter_suite: COMPILE_HEAVY_GATE_RECEIPT_ADAPTER_SUITE.to_string(),
            min_receipt_families: DEFAULT_MIN_RECEIPT_FAMILIES,
            min_live_replacements: DEFAULT_MIN_LIVE_REPLACEMENTS,
            require_cargo_check_receipts: true,
            require_cargo_test_receipts: true,
            require_clippy_receipts: true,
            require_rustfmt_confirmation: true,
            require_dependency_lock_receipts: true,
            require_toolchain_pin_receipts: true,
            production_hold_on_missing_receipts: true,
        }
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }

    pub fn required_family_count(&self) -> u64 {
        [
            self.require_cargo_check_receipts,
            self.require_cargo_test_receipts,
            self.require_clippy_receipts,
            self.require_rustfmt_confirmation,
            self.require_dependency_lock_receipts,
            self.require_toolchain_pin_receipts,
        ]
        .iter()
        .filter(|required| **required)
        .count() as u64
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "adapter_suite": self.adapter_suite,
            "min_receipt_families": self.min_receipt_families,
            "min_live_replacements": self.min_live_replacements,
            "required_family_count": self.required_family_count(),
            "require_cargo_check_receipts": self.require_cargo_check_receipts,
            "require_cargo_test_receipts": self.require_cargo_test_receipts,
            "require_clippy_receipts": self.require_clippy_receipts,
            "require_rustfmt_confirmation": self.require_rustfmt_confirmation,
            "require_dependency_lock_receipts": self.require_dependency_lock_receipts,
            "require_toolchain_pin_receipts": self.require_toolchain_pin_receipts,
            "production_hold_on_missing_receipts": self.production_hold_on_missing_receipts,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptFamily {
    CargoCheck,
    CargoTest,
    Clippy,
    Rustfmt,
    DependencyLock,
    ToolchainPin,
}

impl ReceiptFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CargoCheck => "cargo_check",
            Self::CargoTest => "cargo_test",
            Self::Clippy => "clippy",
            Self::Rustfmt => "rustfmt",
            Self::DependencyLock => "dependency_lock",
            Self::ToolchainPin => "toolchain_pin",
        }
    }

    pub fn artifact_label(self) -> &'static str {
        match self {
            Self::CargoCheck => "cargo-check receipt roots",
            Self::CargoTest => "cargo-test receipt roots",
            Self::Clippy => "clippy receipt roots",
            Self::Rustfmt => "rustfmt confirmation roots",
            Self::DependencyLock => "dependency lock roots",
            Self::ToolchainPin => "toolchain pin roots",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::CargoCheck,
            Self::CargoTest,
            Self::Clippy,
            Self::Rustfmt,
            Self::DependencyLock,
            Self::ToolchainPin,
        ]
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterStatus {
    LiveReceiptObserved,
    DeferredToLiveReplacement,
    MissingReceiptHold,
}

impl AdapterStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LiveReceiptObserved => "live_receipt_observed",
            Self::DeferredToLiveReplacement => "deferred_to_live_replacement",
            Self::MissingReceiptHold => "missing_receipt_hold",
        }
    }

    pub fn is_live(self) -> bool {
        self == Self::LiveReceiptObserved
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceBundle {
    pub readiness_state_root: String,
    pub readiness_config_root: String,
    pub readiness_evidence_packet_root: String,
    pub readiness_live_evidence_root: String,
    pub readiness_deferred_evidence_root: String,
    pub readiness_decision_root: String,
    pub readiness_production_hold_root: String,
    pub readiness_status: String,
    pub readiness_release_memo: String,
    pub readiness_production_go: bool,
    pub readiness_production_hold: bool,
    pub readiness_total_tracks: u64,
    pub readiness_live_tracks: u64,
    pub readiness_deferred_tracks: u64,
    pub readiness_blocked_tracks: u64,
}

impl SourceBundle {
    pub fn from_readiness(state: &readiness::State) -> Self {
        Self {
            readiness_state_root: state.state_root(),
            readiness_config_root: state.roots.config_root.clone(),
            readiness_evidence_packet_root: state.roots.evidence_packet_root.clone(),
            readiness_live_evidence_root: state.roots.live_evidence_root.clone(),
            readiness_deferred_evidence_root: state.roots.deferred_evidence_root.clone(),
            readiness_decision_root: state.roots.decision_root.clone(),
            readiness_production_hold_root: state.roots.production_hold_root.clone(),
            readiness_status: state.decision.go_no_go_status.clone(),
            readiness_release_memo: state.decision.release_memo.clone(),
            readiness_production_go: state.decision.production_go,
            readiness_production_hold: state.decision.production_hold,
            readiness_total_tracks: state.counters.total_tracks,
            readiness_live_tracks: state.counters.live_tracks,
            readiness_deferred_tracks: state.counters.deferred_tracks,
            readiness_blocked_tracks: state.counters.blocked_tracks,
        }
    }

    pub fn devnet() -> Self {
        Self::from_readiness(&readiness::devnet())
    }

    pub fn public_record(&self) -> Value {
        json!({
            "readiness_state_root": self.readiness_state_root,
            "readiness_config_root": self.readiness_config_root,
            "readiness_evidence_packet_root": self.readiness_evidence_packet_root,
            "readiness_live_evidence_root": self.readiness_live_evidence_root,
            "readiness_deferred_evidence_root": self.readiness_deferred_evidence_root,
            "readiness_decision_root": self.readiness_decision_root,
            "readiness_production_hold_root": self.readiness_production_hold_root,
            "readiness_status": self.readiness_status,
            "readiness_release_memo": self.readiness_release_memo,
            "readiness_production_go": self.readiness_production_go,
            "readiness_production_hold": self.readiness_production_hold,
            "readiness_total_tracks": self.readiness_total_tracks,
            "readiness_live_tracks": self.readiness_live_tracks,
            "readiness_deferred_tracks": self.readiness_deferred_tracks,
            "readiness_blocked_tracks": self.readiness_blocked_tracks,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CompileHeavyGateReceipt {
    pub receipt_id: String,
    pub family: ReceiptFamily,
    pub ordinal: u64,
    pub artifact_label: String,
    pub expected_receipt_root: String,
    pub observed_receipt_root: String,
    pub deferred_replacement_root: String,
    pub production_hold_root: String,
    pub adapter_receipt_root: String,
    pub status: AdapterStatus,
    pub live_replacement_required: bool,
    pub production_blocked: bool,
    pub adapter_note: String,
}

impl CompileHeavyGateReceipt {
    pub fn devnet(
        config: &Config,
        source: &SourceBundle,
        family: ReceiptFamily,
        ordinal: u64,
    ) -> Self {
        let expected_receipt_root = expected_receipt_root(config, source, family, ordinal);
        let status = devnet_status(family);
        let observed_receipt_root =
            observed_receipt_root(config, source, family, status, &expected_receipt_root);
        let deferred_replacement_root =
            deferred_replacement_root(config, source, family, status, &expected_receipt_root);
        let production_blocked = config.production_hold_on_missing_receipts
            && status != AdapterStatus::LiveReceiptObserved;
        let production_hold_root =
            production_hold_root(config, source, family, status, production_blocked);
        let adapter_receipt_root = adapter_receipt_root(
            config,
            source,
            family,
            ordinal,
            status,
            &expected_receipt_root,
            &observed_receipt_root,
            &deferred_replacement_root,
            &production_hold_root,
            production_blocked,
        );
        let receipt_id = receipt_id(family, ordinal, &adapter_receipt_root);
        Self {
            receipt_id,
            family,
            ordinal,
            artifact_label: family.artifact_label().to_string(),
            expected_receipt_root,
            observed_receipt_root,
            deferred_replacement_root,
            production_hold_root,
            adapter_receipt_root,
            status,
            live_replacement_required: status != AdapterStatus::LiveReceiptObserved,
            production_blocked,
            adapter_note: adapter_note(status, family).to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "receipt_id": self.receipt_id,
            "family": self.family.as_str(),
            "ordinal": self.ordinal,
            "artifact_label": self.artifact_label,
            "expected_receipt_root": self.expected_receipt_root,
            "observed_receipt_root": self.observed_receipt_root,
            "deferred_replacement_root": self.deferred_replacement_root,
            "production_hold_root": self.production_hold_root,
            "adapter_receipt_root": self.adapter_receipt_root,
            "status": self.status.as_str(),
            "live_replacement_required": self.live_replacement_required,
            "production_blocked": self.production_blocked,
            "adapter_note": self.adapter_note,
        })
    }

    pub fn state_root(&self) -> String {
        self.adapter_receipt_root.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub total_receipt_families: u64,
    pub live_receipt_families: u64,
    pub deferred_replacement_families: u64,
    pub missing_hold_families: u64,
    pub production_blocker_families: u64,
    pub required_receipt_families: u64,
    pub required_live_replacements: u64,
}

impl Counters {
    pub fn from_receipts(config: &Config, receipts: &[CompileHeavyGateReceipt]) -> Self {
        Self {
            total_receipt_families: receipts.len() as u64,
            live_receipt_families: receipts
                .iter()
                .filter(|receipt| receipt.status.is_live())
                .count() as u64,
            deferred_replacement_families: receipts
                .iter()
                .filter(|receipt| receipt.status == AdapterStatus::DeferredToLiveReplacement)
                .count() as u64,
            missing_hold_families: receipts
                .iter()
                .filter(|receipt| receipt.status == AdapterStatus::MissingReceiptHold)
                .count() as u64,
            production_blocker_families: receipts
                .iter()
                .filter(|receipt| receipt.production_blocked)
                .count() as u64,
            required_receipt_families: config
                .required_family_count()
                .max(config.min_receipt_families),
            required_live_replacements: config.min_live_replacements,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "total_receipt_families": self.total_receipt_families,
            "live_receipt_families": self.live_receipt_families,
            "deferred_replacement_families": self.deferred_replacement_families,
            "missing_hold_families": self.missing_hold_families,
            "production_blocker_families": self.production_blocker_families,
            "required_receipt_families": self.required_receipt_families,
            "required_live_replacements": self.required_live_replacements,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdapterVerdict {
    pub verdict_id: String,
    pub deferred_to_live_replacement: bool,
    pub production_hold: bool,
    pub replacement_verdict: String,
    pub production_answer: String,
    pub user_escape_answer: String,
}

impl AdapterVerdict {
    pub fn from_parts(config: &Config, source: &SourceBundle, counters: &Counters) -> Self {
        let enough_families = counters.total_receipt_families >= counters.required_receipt_families;
        let enough_live = counters.live_receipt_families >= counters.required_live_replacements;
        let deferred_to_live_replacement = counters.deferred_replacement_families > 0
            || counters.missing_hold_families > 0
            || !enough_live;
        let production_hold = config.production_hold_on_missing_receipts
            && (deferred_to_live_replacement
                || counters.production_blocker_families > 0
                || source.readiness_production_hold
                || !enough_families);
        let replacement_verdict = if deferred_to_live_replacement {
            "deferred_to_live_replacement_required"
        } else {
            "all_compile_heavy_gate_receipts_live_observed"
        }
        .to_string();
        let production_answer = if production_hold {
            "production hold: compile-heavy gate receipts are missing, deferred, or not live-observed"
        } else {
            "production allowed: compile-heavy gate receipts have live replacements"
        }
        .to_string();
        let user_escape_answer = if production_hold {
            "user escape package remains answerable while compile-heavy receipts are replaced with live evidence"
        } else {
            "user escape package has compile-heavy gate receipt coverage"
        }
        .to_string();
        let verdict_id = verdict_id(
            config,
            source,
            counters,
            deferred_to_live_replacement,
            production_hold,
            &replacement_verdict,
        );
        Self {
            verdict_id,
            deferred_to_live_replacement,
            production_hold,
            replacement_verdict,
            production_answer,
            user_escape_answer,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "verdict_id": self.verdict_id,
            "deferred_to_live_replacement": self.deferred_to_live_replacement,
            "production_hold": self.production_hold,
            "replacement_verdict": self.replacement_verdict,
            "production_answer": self.production_answer,
            "user_escape_answer": self.user_escape_answer,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Roots {
    pub config_root: String,
    pub source_root: String,
    pub receipt_root: String,
    pub cargo_check_receipt_root: String,
    pub cargo_test_receipt_root: String,
    pub clippy_receipt_root: String,
    pub rustfmt_confirmation_root: String,
    pub dependency_lock_root: String,
    pub toolchain_pin_root: String,
    pub deferred_to_live_replacement_root: String,
    pub production_hold_root: String,
    pub verdict_root: String,
    pub state_root: String,
}

impl Roots {
    pub fn from_parts(
        config: &Config,
        source: &SourceBundle,
        receipts: &[CompileHeavyGateReceipt],
        counters: &Counters,
        verdict: &AdapterVerdict,
    ) -> Self {
        let receipt_records = receipts
            .iter()
            .map(CompileHeavyGateReceipt::public_record)
            .collect::<Vec<_>>();
        let config_root = config.state_root();
        let source_root = source.state_root();
        let receipt_root = merkle_root("COMPILE-HEAVY-GATE-ADAPTER-RECEIPTS", &receipt_records);
        let cargo_check_receipt_root = family_root(receipts, ReceiptFamily::CargoCheck);
        let cargo_test_receipt_root = family_root(receipts, ReceiptFamily::CargoTest);
        let clippy_receipt_root = family_root(receipts, ReceiptFamily::Clippy);
        let rustfmt_confirmation_root = family_root(receipts, ReceiptFamily::Rustfmt);
        let dependency_lock_root = family_root(receipts, ReceiptFamily::DependencyLock);
        let toolchain_pin_root = family_root(receipts, ReceiptFamily::ToolchainPin);
        let deferred_to_live_replacement_root =
            deferred_to_live_replacement_root(config, source, receipts, counters, verdict);
        let production_hold_root = production_hold_bundle_root(config, source, receipts, verdict);
        let verdict_root = record_root("verdict", &verdict.public_record());
        let state_root = record_root(
            "state",
            &json!({
                "config_root": config_root,
                "source_root": source_root,
                "receipt_root": receipt_root,
                "cargo_check_receipt_root": cargo_check_receipt_root,
                "cargo_test_receipt_root": cargo_test_receipt_root,
                "clippy_receipt_root": clippy_receipt_root,
                "rustfmt_confirmation_root": rustfmt_confirmation_root,
                "dependency_lock_root": dependency_lock_root,
                "toolchain_pin_root": toolchain_pin_root,
                "deferred_to_live_replacement_root": deferred_to_live_replacement_root,
                "production_hold_root": production_hold_root,
                "counters": counters.public_record(),
                "verdict_root": verdict_root,
            }),
        );
        Self {
            config_root,
            source_root,
            receipt_root,
            cargo_check_receipt_root,
            cargo_test_receipt_root,
            clippy_receipt_root,
            rustfmt_confirmation_root,
            dependency_lock_root,
            toolchain_pin_root,
            deferred_to_live_replacement_root,
            production_hold_root,
            verdict_root,
            state_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "config_root": self.config_root,
            "source_root": self.source_root,
            "receipt_root": self.receipt_root,
            "cargo_check_receipt_root": self.cargo_check_receipt_root,
            "cargo_test_receipt_root": self.cargo_test_receipt_root,
            "clippy_receipt_root": self.clippy_receipt_root,
            "rustfmt_confirmation_root": self.rustfmt_confirmation_root,
            "dependency_lock_root": self.dependency_lock_root,
            "toolchain_pin_root": self.toolchain_pin_root,
            "deferred_to_live_replacement_root": self.deferred_to_live_replacement_root,
            "production_hold_root": self.production_hold_root,
            "verdict_root": self.verdict_root,
            "state_root": self.state_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub source: SourceBundle,
    pub receipts: Vec<CompileHeavyGateReceipt>,
    pub counters: Counters,
    pub verdict: AdapterVerdict,
    pub roots: Roots,
}

impl State {
    pub fn new(
        config: Config,
        source: SourceBundle,
        receipts: Vec<CompileHeavyGateReceipt>,
    ) -> Result<Self> {
        validate_config(&config)?;
        validate_source(&source)?;
        validate_receipts(&receipts)?;
        let counters = Counters::from_receipts(&config, &receipts);
        let verdict = AdapterVerdict::from_parts(&config, &source, &counters);
        let roots = Roots::from_parts(&config, &source, &receipts, &counters, &verdict);
        Ok(Self {
            config,
            source,
            receipts,
            counters,
            verdict,
            roots,
        })
    }

    pub fn devnet() -> Self {
        let config = Config::devnet();
        let source = SourceBundle::devnet();
        let receipts = devnet_receipts(&config, &source);
        match Self::new(config, source, receipts) {
            Ok(state) => state,
            Err(reason) => fallback_state(reason),
        }
    }

    pub fn production_hold(&self) -> bool {
        self.verdict.production_hold
    }

    pub fn public_record(&self) -> Value {
        json!({
            "config": self.config.public_record(),
            "source": self.source.public_record(),
            "receipts": self
                .receipts
                .iter()
                .map(CompileHeavyGateReceipt::public_record)
                .collect::<Vec<_>>(),
            "counters": self.counters.public_record(),
            "verdict": self.verdict.public_record(),
            "roots": self.roots.public_record(),
            "production_hold": self.production_hold(),
        })
    }

    pub fn state_root(&self) -> String {
        self.roots.state_root.clone()
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

fn devnet_receipts(config: &Config, source: &SourceBundle) -> Vec<CompileHeavyGateReceipt> {
    ReceiptFamily::all()
        .into_iter()
        .enumerate()
        .map(|(index, family)| {
            CompileHeavyGateReceipt::devnet(config, source, family, index as u64 + 1)
        })
        .collect()
}

fn devnet_status(family: ReceiptFamily) -> AdapterStatus {
    match family {
        ReceiptFamily::DependencyLock | ReceiptFamily::ToolchainPin => {
            AdapterStatus::LiveReceiptObserved
        }
        ReceiptFamily::Rustfmt => AdapterStatus::DeferredToLiveReplacement,
        ReceiptFamily::CargoCheck | ReceiptFamily::CargoTest | ReceiptFamily::Clippy => {
            AdapterStatus::MissingReceiptHold
        }
    }
}

fn expected_receipt_root(
    config: &Config,
    source: &SourceBundle,
    family: ReceiptFamily,
    ordinal: u64,
) -> String {
    record_root(
        "expected-receipt",
        &json!({
            "adapter_suite": config.adapter_suite,
            "readiness_state_root": source.readiness_state_root,
            "family": family.as_str(),
            "ordinal": ordinal,
            "artifact_label": family.artifact_label(),
        }),
    )
}

fn observed_receipt_root(
    config: &Config,
    source: &SourceBundle,
    family: ReceiptFamily,
    status: AdapterStatus,
    expected_receipt_root: &str,
) -> String {
    record_root(
        "observed-receipt",
        &json!({
            "adapter_suite": config.adapter_suite,
            "readiness_live_evidence_root": source.readiness_live_evidence_root,
            "family": family.as_str(),
            "expected_receipt_root": expected_receipt_root,
            "status": status.as_str(),
            "observed": status == AdapterStatus::LiveReceiptObserved,
        }),
    )
}

fn deferred_replacement_root(
    config: &Config,
    source: &SourceBundle,
    family: ReceiptFamily,
    status: AdapterStatus,
    expected_receipt_root: &str,
) -> String {
    record_root(
        "deferred-replacement",
        &json!({
            "adapter_suite": config.adapter_suite,
            "readiness_deferred_evidence_root": source.readiness_deferred_evidence_root,
            "family": family.as_str(),
            "expected_receipt_root": expected_receipt_root,
            "status": status.as_str(),
            "replacement_required": status != AdapterStatus::LiveReceiptObserved,
            "verdict": if status == AdapterStatus::LiveReceiptObserved {
                "live_receipt_accepted"
            } else {
                "deferred_to_live_replacement"
            },
        }),
    )
}

fn production_hold_root(
    config: &Config,
    source: &SourceBundle,
    family: ReceiptFamily,
    status: AdapterStatus,
    production_blocked: bool,
) -> String {
    record_root(
        "production-hold",
        &json!({
            "adapter_suite": config.adapter_suite,
            "source_production_hold_root": source.readiness_production_hold_root,
            "family": family.as_str(),
            "status": status.as_str(),
            "production_blocked": production_blocked,
            "policy": "production hold remains active when compile-heavy gate receipts are missing",
        }),
    )
}

fn adapter_receipt_root(
    config: &Config,
    source: &SourceBundle,
    family: ReceiptFamily,
    ordinal: u64,
    status: AdapterStatus,
    expected_receipt_root: &str,
    observed_receipt_root: &str,
    deferred_replacement_root: &str,
    production_hold_root: &str,
    production_blocked: bool,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-COMPILE-HEAVY-GATE-RECEIPT-ADAPTER",
        &[
            HashPart::Str(&config.adapter_suite),
            HashPart::Str(&source.readiness_state_root),
            HashPart::Str(family.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(status.as_str()),
            HashPart::Str(expected_receipt_root),
            HashPart::Str(observed_receipt_root),
            HashPart::Str(deferred_replacement_root),
            HashPart::Str(production_hold_root),
            HashPart::Str(bool_str(production_blocked)),
        ],
        32,
    )
}

fn receipt_id(family: ReceiptFamily, ordinal: u64, adapter_receipt_root: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-COMPILE-HEAVY-GATE-RECEIPT-ADAPTER-ID",
        &[
            HashPart::Str(family.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(adapter_receipt_root),
        ],
        16,
    )
}

fn family_root(receipts: &[CompileHeavyGateReceipt], family: ReceiptFamily) -> String {
    let records = receipts
        .iter()
        .filter(|receipt| receipt.family == family)
        .map(CompileHeavyGateReceipt::public_record)
        .collect::<Vec<_>>();
    merkle_root(
        &format!("COMPILE-HEAVY-GATE-{}-RECEIPTS", family.as_str()),
        &records,
    )
}

fn deferred_to_live_replacement_root(
    config: &Config,
    source: &SourceBundle,
    receipts: &[CompileHeavyGateReceipt],
    counters: &Counters,
    verdict: &AdapterVerdict,
) -> String {
    let records = receipts
        .iter()
        .filter(|receipt| receipt.live_replacement_required)
        .map(|receipt| {
            json!({
                "receipt_id": receipt.receipt_id,
                "family": receipt.family.as_str(),
                "deferred_replacement_root": receipt.deferred_replacement_root,
                "status": receipt.status.as_str(),
            })
        })
        .collect::<Vec<_>>();
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-COMPILE-HEAVY-GATE-DEFERRED-LIVE-REPLACEMENT",
        &[
            HashPart::Str(&config.adapter_suite),
            HashPart::Str(&source.readiness_deferred_evidence_root),
            HashPart::Str(&merkle_root(
                "COMPILE-HEAVY-GATE-DEFERRED-LIVE-REPLACEMENT-LEAVES",
                &records,
            )),
            HashPart::U64(counters.deferred_replacement_families),
            HashPart::U64(counters.missing_hold_families),
            HashPart::Str(bool_str(verdict.deferred_to_live_replacement)),
        ],
        32,
    )
}

fn production_hold_bundle_root(
    config: &Config,
    source: &SourceBundle,
    receipts: &[CompileHeavyGateReceipt],
    verdict: &AdapterVerdict,
) -> String {
    let records = receipts
        .iter()
        .filter(|receipt| receipt.production_blocked)
        .map(|receipt| {
            json!({
                "receipt_id": receipt.receipt_id,
                "family": receipt.family.as_str(),
                "production_hold_root": receipt.production_hold_root,
                "status": receipt.status.as_str(),
            })
        })
        .collect::<Vec<_>>();
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-COMPILE-HEAVY-GATE-PRODUCTION-HOLD",
        &[
            HashPart::Str(&config.adapter_suite),
            HashPart::Str(&source.readiness_production_hold_root),
            HashPart::Str(&merkle_root(
                "COMPILE-HEAVY-GATE-PRODUCTION-HOLD-LEAVES",
                &records,
            )),
            HashPart::Str(bool_str(verdict.production_hold)),
            HashPart::Str(&verdict.production_answer),
        ],
        32,
    )
}

fn verdict_id(
    config: &Config,
    source: &SourceBundle,
    counters: &Counters,
    deferred_to_live_replacement: bool,
    production_hold: bool,
    replacement_verdict: &str,
) -> String {
    let counters_record = counters.public_record();
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-COMPILE-HEAVY-GATE-ADAPTER-VERDICT",
        &[
            HashPart::Str(&config.adapter_suite),
            HashPart::Str(&source.readiness_decision_root),
            HashPart::Json(&counters_record),
            HashPart::Str(bool_str(deferred_to_live_replacement)),
            HashPart::Str(bool_str(production_hold)),
            HashPart::Str(replacement_verdict),
        ],
        32,
    )
}

fn adapter_note(status: AdapterStatus, family: ReceiptFamily) -> &'static str {
    match status {
        AdapterStatus::LiveReceiptObserved => {
            "compile-heavy gate family has an accepted live receipt root"
        }
        AdapterStatus::DeferredToLiveReplacement => match family {
            ReceiptFamily::Rustfmt => {
                "rustfmt confirmation root is deterministic but deferred to live replacement"
            }
            _ => "compile-heavy gate family is deferred until live replacement arrives",
        },
        AdapterStatus::MissingReceiptHold => {
            "production hold stays active because this compile-heavy gate receipt is missing"
        }
    }
}

fn validate_config(config: &Config) -> Result<()> {
    ensure(
        config.chain_id == CHAIN_ID,
        "compile-heavy adapter chain mismatch",
    )?;
    ensure(
        config.protocol_version == PROTOCOL_VERSION,
        "compile-heavy adapter protocol mismatch",
    )?;
    ensure(
        config.min_receipt_families > 0,
        "compile-heavy adapter requires at least one receipt family",
    )?;
    ensure(
        config.min_live_replacements > 0,
        "compile-heavy adapter requires live replacement threshold",
    )?;
    Ok(())
}

fn validate_source(source: &SourceBundle) -> Result<()> {
    ensure(
        !source.readiness_state_root.is_empty(),
        "compile-heavy adapter missing readiness state root",
    )?;
    ensure(
        !source.readiness_production_hold_root.is_empty(),
        "compile-heavy adapter missing readiness production hold root",
    )?;
    Ok(())
}

fn validate_receipts(receipts: &[CompileHeavyGateReceipt]) -> Result<()> {
    ensure(
        !receipts.is_empty(),
        "compile-heavy adapter requires receipt families",
    )
}

fn fallback_state(reason: String) -> State {
    let config = Config::default();
    let source = SourceBundle::devnet();
    let receipt = CompileHeavyGateReceipt {
        receipt_id: record_root("fallback-receipt-id", &json!({"reason": &reason})),
        family: ReceiptFamily::CargoCheck,
        ordinal: 1,
        artifact_label: "cargo-check receipt roots".to_string(),
        expected_receipt_root: record_root("fallback-expected", &json!({"reason": &reason})),
        observed_receipt_root: record_root("fallback-observed", &json!({"reason": &reason})),
        deferred_replacement_root: record_root("fallback-deferred", &json!({"reason": &reason})),
        production_hold_root: record_root("fallback-hold", &json!({"reason": &reason})),
        adapter_receipt_root: record_root("fallback-adapter", &json!({"reason": &reason})),
        status: AdapterStatus::MissingReceiptHold,
        live_replacement_required: true,
        production_blocked: true,
        adapter_note: reason,
    };
    let receipts = vec![receipt];
    let counters = Counters::from_receipts(&config, &receipts);
    let verdict = AdapterVerdict::from_parts(&config, &source, &counters);
    let roots = Roots::from_parts(&config, &source, &receipts, &counters, &verdict);
    State {
        config,
        source,
        receipts,
        counters,
        verdict,
        roots,
    }
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-COMPILE-HEAVY-GATE-RECEIPT-ADAPTER-RECORD",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(kind),
            HashPart::Json(record),
        ],
        32,
    )
}

fn bool_str(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}
