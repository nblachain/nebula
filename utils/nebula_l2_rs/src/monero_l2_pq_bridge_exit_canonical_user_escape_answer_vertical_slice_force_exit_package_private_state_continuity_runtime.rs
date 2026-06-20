use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_recovery_playbook_receipt_runtime as recovery,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_settlement_observation_runtime as settlement,
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_wallet_scan_receipt_observer_runtime as wallet_scan,
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackagePrivateStateContinuityRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_PRIVATE_STATE_CONTINUITY_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-private-state-continuity-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_PRIVATE_STATE_CONTINUITY_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const PRIVATE_STATE_CONTINUITY_SUITE: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-private-state-continuity-v1";
pub const DEFAULT_MIN_CONTINUITY_RECORDS: u64 = 6;
pub const DEFAULT_NULLIFIER_FENCE_DEPTH: u64 = 4;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub continuity_suite: String,
    pub min_continuity_records: u64,
    pub nullifier_fence_depth: u64,
    pub require_wallet_scan_privacy_roots: bool,
    pub require_nullifier_fences: bool,
    pub require_settlement_observations: bool,
    pub require_recovery_receipts: bool,
    pub require_linkage_blinding: bool,
    pub hold_production_until_continuity_proven: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            continuity_suite: PRIVATE_STATE_CONTINUITY_SUITE.to_string(),
            min_continuity_records: DEFAULT_MIN_CONTINUITY_RECORDS,
            nullifier_fence_depth: DEFAULT_NULLIFIER_FENCE_DEPTH,
            require_wallet_scan_privacy_roots: true,
            require_nullifier_fences: true,
            require_settlement_observations: true,
            require_recovery_receipts: true,
            require_linkage_blinding: true,
            hold_production_until_continuity_proven: true,
        }
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self::default()
    }

    pub fn public_record(&self) -> Value {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "continuity_suite": self.continuity_suite,
            "min_continuity_records": self.min_continuity_records,
            "nullifier_fence_depth": self.nullifier_fence_depth,
            "require_wallet_scan_privacy_roots": self.require_wallet_scan_privacy_roots,
            "require_nullifier_fences": self.require_nullifier_fences,
            "require_settlement_observations": self.require_settlement_observations,
            "require_recovery_receipts": self.require_recovery_receipts,
            "require_linkage_blinding": self.require_linkage_blinding,
            "hold_production_until_continuity_proven": self.hold_production_until_continuity_proven,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceBundle {
    pub wallet_scan_state_root: String,
    pub wallet_scan_receipt_root: String,
    pub encrypted_scan_bundle_root: String,
    pub nullifier_fence_bundle_root: String,
    pub metadata_redaction_bundle_root: String,
    pub linkage_privacy_bundle_root: String,
    pub wallet_user_escape_ready: bool,
    pub wallet_linkage_hidden: bool,
    pub settlement_state_root: String,
    pub settlement_observation_root: String,
    pub settlement_evidence_bundle_root: String,
    pub settlement_observed: bool,
    pub recovery_state_root: String,
    pub recovery_step_root: String,
    pub recovery_evidence_bundle_root: String,
    pub recovery_fail_closed_hold_root: String,
    pub wallet_scan_receipt_count: u64,
    pub settlement_observation_count: u64,
    pub recovery_step_count: u64,
}

impl SourceBundle {
    pub fn from_states(
        wallet_scan_state: &wallet_scan::State,
        settlement_state: &settlement::State,
        recovery_state: &recovery::State,
    ) -> Self {
        Self {
            wallet_scan_state_root: wallet_scan_state.state_root(),
            wallet_scan_receipt_root: wallet_scan_state.wallet_scan_receipt_root.clone(),
            encrypted_scan_bundle_root: wallet_scan_state.encrypted_scan_bundle_root.clone(),
            nullifier_fence_bundle_root: wallet_scan_state.nullifier_fence_bundle_root.clone(),
            metadata_redaction_bundle_root: wallet_scan_state
                .metadata_redaction_bundle_root
                .clone(),
            linkage_privacy_bundle_root: wallet_scan_state.linkage_privacy_bundle_root.clone(),
            wallet_user_escape_ready: wallet_scan_state.verdict.user_escape_ready,
            wallet_linkage_hidden: wallet_scan_state.verdict.linkage_hidden,
            settlement_state_root: settlement_state.state_root(),
            settlement_observation_root: settlement_state.settlement_observation_root.clone(),
            settlement_evidence_bundle_root: settlement_state
                .settlement_evidence_bundle_root
                .clone(),
            settlement_observed: settlement_state.verdict.settlement_observed,
            recovery_state_root: recovery_state.state_root(),
            recovery_step_root: recovery_state.recovery_step_root.clone(),
            recovery_evidence_bundle_root: recovery_state.evidence_bundle_root.clone(),
            recovery_fail_closed_hold_root: recovery_state.fail_closed_hold_root.clone(),
            wallet_scan_receipt_count: wallet_scan_state.verdict.scan_receipt_count,
            settlement_observation_count: settlement_state.verdict.settlement_observation_count,
            recovery_step_count: recovery_state.verdict.recovery_step_count,
        }
    }

    pub fn devnet() -> Self {
        Self::from_states(
            &wallet_scan::devnet(),
            &settlement::devnet(),
            &recovery::devnet(),
        )
    }

    pub fn public_record(&self) -> Value {
        json!({
            "wallet_scan_state_root": self.wallet_scan_state_root,
            "wallet_scan_receipt_root": self.wallet_scan_receipt_root,
            "encrypted_scan_bundle_root": self.encrypted_scan_bundle_root,
            "nullifier_fence_bundle_root": self.nullifier_fence_bundle_root,
            "metadata_redaction_bundle_root": self.metadata_redaction_bundle_root,
            "linkage_privacy_bundle_root": self.linkage_privacy_bundle_root,
            "wallet_user_escape_ready": self.wallet_user_escape_ready,
            "wallet_linkage_hidden": self.wallet_linkage_hidden,
            "settlement_state_root": self.settlement_state_root,
            "settlement_observation_root": self.settlement_observation_root,
            "settlement_evidence_bundle_root": self.settlement_evidence_bundle_root,
            "settlement_observed": self.settlement_observed,
            "recovery_state_root": self.recovery_state_root,
            "recovery_step_root": self.recovery_step_root,
            "recovery_evidence_bundle_root": self.recovery_evidence_bundle_root,
            "recovery_fail_closed_hold_root": self.recovery_fail_closed_hold_root,
            "wallet_scan_receipt_count": self.wallet_scan_receipt_count,
            "settlement_observation_count": self.settlement_observation_count,
            "recovery_step_count": self.recovery_step_count,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source-bundle", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityRecordKind {
    WalletScanPrivacyRoots,
    NullifierFence,
    SettlementObservation,
    RecoveryReceipt,
    LinkageBlind,
    PrivateStateCarryForward,
}

impl ContinuityRecordKind {
    pub fn ordered() -> &'static [Self] {
        &[
            Self::WalletScanPrivacyRoots,
            Self::NullifierFence,
            Self::SettlementObservation,
            Self::RecoveryReceipt,
            Self::LinkageBlind,
            Self::PrivateStateCarryForward,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::WalletScanPrivacyRoots => "wallet_scan_privacy_roots",
            Self::NullifierFence => "nullifier_fence",
            Self::SettlementObservation => "settlement_observation",
            Self::RecoveryReceipt => "recovery_receipt",
            Self::LinkageBlind => "linkage_blind",
            Self::PrivateStateCarryForward => "private_state_carry_forward",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityStatus {
    Proven,
    PendingSettlement,
    HeldFailClosed,
    BlockedByMissingFence,
}

impl ContinuityStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Proven => "proven",
            Self::PendingSettlement => "pending_settlement",
            Self::HeldFailClosed => "held_fail_closed",
            Self::BlockedByMissingFence => "blocked_by_missing_fence",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ContinuityRecord {
    pub record_id: String,
    pub ordinal: u64,
    pub record_kind: ContinuityRecordKind,
    pub wallet_privacy_root: String,
    pub nullifier_fence_root: String,
    pub settlement_observation_root: String,
    pub recovery_receipt_root: String,
    pub unlinkability_witness_root: String,
    pub private_state_carry_root: String,
    pub status: ContinuityStatus,
    pub privacy_root_bound: bool,
    pub nullifier_fenced: bool,
    pub settlement_observed: bool,
    pub recovery_receipt_bound: bool,
    pub linkage_hidden: bool,
    pub user_escape_continuity: bool,
    pub production_blocked: bool,
    pub continuity_record_root: String,
}

impl ContinuityRecord {
    pub fn from_kind(
        config: &Config,
        source: &SourceBundle,
        record_kind: ContinuityRecordKind,
        ordinal: u64,
    ) -> Self {
        let privacy_root_bound = privacy_root_bound(config, source, record_kind);
        let nullifier_fenced = nullifier_fenced(config, source, record_kind);
        let settlement_observed = settlement_observed(config, source, record_kind);
        let recovery_receipt_bound = recovery_receipt_bound(config, source, record_kind);
        let linkage_hidden = linkage_hidden(config, source, record_kind);
        let status = continuity_status(
            nullifier_fenced,
            settlement_observed,
            recovery_receipt_bound,
            linkage_hidden,
        );
        let wallet_privacy_root = wallet_privacy_root(config, source, record_kind, ordinal);
        let nullifier_fence_root =
            nullifier_fence_root(config, source, record_kind, ordinal, &wallet_privacy_root);
        let settlement_root =
            settlement_root(config, source, record_kind, ordinal, &nullifier_fence_root);
        let recovery_receipt_root =
            recovery_receipt_root(config, source, record_kind, ordinal, &settlement_root);
        let unlinkability_witness_root = unlinkability_witness_root(
            config,
            source,
            record_kind,
            ordinal,
            &wallet_privacy_root,
            &nullifier_fence_root,
            &recovery_receipt_root,
        );
        let private_state_carry_root = private_state_carry_root(
            config,
            source,
            record_kind,
            ordinal,
            &unlinkability_witness_root,
        );
        let user_escape_continuity = privacy_root_bound
            && nullifier_fenced
            && recovery_receipt_bound
            && linkage_hidden
            && status != ContinuityStatus::BlockedByMissingFence;
        let production_blocked =
            config.hold_production_until_continuity_proven && status != ContinuityStatus::Proven;
        let continuity_record_root = continuity_record_root(
            config,
            source,
            record_kind,
            ordinal,
            &wallet_privacy_root,
            &nullifier_fence_root,
            &settlement_root,
            &recovery_receipt_root,
            &unlinkability_witness_root,
            &private_state_carry_root,
            status,
            user_escape_continuity,
            production_blocked,
        );
        let record_id = record_id(record_kind, ordinal, &continuity_record_root);
        Self {
            record_id,
            ordinal,
            record_kind,
            wallet_privacy_root,
            nullifier_fence_root,
            settlement_observation_root: settlement_root,
            recovery_receipt_root,
            unlinkability_witness_root,
            private_state_carry_root,
            status,
            privacy_root_bound,
            nullifier_fenced,
            settlement_observed,
            recovery_receipt_bound,
            linkage_hidden,
            user_escape_continuity,
            production_blocked,
            continuity_record_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "record_id": self.record_id,
            "ordinal": self.ordinal,
            "record_kind": self.record_kind.as_str(),
            "wallet_privacy_root": self.wallet_privacy_root,
            "nullifier_fence_root": self.nullifier_fence_root,
            "settlement_observation_root": self.settlement_observation_root,
            "recovery_receipt_root": self.recovery_receipt_root,
            "unlinkability_witness_root": self.unlinkability_witness_root,
            "private_state_carry_root": self.private_state_carry_root,
            "status": self.status.as_str(),
            "privacy_root_bound": self.privacy_root_bound,
            "nullifier_fenced": self.nullifier_fenced,
            "settlement_observed": self.settlement_observed,
            "recovery_receipt_bound": self.recovery_receipt_bound,
            "linkage_hidden": self.linkage_hidden,
            "user_escape_continuity": self.user_escape_continuity,
            "production_blocked": self.production_blocked,
            "continuity_record_root": self.continuity_record_root,
        })
    }

    pub fn state_root(&self) -> String {
        self.continuity_record_root.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ContinuityVerdict {
    pub continuity_record_count: u64,
    pub proven_count: u64,
    pub pending_settlement_count: u64,
    pub held_fail_closed_count: u64,
    pub missing_fence_count: u64,
    pub privacy_root_count: u64,
    pub nullifier_fence_count: u64,
    pub settlement_observed_count: u64,
    pub recovery_receipt_count: u64,
    pub linkage_hidden_count: u64,
    pub continuity_proven: bool,
    pub user_escape_continuity_proven: bool,
    pub production_blocked: bool,
    pub continuity_status: String,
    pub user_escape_answer: String,
    pub production_answer: String,
    pub verdict_root: String,
}

impl ContinuityVerdict {
    pub fn new(config: &Config, source: &SourceBundle, records: &[ContinuityRecord]) -> Self {
        let continuity_record_count = records.len() as u64;
        let proven_count = count_status(records, ContinuityStatus::Proven);
        let pending_settlement_count = count_status(records, ContinuityStatus::PendingSettlement);
        let held_fail_closed_count = count_status(records, ContinuityStatus::HeldFailClosed);
        let missing_fence_count = count_status(records, ContinuityStatus::BlockedByMissingFence);
        let privacy_root_count = records
            .iter()
            .filter(|record| record.privacy_root_bound)
            .count() as u64;
        let nullifier_fence_count = records
            .iter()
            .filter(|record| record.nullifier_fenced)
            .count() as u64;
        let settlement_observed_count = records
            .iter()
            .filter(|record| record.settlement_observed)
            .count() as u64;
        let recovery_receipt_count = records
            .iter()
            .filter(|record| record.recovery_receipt_bound)
            .count() as u64;
        let linkage_hidden_count = records
            .iter()
            .filter(|record| record.linkage_hidden)
            .count() as u64;
        let continuity_proven = continuity_record_count >= config.min_continuity_records
            && missing_fence_count == 0
            && held_fail_closed_count == 0
            && privacy_root_count == continuity_record_count
            && nullifier_fence_count == continuity_record_count
            && recovery_receipt_count == continuity_record_count
            && linkage_hidden_count == continuity_record_count
            && (!config.require_settlement_observations
                || settlement_observed_count == continuity_record_count);
        let user_escape_continuity_proven =
            continuity_proven || (missing_fence_count == 0 && held_fail_closed_count == 0);
        let production_blocked =
            config.hold_production_until_continuity_proven && !continuity_proven;
        let continuity_status = continuity_status_summary(
            continuity_proven,
            missing_fence_count,
            held_fail_closed_count,
            pending_settlement_count,
        )
        .to_string();
        let user_escape_answer = user_escape_answer(
            user_escape_continuity_proven,
            source.wallet_user_escape_ready,
        )
        .to_string();
        let production_answer = production_answer(production_blocked).to_string();
        let verdict_root = verdict_root(
            config,
            source,
            continuity_record_count,
            proven_count,
            pending_settlement_count,
            held_fail_closed_count,
            missing_fence_count,
            privacy_root_count,
            nullifier_fence_count,
            settlement_observed_count,
            recovery_receipt_count,
            linkage_hidden_count,
            continuity_proven,
            user_escape_continuity_proven,
            production_blocked,
            &continuity_status,
            &user_escape_answer,
            &production_answer,
        );
        Self {
            continuity_record_count,
            proven_count,
            pending_settlement_count,
            held_fail_closed_count,
            missing_fence_count,
            privacy_root_count,
            nullifier_fence_count,
            settlement_observed_count,
            recovery_receipt_count,
            linkage_hidden_count,
            continuity_proven,
            user_escape_continuity_proven,
            production_blocked,
            continuity_status,
            user_escape_answer,
            production_answer,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "continuity_record_count": self.continuity_record_count,
            "proven_count": self.proven_count,
            "pending_settlement_count": self.pending_settlement_count,
            "held_fail_closed_count": self.held_fail_closed_count,
            "missing_fence_count": self.missing_fence_count,
            "privacy_root_count": self.privacy_root_count,
            "nullifier_fence_count": self.nullifier_fence_count,
            "settlement_observed_count": self.settlement_observed_count,
            "recovery_receipt_count": self.recovery_receipt_count,
            "linkage_hidden_count": self.linkage_hidden_count,
            "continuity_proven": self.continuity_proven,
            "user_escape_continuity_proven": self.user_escape_continuity_proven,
            "production_blocked": self.production_blocked,
            "continuity_status": self.continuity_status,
            "user_escape_answer": self.user_escape_answer,
            "production_answer": self.production_answer,
            "verdict_root": self.verdict_root,
        })
    }

    pub fn state_root(&self) -> String {
        self.verdict_root.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub source: SourceBundle,
    pub continuity_records: Vec<ContinuityRecord>,
    pub verdict: ContinuityVerdict,
    pub continuity_record_root: String,
    pub wallet_privacy_bundle_root: String,
    pub nullifier_fence_bundle_root: String,
    pub settlement_continuity_bundle_root: String,
    pub recovery_continuity_bundle_root: String,
    pub unlinkability_bundle_root: String,
    pub state_commitment_root: String,
}

impl State {
    pub fn new(
        config: Config,
        wallet_scan_state: wallet_scan::State,
        settlement_state: settlement::State,
        recovery_state: recovery::State,
    ) -> Result<Self> {
        validate_config(&config)?;
        let source =
            SourceBundle::from_states(&wallet_scan_state, &settlement_state, &recovery_state);
        validate_source(&source)?;
        let continuity_records = ContinuityRecordKind::ordered()
            .iter()
            .enumerate()
            .map(|(index, record_kind)| {
                ContinuityRecord::from_kind(&config, &source, *record_kind, index as u64 + 1)
            })
            .collect::<Vec<_>>();
        let verdict = ContinuityVerdict::new(&config, &source, &continuity_records);
        let continuity_record_root = continuity_record_vector_root(&continuity_records);
        let wallet_privacy_bundle_root =
            wallet_privacy_bundle_root(&config, &source, &continuity_records, &verdict);
        let nullifier_fence_bundle_root =
            nullifier_fence_bundle_root(&config, &source, &continuity_records, &verdict);
        let settlement_continuity_bundle_root =
            settlement_continuity_bundle_root(&config, &source, &continuity_records, &verdict);
        let recovery_continuity_bundle_root =
            recovery_continuity_bundle_root(&config, &source, &continuity_records, &verdict);
        let unlinkability_bundle_root =
            unlinkability_bundle_root(&config, &source, &continuity_records, &verdict);
        let state_commitment_root = state_commitment_root(
            &config,
            &source,
            &continuity_record_root,
            &wallet_privacy_bundle_root,
            &nullifier_fence_bundle_root,
            &settlement_continuity_bundle_root,
            &recovery_continuity_bundle_root,
            &unlinkability_bundle_root,
            &verdict,
        );
        Ok(Self {
            config,
            source,
            continuity_records,
            verdict,
            continuity_record_root,
            wallet_privacy_bundle_root,
            nullifier_fence_bundle_root,
            settlement_continuity_bundle_root,
            recovery_continuity_bundle_root,
            unlinkability_bundle_root,
            state_commitment_root,
        })
    }

    pub fn devnet() -> Self {
        match Self::new(
            Config::default(),
            wallet_scan::devnet(),
            settlement::devnet(),
            recovery::devnet(),
        ) {
            Ok(state) => state,
            Err(reason) => fallback_state(reason),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_private_state_continuity_runtime",
            "chain_id": CHAIN_ID,
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "source": self.source.public_record(),
            "continuity_record_root": self.continuity_record_root,
            "wallet_privacy_bundle_root": self.wallet_privacy_bundle_root,
            "nullifier_fence_bundle_root": self.nullifier_fence_bundle_root,
            "settlement_continuity_bundle_root": self.settlement_continuity_bundle_root,
            "recovery_continuity_bundle_root": self.recovery_continuity_bundle_root,
            "unlinkability_bundle_root": self.unlinkability_bundle_root,
            "state_commitment_root": self.state_commitment_root,
            "verdict": self.verdict.public_record(),
            "continuity_records": self
                .continuity_records
                .iter()
                .map(ContinuityRecord::public_record)
                .collect::<Vec<_>>(),
        })
    }

    pub fn state_root(&self) -> String {
        self.state_commitment_root.clone()
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

pub fn continuity_record_vector_root(records: &[ContinuityRecord]) -> String {
    merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-PRIVATE-STATE-CONTINUITY-RECORDS",
        records
            .iter()
            .map(|record| record.continuity_record_root.clone())
            .collect::<Vec<_>>(),
    )
}

fn wallet_privacy_bundle_root(
    config: &Config,
    source: &SourceBundle,
    records: &[ContinuityRecord],
    verdict: &ContinuityVerdict,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-PRIVATE-STATE-CONTINUITY-WALLET-PRIVACY-BUNDLE",
        &[
            HashPart::Str(&config.continuity_suite),
            HashPart::Str(&source.wallet_scan_state_root),
            HashPart::Str(&source.encrypted_scan_bundle_root),
            HashPart::Str(&source.metadata_redaction_bundle_root),
            HashPart::Str(&continuity_record_vector_root(records)),
            HashPart::Str(&verdict.verdict_root),
        ],
        32,
    )
}

fn nullifier_fence_bundle_root(
    config: &Config,
    source: &SourceBundle,
    records: &[ContinuityRecord],
    verdict: &ContinuityVerdict,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-PRIVATE-STATE-CONTINUITY-NULLIFIER-FENCE-BUNDLE",
        &[
            HashPart::Str(&config.continuity_suite),
            HashPart::Str(&source.nullifier_fence_bundle_root),
            HashPart::U64(config.nullifier_fence_depth),
            HashPart::U64(verdict.nullifier_fence_count),
            HashPart::Str(&continuity_record_vector_root(records)),
        ],
        32,
    )
}

fn settlement_continuity_bundle_root(
    config: &Config,
    source: &SourceBundle,
    records: &[ContinuityRecord],
    verdict: &ContinuityVerdict,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-PRIVATE-STATE-CONTINUITY-SETTLEMENT-BUNDLE",
        &[
            HashPart::Str(&config.continuity_suite),
            HashPart::Str(&source.settlement_state_root),
            HashPart::Str(&source.settlement_observation_root),
            HashPart::Str(&source.settlement_evidence_bundle_root),
            HashPart::U64(verdict.settlement_observed_count),
            HashPart::Str(&continuity_record_vector_root(records)),
        ],
        32,
    )
}

fn recovery_continuity_bundle_root(
    config: &Config,
    source: &SourceBundle,
    records: &[ContinuityRecord],
    verdict: &ContinuityVerdict,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-PRIVATE-STATE-CONTINUITY-RECOVERY-BUNDLE",
        &[
            HashPart::Str(&config.continuity_suite),
            HashPart::Str(&source.recovery_state_root),
            HashPart::Str(&source.recovery_step_root),
            HashPart::Str(&source.recovery_evidence_bundle_root),
            HashPart::Str(&source.recovery_fail_closed_hold_root),
            HashPart::U64(verdict.recovery_receipt_count),
            HashPart::Str(&continuity_record_vector_root(records)),
        ],
        32,
    )
}

fn unlinkability_bundle_root(
    config: &Config,
    source: &SourceBundle,
    records: &[ContinuityRecord],
    verdict: &ContinuityVerdict,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-PRIVATE-STATE-CONTINUITY-UNLINKABILITY-BUNDLE",
        &[
            HashPart::Str(&config.continuity_suite),
            HashPart::Str(&source.linkage_privacy_bundle_root),
            HashPart::U64(verdict.linkage_hidden_count),
            HashPart::Str(&continuity_record_vector_root(records)),
            HashPart::Str(&verdict.verdict_root),
        ],
        32,
    )
}

fn state_commitment_root(
    config: &Config,
    source: &SourceBundle,
    continuity_record_root: &str,
    wallet_privacy_bundle_root: &str,
    nullifier_fence_bundle_root: &str,
    settlement_continuity_bundle_root: &str,
    recovery_continuity_bundle_root: &str,
    unlinkability_bundle_root: &str,
    verdict: &ContinuityVerdict,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-PRIVATE-STATE-CONTINUITY-STATE",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&source.state_root()),
            HashPart::Str(continuity_record_root),
            HashPart::Str(wallet_privacy_bundle_root),
            HashPart::Str(nullifier_fence_bundle_root),
            HashPart::Str(settlement_continuity_bundle_root),
            HashPart::Str(recovery_continuity_bundle_root),
            HashPart::Str(unlinkability_bundle_root),
            HashPart::Str(&verdict.verdict_root),
        ],
        32,
    )
}

fn wallet_privacy_root(
    config: &Config,
    source: &SourceBundle,
    record_kind: ContinuityRecordKind,
    ordinal: u64,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-PRIVATE-STATE-CONTINUITY-WALLET-PRIVACY-ROOT",
        &[
            HashPart::Str(&config.continuity_suite),
            HashPart::Str(record_kind.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(&source.encrypted_scan_bundle_root),
            HashPart::Str(&source.metadata_redaction_bundle_root),
        ],
        32,
    )
}

fn nullifier_fence_root(
    config: &Config,
    source: &SourceBundle,
    record_kind: ContinuityRecordKind,
    ordinal: u64,
    wallet_privacy_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-PRIVATE-STATE-CONTINUITY-NULLIFIER-FENCE",
        &[
            HashPart::Str(&config.continuity_suite),
            HashPart::Str(record_kind.as_str()),
            HashPart::U64(ordinal),
            HashPart::U64(config.nullifier_fence_depth),
            HashPart::Str(wallet_privacy_root),
            HashPart::Str(&source.nullifier_fence_bundle_root),
        ],
        32,
    )
}

fn settlement_root(
    config: &Config,
    source: &SourceBundle,
    record_kind: ContinuityRecordKind,
    ordinal: u64,
    nullifier_fence_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-PRIVATE-STATE-CONTINUITY-SETTLEMENT-ROOT",
        &[
            HashPart::Str(&config.continuity_suite),
            HashPart::Str(record_kind.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(nullifier_fence_root),
            HashPart::Str(&source.settlement_observation_root),
        ],
        32,
    )
}

fn recovery_receipt_root(
    config: &Config,
    source: &SourceBundle,
    record_kind: ContinuityRecordKind,
    ordinal: u64,
    settlement_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-PRIVATE-STATE-CONTINUITY-RECOVERY-RECEIPT",
        &[
            HashPart::Str(&config.continuity_suite),
            HashPart::Str(record_kind.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(settlement_root),
            HashPart::Str(&source.recovery_evidence_bundle_root),
        ],
        32,
    )
}

fn unlinkability_witness_root(
    config: &Config,
    source: &SourceBundle,
    record_kind: ContinuityRecordKind,
    ordinal: u64,
    wallet_privacy_root: &str,
    nullifier_fence_root: &str,
    recovery_receipt_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-PRIVATE-STATE-CONTINUITY-UNLINKABILITY-WITNESS",
        &[
            HashPart::Str(&config.continuity_suite),
            HashPart::Str(record_kind.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(wallet_privacy_root),
            HashPart::Str(nullifier_fence_root),
            HashPart::Str(recovery_receipt_root),
            HashPart::Str(&source.linkage_privacy_bundle_root),
        ],
        32,
    )
}

fn private_state_carry_root(
    config: &Config,
    source: &SourceBundle,
    record_kind: ContinuityRecordKind,
    ordinal: u64,
    unlinkability_witness_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-PRIVATE-STATE-CONTINUITY-CARRY-FORWARD",
        &[
            HashPart::Str(&config.continuity_suite),
            HashPart::Str(record_kind.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(unlinkability_witness_root),
            HashPart::Str(&source.wallet_scan_receipt_root),
            HashPart::Str(&source.recovery_step_root),
        ],
        32,
    )
}

fn continuity_record_root(
    config: &Config,
    source: &SourceBundle,
    record_kind: ContinuityRecordKind,
    ordinal: u64,
    wallet_privacy_root: &str,
    nullifier_fence_root: &str,
    settlement_root: &str,
    recovery_receipt_root: &str,
    unlinkability_witness_root: &str,
    private_state_carry_root: &str,
    status: ContinuityStatus,
    user_escape_continuity: bool,
    production_blocked: bool,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-PRIVATE-STATE-CONTINUITY-RECORD",
        &[
            HashPart::Str(&config.continuity_suite),
            HashPart::Str(&source.wallet_scan_state_root),
            HashPart::Str(record_kind.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(wallet_privacy_root),
            HashPart::Str(nullifier_fence_root),
            HashPart::Str(settlement_root),
            HashPart::Str(recovery_receipt_root),
            HashPart::Str(unlinkability_witness_root),
            HashPart::Str(private_state_carry_root),
            HashPart::Str(status.as_str()),
            HashPart::Str(bool_str(user_escape_continuity)),
            HashPart::Str(bool_str(production_blocked)),
        ],
        32,
    )
}

fn verdict_root(
    config: &Config,
    source: &SourceBundle,
    continuity_record_count: u64,
    proven_count: u64,
    pending_settlement_count: u64,
    held_fail_closed_count: u64,
    missing_fence_count: u64,
    privacy_root_count: u64,
    nullifier_fence_count: u64,
    settlement_observed_count: u64,
    recovery_receipt_count: u64,
    linkage_hidden_count: u64,
    continuity_proven: bool,
    user_escape_continuity_proven: bool,
    production_blocked: bool,
    continuity_status: &str,
    user_escape_answer: &str,
    production_answer: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-PRIVATE-STATE-CONTINUITY-VERDICT",
        &[
            HashPart::Str(&config.continuity_suite),
            HashPart::Str(&source.wallet_scan_state_root),
            HashPart::Str(&source.settlement_state_root),
            HashPart::Str(&source.recovery_state_root),
            HashPart::U64(continuity_record_count),
            HashPart::U64(proven_count),
            HashPart::U64(pending_settlement_count),
            HashPart::U64(held_fail_closed_count),
            HashPart::U64(missing_fence_count),
            HashPart::U64(privacy_root_count),
            HashPart::U64(nullifier_fence_count),
            HashPart::U64(settlement_observed_count),
            HashPart::U64(recovery_receipt_count),
            HashPart::U64(linkage_hidden_count),
            HashPart::Str(bool_str(continuity_proven)),
            HashPart::Str(bool_str(user_escape_continuity_proven)),
            HashPart::Str(bool_str(production_blocked)),
            HashPart::Str(continuity_status),
            HashPart::Str(user_escape_answer),
            HashPart::Str(production_answer),
        ],
        32,
    )
}

fn record_id(record_kind: ContinuityRecordKind, ordinal: u64, record_root: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-PRIVATE-STATE-CONTINUITY-ID",
        &[
            HashPart::Str(record_kind.as_str()),
            HashPart::U64(ordinal),
            HashPart::Str(record_root),
        ],
        16,
    )
}

fn privacy_root_bound(
    config: &Config,
    source: &SourceBundle,
    record_kind: ContinuityRecordKind,
) -> bool {
    !config.require_wallet_scan_privacy_roots
        || (!source.encrypted_scan_bundle_root.is_empty()
            && !source.metadata_redaction_bundle_root.is_empty()
            && record_kind != ContinuityRecordKind::SettlementObservation)
        || source.wallet_user_escape_ready
}

fn nullifier_fenced(
    config: &Config,
    source: &SourceBundle,
    record_kind: ContinuityRecordKind,
) -> bool {
    !config.require_nullifier_fences
        || (!source.nullifier_fence_bundle_root.is_empty()
            && source.wallet_scan_receipt_count >= config.nullifier_fence_depth)
        || record_kind == ContinuityRecordKind::RecoveryReceipt
}

fn settlement_observed(
    config: &Config,
    source: &SourceBundle,
    record_kind: ContinuityRecordKind,
) -> bool {
    !config.require_settlement_observations
        || source.settlement_observed
        || record_kind == ContinuityRecordKind::RecoveryReceipt
}

fn recovery_receipt_bound(
    config: &Config,
    source: &SourceBundle,
    record_kind: ContinuityRecordKind,
) -> bool {
    !config.require_recovery_receipts
        || (!source.recovery_evidence_bundle_root.is_empty() && source.recovery_step_count > 0)
        || record_kind == ContinuityRecordKind::SettlementObservation
}

fn linkage_hidden(
    config: &Config,
    source: &SourceBundle,
    record_kind: ContinuityRecordKind,
) -> bool {
    !config.require_linkage_blinding
        || (source.wallet_linkage_hidden && !source.linkage_privacy_bundle_root.is_empty())
        || record_kind == ContinuityRecordKind::LinkageBlind
}

fn continuity_status(
    nullifier_fenced: bool,
    settlement_observed: bool,
    recovery_receipt_bound: bool,
    linkage_hidden: bool,
) -> ContinuityStatus {
    if !nullifier_fenced || !linkage_hidden {
        ContinuityStatus::BlockedByMissingFence
    } else if !recovery_receipt_bound {
        ContinuityStatus::HeldFailClosed
    } else if !settlement_observed {
        ContinuityStatus::PendingSettlement
    } else {
        ContinuityStatus::Proven
    }
}

fn continuity_status_summary(
    continuity_proven: bool,
    missing_fence_count: u64,
    held_fail_closed_count: u64,
    pending_settlement_count: u64,
) -> &'static str {
    if continuity_proven {
        "private_state_continuity_proven"
    } else if missing_fence_count > 0 {
        "blocked_by_missing_nullifier_or_linkage_fence"
    } else if held_fail_closed_count > 0 {
        "held_fail_closed_until_recovery_receipts_bind"
    } else if pending_settlement_count > 0 {
        "pending_settlement_observation"
    } else {
        "continuity_records_collected"
    }
}

fn user_escape_answer(
    user_escape_continuity_proven: bool,
    wallet_user_escape_ready: bool,
) -> &'static str {
    if user_escape_continuity_proven && wallet_user_escape_ready {
        "user escape continuity is proven from private roots without revealing wallet linkage"
    } else if user_escape_continuity_proven {
        "private state continuity is preserved under fail-closed wallet readiness"
    } else {
        "user escape continuity remains held until privacy roots, fences, and receipts bind"
    }
}

fn production_answer(production_blocked: bool) -> &'static str {
    if production_blocked {
        "production remains held until private state continuity is proven"
    } else {
        "production may proceed with unlinkable private state continuity proof"
    }
}

fn count_status(records: &[ContinuityRecord], status: ContinuityStatus) -> u64 {
    records
        .iter()
        .filter(|record| record.status == status)
        .count() as u64
}

fn validate_config(config: &Config) -> Result<()> {
    ensure(
        config.chain_id == CHAIN_ID,
        "force-exit private state continuity chain mismatch",
    )?;
    ensure(
        config.protocol_version == PROTOCOL_VERSION,
        "force-exit private state continuity protocol mismatch",
    )?;
    ensure(
        config.min_continuity_records > 0,
        "force-exit private state continuity requires records",
    )?;
    ensure(
        config.nullifier_fence_depth > 0,
        "force-exit private state continuity requires nullifier fences",
    )?;
    Ok(())
}

fn validate_source(source: &SourceBundle) -> Result<()> {
    ensure(
        !source.wallet_scan_state_root.is_empty(),
        "force-exit private state continuity missing wallet scan state root",
    )?;
    ensure(
        !source.settlement_state_root.is_empty(),
        "force-exit private state continuity missing settlement state root",
    )?;
    ensure(
        !source.recovery_state_root.is_empty(),
        "force-exit private state continuity missing recovery state root",
    )?;
    Ok(())
}

fn ensure(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message.to_string())
    }
}

fn fallback_state(reason: String) -> State {
    let config = Config::default();
    let source = SourceBundle {
        wallet_scan_state_root: record_root(
            "fallback-wallet-scan-state",
            &json!({"reason": &reason}),
        ),
        wallet_scan_receipt_root: record_root(
            "fallback-wallet-scan-receipt",
            &json!({"reason": &reason}),
        ),
        encrypted_scan_bundle_root: record_root(
            "fallback-encrypted-scan",
            &json!({"reason": &reason}),
        ),
        nullifier_fence_bundle_root: record_root(
            "fallback-nullifier-fence",
            &json!({"reason": &reason}),
        ),
        metadata_redaction_bundle_root: record_root(
            "fallback-metadata-redaction",
            &json!({"reason": &reason}),
        ),
        linkage_privacy_bundle_root: record_root(
            "fallback-linkage-privacy",
            &json!({"reason": &reason}),
        ),
        wallet_user_escape_ready: false,
        wallet_linkage_hidden: true,
        settlement_state_root: record_root(
            "fallback-settlement-state",
            &json!({"reason": &reason}),
        ),
        settlement_observation_root: record_root(
            "fallback-settlement-observation",
            &json!({"reason": &reason}),
        ),
        settlement_evidence_bundle_root: record_root(
            "fallback-settlement-evidence",
            &json!({"reason": &reason}),
        ),
        settlement_observed: false,
        recovery_state_root: record_root("fallback-recovery-state", &json!({"reason": &reason})),
        recovery_step_root: record_root("fallback-recovery-step", &json!({"reason": &reason})),
        recovery_evidence_bundle_root: record_root(
            "fallback-recovery-evidence",
            &json!({"reason": &reason}),
        ),
        recovery_fail_closed_hold_root: record_root(
            "fallback-recovery-hold",
            &json!({"reason": &reason}),
        ),
        wallet_scan_receipt_count: 1,
        settlement_observation_count: 1,
        recovery_step_count: 1,
    };
    let continuity_records = ContinuityRecordKind::ordered()
        .iter()
        .enumerate()
        .map(|(index, record_kind)| {
            ContinuityRecord::from_kind(&config, &source, *record_kind, index as u64 + 1)
        })
        .collect::<Vec<_>>();
    let verdict = ContinuityVerdict::new(&config, &source, &continuity_records);
    let continuity_record_root = continuity_record_vector_root(&continuity_records);
    let wallet_privacy_bundle_root =
        wallet_privacy_bundle_root(&config, &source, &continuity_records, &verdict);
    let nullifier_fence_bundle_root =
        nullifier_fence_bundle_root(&config, &source, &continuity_records, &verdict);
    let settlement_continuity_bundle_root =
        settlement_continuity_bundle_root(&config, &source, &continuity_records, &verdict);
    let recovery_continuity_bundle_root =
        recovery_continuity_bundle_root(&config, &source, &continuity_records, &verdict);
    let unlinkability_bundle_root =
        unlinkability_bundle_root(&config, &source, &continuity_records, &verdict);
    let state_commitment_root = state_commitment_root(
        &config,
        &source,
        &continuity_record_root,
        &wallet_privacy_bundle_root,
        &nullifier_fence_bundle_root,
        &settlement_continuity_bundle_root,
        &recovery_continuity_bundle_root,
        &unlinkability_bundle_root,
        &verdict,
    );
    State {
        config,
        source,
        continuity_records,
        verdict,
        continuity_record_root,
        wallet_privacy_bundle_root,
        nullifier_fence_bundle_root,
        settlement_continuity_bundle_root,
        recovery_continuity_bundle_root,
        unlinkability_bundle_root,
        state_commitment_root,
    }
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-PRIVATE-STATE-CONTINUITY-JSON-RECORD",
        &[HashPart::Str(kind), HashPart::Json(record)],
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
