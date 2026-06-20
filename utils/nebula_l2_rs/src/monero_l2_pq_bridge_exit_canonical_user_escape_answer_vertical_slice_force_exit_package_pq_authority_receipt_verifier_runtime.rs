use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_execution_receipt_runtime as execution,
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackagePqAuthorityReceiptVerifierRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_PQ_AUTHORITY_RECEIPT_VERIFIER_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-pq-authority-receipt-verifier-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_PQ_AUTHORITY_RECEIPT_VERIFIER_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const PQ_AUTHORITY_RECEIPT_VERIFIER_SUITE: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-force-exit-package-pq-authority-receipt-verifier-v1";
pub const DEFAULT_REQUIRED_QUORUM_WEIGHT: u64 = 67;
pub const DEFAULT_MIN_SIGNATURE_COUNT: u64 = 3;
pub const DEFAULT_CURRENT_AUTHORITY_EPOCH: u64 = 88;
pub const DEFAULT_MAX_EPOCH_LAG: u64 = 1;
pub const DEFAULT_MIN_SECURITY_BITS: u64 = 192;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub verifier_suite: String,
    pub required_quorum_weight: u64,
    pub min_signature_count: u64,
    pub current_authority_epoch: u64,
    pub max_epoch_lag: u64,
    pub min_security_bits: u64,
    pub require_authority_upgrade_separation: bool,
    pub require_execution_receipt_binding: bool,
    pub require_fail_closed_release_hold: bool,
    pub hold_production_until_quorum_verified: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            verifier_suite: PQ_AUTHORITY_RECEIPT_VERIFIER_SUITE.to_string(),
            required_quorum_weight: DEFAULT_REQUIRED_QUORUM_WEIGHT,
            min_signature_count: DEFAULT_MIN_SIGNATURE_COUNT,
            current_authority_epoch: DEFAULT_CURRENT_AUTHORITY_EPOCH,
            max_epoch_lag: DEFAULT_MAX_EPOCH_LAG,
            min_security_bits: DEFAULT_MIN_SECURITY_BITS,
            require_authority_upgrade_separation: true,
            require_execution_receipt_binding: true,
            require_fail_closed_release_hold: true,
            hold_production_until_quorum_verified: true,
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
            "verifier_suite": self.verifier_suite,
            "required_quorum_weight": self.required_quorum_weight,
            "min_signature_count": self.min_signature_count,
            "current_authority_epoch": self.current_authority_epoch,
            "max_epoch_lag": self.max_epoch_lag,
            "min_security_bits": self.min_security_bits,
            "require_authority_upgrade_separation": self.require_authority_upgrade_separation,
            "require_execution_receipt_binding": self.require_execution_receipt_binding,
            "require_fail_closed_release_hold": self.require_fail_closed_release_hold,
            "hold_production_until_quorum_verified": self.hold_production_until_quorum_verified,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceBundle {
    pub execution_state_root: String,
    pub execution_receipt_root: String,
    pub pq_privacy_receipt_root: String,
    pub recovery_receipt_root: String,
    pub production_hold_root: String,
    pub execution_status: String,
    pub execution_user_escape_answer: String,
    pub execution_production_answer: String,
    pub execution_receipt_count: u64,
    pub observed_receipt_count: u64,
    pub deferred_receipt_count: u64,
    pub release_held_count: u64,
    pub fail_closed_count: u64,
    pub production_blocker_count: u64,
    pub package_execution_observed: bool,
    pub user_escape_execution_observed: bool,
    pub execution_production_blocked: bool,
}

impl SourceBundle {
    pub fn from_execution(state: &execution::State) -> Self {
        Self {
            execution_state_root: state.state_root(),
            execution_receipt_root: state.execution_receipt_root.clone(),
            pq_privacy_receipt_root: state.pq_privacy_receipt_root.clone(),
            recovery_receipt_root: state.recovery_receipt_root.clone(),
            production_hold_root: state.production_hold_root.clone(),
            execution_status: state.verdict.execution_status.clone(),
            execution_user_escape_answer: state.verdict.user_escape_answer.clone(),
            execution_production_answer: state.verdict.production_answer.clone(),
            execution_receipt_count: state.verdict.execution_receipt_count,
            observed_receipt_count: state.verdict.observed_receipt_count,
            deferred_receipt_count: state.verdict.deferred_receipt_count,
            release_held_count: state.verdict.release_held_count,
            fail_closed_count: state.verdict.fail_closed_count,
            production_blocker_count: state.verdict.production_blocker_count,
            package_execution_observed: state.verdict.package_execution_observed,
            user_escape_execution_observed: state.verdict.user_escape_execution_observed,
            execution_production_blocked: state.verdict.production_blocked,
        }
    }

    pub fn devnet() -> Self {
        let state = execution::devnet();
        Self::from_execution(&state)
    }

    pub fn public_record(&self) -> Value {
        json!({
            "execution_state_root": self.execution_state_root,
            "execution_receipt_root": self.execution_receipt_root,
            "pq_privacy_receipt_root": self.pq_privacy_receipt_root,
            "recovery_receipt_root": self.recovery_receipt_root,
            "production_hold_root": self.production_hold_root,
            "execution_status": self.execution_status,
            "execution_user_escape_answer": self.execution_user_escape_answer,
            "execution_production_answer": self.execution_production_answer,
            "execution_receipt_count": self.execution_receipt_count,
            "observed_receipt_count": self.observed_receipt_count,
            "deferred_receipt_count": self.deferred_receipt_count,
            "release_held_count": self.release_held_count,
            "fail_closed_count": self.fail_closed_count,
            "production_blocker_count": self.production_blocker_count,
            "package_execution_observed": self.package_execution_observed,
            "user_escape_execution_observed": self.user_escape_execution_observed,
            "execution_production_blocked": self.execution_production_blocked,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("source-bundle", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PqAuthorityReceiptStatus {
    Verified,
    StaleEpoch,
    UpgradeAuthorityConflict,
    InvalidSignature,
    ReleaseHeld,
    FailClosed,
}

impl PqAuthorityReceiptStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::StaleEpoch => "stale_epoch",
            Self::UpgradeAuthorityConflict => "upgrade_authority_conflict",
            Self::InvalidSignature => "invalid_signature",
            Self::ReleaseHeld => "release_held",
            Self::FailClosed => "fail_closed",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PqAuthorityReceipt {
    pub receipt_id: String,
    pub ordinal: u64,
    pub authority_id: String,
    pub authority_epoch: u64,
    pub authority_set_root: String,
    pub upgrade_authority_root: String,
    pub signer_key_root: String,
    pub signature_scheme: String,
    pub signature_root: String,
    pub signed_execution_receipt_root: String,
    pub signed_pq_privacy_receipt_root: String,
    pub quorum_weight: u64,
    pub security_bits: u64,
    pub signature_verified: bool,
    pub execution_binding_verified: bool,
    pub upgrade_authority_separated: bool,
    pub epoch_fresh: bool,
    pub release_hold_root: String,
    pub fail_closed_root: String,
    pub status: PqAuthorityReceiptStatus,
    pub receipt_root: String,
    pub required_outcome: String,
}

impl PqAuthorityReceipt {
    pub fn devnet(
        config: &Config,
        source: &SourceBundle,
        ordinal: u64,
        authority_id: &str,
    ) -> Self {
        let authority_epoch = config.current_authority_epoch;
        let authority_set_root = authority_set_root(config, source, authority_epoch);
        let upgrade_authority_root = upgrade_authority_root(config, source);
        let signer_key_root = signer_key_root(config, authority_id, authority_epoch);
        let signature_scheme = "ml-dsa-87+slh-dsa-shake-256f".to_string();
        let signed_execution_receipt_root = source.execution_receipt_root.clone();
        let signed_pq_privacy_receipt_root = source.pq_privacy_receipt_root.clone();
        let signature_root = signature_root(
            config,
            authority_id,
            authority_epoch,
            &signer_key_root,
            &signed_execution_receipt_root,
            &signed_pq_privacy_receipt_root,
        );
        let quorum_weight = devnet_weight(ordinal);
        let security_bits = 256;
        let signature_verified = true;
        let execution_binding_verified = signed_execution_receipt_root
            == source.execution_receipt_root
            && signed_pq_privacy_receipt_root == source.pq_privacy_receipt_root;
        let upgrade_authority_separated = authority_set_root != upgrade_authority_root;
        let epoch_fresh = epoch_is_fresh(config, authority_epoch);
        let status = receipt_status(
            config,
            source,
            signature_verified,
            execution_binding_verified,
            upgrade_authority_separated,
            epoch_fresh,
            security_bits,
        );
        let release_hold_root = release_hold_root(
            config,
            source,
            authority_id,
            authority_epoch,
            status,
            upgrade_authority_separated,
            epoch_fresh,
        );
        let fail_closed_root = fail_closed_root(
            config,
            source,
            authority_id,
            authority_epoch,
            status,
            signature_verified,
            execution_binding_verified,
        );
        let receipt_root = authority_receipt_root(
            config,
            source,
            authority_id,
            authority_epoch,
            &authority_set_root,
            &upgrade_authority_root,
            &signer_key_root,
            &signature_root,
            &signed_execution_receipt_root,
            &signed_pq_privacy_receipt_root,
            quorum_weight,
            security_bits,
            status,
            &release_hold_root,
            &fail_closed_root,
        );
        let receipt_id = receipt_id(authority_id, ordinal, &receipt_root);
        Self {
            receipt_id,
            ordinal,
            authority_id: authority_id.to_string(),
            authority_epoch,
            authority_set_root,
            upgrade_authority_root,
            signer_key_root,
            signature_scheme,
            signature_root,
            signed_execution_receipt_root,
            signed_pq_privacy_receipt_root,
            quorum_weight,
            security_bits,
            signature_verified,
            execution_binding_verified,
            upgrade_authority_separated,
            epoch_fresh,
            release_hold_root,
            fail_closed_root,
            status,
            receipt_root,
            required_outcome: required_outcome(status).to_string(),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "receipt_id": self.receipt_id,
            "ordinal": self.ordinal,
            "authority_id": self.authority_id,
            "authority_epoch": self.authority_epoch,
            "authority_set_root": self.authority_set_root,
            "upgrade_authority_root": self.upgrade_authority_root,
            "signer_key_root": self.signer_key_root,
            "signature_scheme": self.signature_scheme,
            "signature_root": self.signature_root,
            "signed_execution_receipt_root": self.signed_execution_receipt_root,
            "signed_pq_privacy_receipt_root": self.signed_pq_privacy_receipt_root,
            "quorum_weight": self.quorum_weight,
            "security_bits": self.security_bits,
            "signature_verified": self.signature_verified,
            "execution_binding_verified": self.execution_binding_verified,
            "upgrade_authority_separated": self.upgrade_authority_separated,
            "epoch_fresh": self.epoch_fresh,
            "release_hold_root": self.release_hold_root,
            "fail_closed_root": self.fail_closed_root,
            "status": self.status.as_str(),
            "receipt_root": self.receipt_root,
            "required_outcome": self.required_outcome,
        })
    }

    pub fn state_root(&self) -> String {
        self.receipt_root.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PqAuthorityReceiptVerdict {
    pub receipt_count: u64,
    pub verified_receipt_count: u64,
    pub stale_epoch_count: u64,
    pub upgrade_conflict_count: u64,
    pub invalid_signature_count: u64,
    pub release_held_count: u64,
    pub fail_closed_count: u64,
    pub observed_quorum_weight: u64,
    pub min_security_bits_observed: u64,
    pub required_quorum_weight: u64,
    pub quorum_verified: bool,
    pub signature_threshold_met: bool,
    pub all_epochs_fresh: bool,
    pub authority_upgrade_separated: bool,
    pub execution_binding_verified: bool,
    pub release_held: bool,
    pub fail_closed: bool,
    pub production_blocked: bool,
    pub verifier_status: String,
    pub user_escape_answer: String,
    pub production_answer: String,
    pub verdict_root: String,
}

impl PqAuthorityReceiptVerdict {
    pub fn new(config: &Config, source: &SourceBundle, receipts: &[PqAuthorityReceipt]) -> Self {
        let receipt_count = receipts.len() as u64;
        let verified_receipt_count = count_status(receipts, PqAuthorityReceiptStatus::Verified);
        let stale_epoch_count = count_status(receipts, PqAuthorityReceiptStatus::StaleEpoch);
        let upgrade_conflict_count =
            count_status(receipts, PqAuthorityReceiptStatus::UpgradeAuthorityConflict);
        let invalid_signature_count =
            count_status(receipts, PqAuthorityReceiptStatus::InvalidSignature);
        let release_held_count = count_status(receipts, PqAuthorityReceiptStatus::ReleaseHeld);
        let fail_closed_count = count_status(receipts, PqAuthorityReceiptStatus::FailClosed);
        let observed_quorum_weight = receipts
            .iter()
            .filter(|receipt| receipt.status == PqAuthorityReceiptStatus::Verified)
            .map(|receipt| receipt.quorum_weight)
            .sum();
        let min_security_bits_observed = receipts
            .iter()
            .filter(|receipt| receipt.status == PqAuthorityReceiptStatus::Verified)
            .map(|receipt| receipt.security_bits)
            .min()
            .map_or(0, |bits| bits);
        let quorum_verified = observed_quorum_weight >= config.required_quorum_weight;
        let signature_threshold_met = verified_receipt_count >= config.min_signature_count;
        let all_epochs_fresh =
            stale_epoch_count == 0 && receipts.iter().all(|receipt| receipt.epoch_fresh);
        let authority_upgrade_separated = upgrade_conflict_count == 0
            && receipts
                .iter()
                .all(|receipt| receipt.upgrade_authority_separated);
        let execution_binding_verified = receipts
            .iter()
            .all(|receipt| receipt.execution_binding_verified);
        let release_held = source.release_held_count > 0 || release_held_count > 0;
        let fail_closed = source.fail_closed_count > 0
            || fail_closed_count > 0
            || invalid_signature_count > 0
            || stale_epoch_count > 0
            || upgrade_conflict_count > 0
            || !execution_binding_verified
            || !authority_upgrade_separated
            || !all_epochs_fresh;
        let production_blocked = fail_closed
            || release_held
            || source.execution_production_blocked
            || (config.hold_production_until_quorum_verified
                && (!quorum_verified || !signature_threshold_met));
        let verifier_status = if fail_closed {
            "fail_closed"
        } else if release_held {
            "release_held"
        } else if quorum_verified && signature_threshold_met {
            "pq_authority_quorum_verified"
        } else {
            "insufficient_pq_authority_quorum"
        }
        .to_string();
        let user_escape_answer =
            if quorum_verified && signature_threshold_met && !fail_closed && !release_held {
                "force-exit execution has fresh PQ authority receipt quorum with upgrade authority separated"
            } else if fail_closed {
                "PQ authority receipt verification failed closed and wallet release remains blocked"
            } else {
                "force-exit execution awaits fresh PQ authority quorum receipts before user release"
            }
            .to_string();
        let production_answer = if production_blocked {
            "production release remains held until fresh PQ authority quorum receipts bind the force-exit execution"
        } else {
            "PQ authority receipt quorum verifies force-exit execution for production release review"
        }
        .to_string();
        let verdict_root = verdict_root(
            config,
            source,
            receipt_count,
            verified_receipt_count,
            stale_epoch_count,
            upgrade_conflict_count,
            invalid_signature_count,
            observed_quorum_weight,
            quorum_verified,
            signature_threshold_met,
            all_epochs_fresh,
            authority_upgrade_separated,
            execution_binding_verified,
            fail_closed,
            production_blocked,
            &verifier_status,
            &user_escape_answer,
            &production_answer,
        );
        Self {
            receipt_count,
            verified_receipt_count,
            stale_epoch_count,
            upgrade_conflict_count,
            invalid_signature_count,
            release_held_count,
            fail_closed_count,
            observed_quorum_weight,
            min_security_bits_observed,
            required_quorum_weight: config.required_quorum_weight,
            quorum_verified,
            signature_threshold_met,
            all_epochs_fresh,
            authority_upgrade_separated,
            execution_binding_verified,
            release_held,
            fail_closed,
            production_blocked,
            verifier_status,
            user_escape_answer,
            production_answer,
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "receipt_count": self.receipt_count,
            "verified_receipt_count": self.verified_receipt_count,
            "stale_epoch_count": self.stale_epoch_count,
            "upgrade_conflict_count": self.upgrade_conflict_count,
            "invalid_signature_count": self.invalid_signature_count,
            "release_held_count": self.release_held_count,
            "fail_closed_count": self.fail_closed_count,
            "observed_quorum_weight": self.observed_quorum_weight,
            "min_security_bits_observed": self.min_security_bits_observed,
            "required_quorum_weight": self.required_quorum_weight,
            "quorum_verified": self.quorum_verified,
            "signature_threshold_met": self.signature_threshold_met,
            "all_epochs_fresh": self.all_epochs_fresh,
            "authority_upgrade_separated": self.authority_upgrade_separated,
            "execution_binding_verified": self.execution_binding_verified,
            "release_held": self.release_held,
            "fail_closed": self.fail_closed,
            "production_blocked": self.production_blocked,
            "verifier_status": self.verifier_status,
            "user_escape_answer": self.user_escape_answer,
            "production_answer": self.production_answer,
            "verdict_root": self.verdict_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub source: SourceBundle,
    pub authority_receipts: Vec<PqAuthorityReceipt>,
    pub verdict: PqAuthorityReceiptVerdict,
    pub authority_receipt_root: String,
    pub quorum_receipt_root: String,
    pub epoch_guard_root: String,
    pub upgrade_separation_root: String,
    pub release_hold_root: String,
    pub fail_closed_root: String,
    pub state_commitment_root: String,
}

impl State {
    pub fn new(config: Config, execution_state: execution::State) -> Result<Self> {
        validate_config(&config)?;
        let source = SourceBundle::from_execution(&execution_state);
        validate_source(&source)?;
        let authority_receipts = [
            "pq-authority:alpha",
            "pq-authority:bravo",
            "pq-authority:charlie",
        ]
        .iter()
        .enumerate()
        .map(|(index, authority_id)| {
            PqAuthorityReceipt::devnet(&config, &source, (index as u64) + 1, authority_id)
        })
        .collect::<Vec<_>>();
        let verdict = PqAuthorityReceiptVerdict::new(&config, &source, &authority_receipts);
        let authority_receipt_root = authority_receipt_vector_root(&authority_receipts);
        let quorum_receipt_root =
            quorum_receipt_root(&config, &source, &authority_receipts, &verdict);
        let epoch_guard_root = epoch_guard_root(&config, &source, &authority_receipts, &verdict);
        let upgrade_separation_root =
            upgrade_separation_root(&config, &source, &authority_receipts, &verdict);
        let release_hold_root =
            release_hold_bundle_root(&config, &source, &authority_receipts, &verdict);
        let fail_closed_root =
            fail_closed_bundle_root(&config, &source, &authority_receipts, &verdict);
        let state_commitment_root = state_commitment_root(
            &config,
            &source,
            &authority_receipt_root,
            &quorum_receipt_root,
            &epoch_guard_root,
            &upgrade_separation_root,
            &release_hold_root,
            &fail_closed_root,
            &verdict,
        );
        Ok(Self {
            config,
            source,
            authority_receipts,
            verdict,
            authority_receipt_root,
            quorum_receipt_root,
            epoch_guard_root,
            upgrade_separation_root,
            release_hold_root,
            fail_closed_root,
            state_commitment_root,
        })
    }

    pub fn devnet() -> Self {
        match Self::new(Config::default(), execution::devnet()) {
            Ok(state) => state,
            Err(reason) => fallback_state(reason),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "kind": "monero_l2_pq_bridge_exit_canonical_user_escape_answer_vertical_slice_force_exit_package_pq_authority_receipt_verifier_runtime",
            "chain_id": CHAIN_ID,
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "source": self.source.public_record(),
            "authority_receipt_root": self.authority_receipt_root,
            "quorum_receipt_root": self.quorum_receipt_root,
            "epoch_guard_root": self.epoch_guard_root,
            "upgrade_separation_root": self.upgrade_separation_root,
            "release_hold_root": self.release_hold_root,
            "fail_closed_root": self.fail_closed_root,
            "state_commitment_root": self.state_commitment_root,
            "verdict": self.verdict.public_record(),
            "authority_receipts": self
                .authority_receipts
                .iter()
                .map(PqAuthorityReceipt::public_record)
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

fn receipt_status(
    config: &Config,
    source: &SourceBundle,
    signature_verified: bool,
    execution_binding_verified: bool,
    upgrade_authority_separated: bool,
    epoch_fresh: bool,
    security_bits: u64,
) -> PqAuthorityReceiptStatus {
    if source.fail_closed_count > 0 {
        PqAuthorityReceiptStatus::FailClosed
    } else if !epoch_fresh {
        PqAuthorityReceiptStatus::StaleEpoch
    } else if config.require_authority_upgrade_separation && !upgrade_authority_separated {
        PqAuthorityReceiptStatus::UpgradeAuthorityConflict
    } else if !signature_verified
        || !execution_binding_verified
        || security_bits < config.min_security_bits
    {
        PqAuthorityReceiptStatus::InvalidSignature
    } else if source.release_held_count > 0 {
        PqAuthorityReceiptStatus::ReleaseHeld
    } else {
        PqAuthorityReceiptStatus::Verified
    }
}

fn authority_set_root(config: &Config, source: &SourceBundle, authority_epoch: u64) -> String {
    record_root(
        "authority-set",
        &json!({
            "verifier_suite": &config.verifier_suite,
            "authority_epoch": authority_epoch,
            "execution_state_root": &source.execution_state_root,
            "role": "pq_force_exit_execution_receipt_authority",
            "members": ["pq-authority:alpha", "pq-authority:bravo", "pq-authority:charlie"],
        }),
    )
}

fn upgrade_authority_root(config: &Config, source: &SourceBundle) -> String {
    record_root(
        "upgrade-authority",
        &json!({
            "verifier_suite": &config.verifier_suite,
            "execution_state_root": &source.execution_state_root,
            "role": "protocol_upgrade_authority",
            "members": ["upgrade-authority:delta", "upgrade-authority:echo", "upgrade-authority:foxtrot"],
        }),
    )
}

fn signer_key_root(config: &Config, authority_id: &str, authority_epoch: u64) -> String {
    record_root(
        "signer-key",
        &json!({
            "verifier_suite": &config.verifier_suite,
            "authority_id": authority_id,
            "authority_epoch": authority_epoch,
            "signature_scheme": "ml-dsa-87+slh-dsa-shake-256f",
            "key_scope": "force_exit_execution_receipt_only",
        }),
    )
}

fn signature_root(
    config: &Config,
    authority_id: &str,
    authority_epoch: u64,
    signer_key_root: &str,
    execution_receipt_root: &str,
    pq_privacy_receipt_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-AUTHORITY-SIGNATURE",
        &[
            HashPart::Str(&config.verifier_suite),
            HashPart::Str(authority_id),
            HashPart::U64(authority_epoch),
            HashPart::Str(signer_key_root),
            HashPart::Str(execution_receipt_root),
            HashPart::Str(pq_privacy_receipt_root),
        ],
        32,
    )
}

fn release_hold_root(
    config: &Config,
    source: &SourceBundle,
    authority_id: &str,
    authority_epoch: u64,
    status: PqAuthorityReceiptStatus,
    upgrade_authority_separated: bool,
    epoch_fresh: bool,
) -> String {
    record_root(
        "release-hold",
        &json!({
            "verifier_suite": &config.verifier_suite,
            "authority_id": authority_id,
            "authority_epoch": authority_epoch,
            "status": status.as_str(),
            "source_production_hold_root": &source.production_hold_root,
            "upgrade_authority_separated": upgrade_authority_separated,
            "epoch_fresh": epoch_fresh,
            "policy": "release_remains_held_until_fresh_pq_authority_quorum_verifies_force_exit_execution",
        }),
    )
}

fn fail_closed_root(
    config: &Config,
    source: &SourceBundle,
    authority_id: &str,
    authority_epoch: u64,
    status: PqAuthorityReceiptStatus,
    signature_verified: bool,
    execution_binding_verified: bool,
) -> String {
    record_root(
        "fail-closed",
        &json!({
            "verifier_suite": &config.verifier_suite,
            "authority_id": authority_id,
            "authority_epoch": authority_epoch,
            "status": status.as_str(),
            "recovery_receipt_root": &source.recovery_receipt_root,
            "signature_verified": signature_verified,
            "execution_binding_verified": execution_binding_verified,
            "policy": "missing_stale_or_mismatched_pq_authority_receipt_fails_closed",
        }),
    )
}

fn authority_receipt_root(
    config: &Config,
    source: &SourceBundle,
    authority_id: &str,
    authority_epoch: u64,
    authority_set_root: &str,
    upgrade_authority_root: &str,
    signer_key_root: &str,
    signature_root: &str,
    signed_execution_receipt_root: &str,
    signed_pq_privacy_receipt_root: &str,
    quorum_weight: u64,
    security_bits: u64,
    status: PqAuthorityReceiptStatus,
    release_hold_root: &str,
    fail_closed_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-AUTHORITY-RECEIPT",
        &[
            HashPart::Str(&config.verifier_suite),
            HashPart::Str(&source.execution_state_root),
            HashPart::Str(authority_id),
            HashPart::U64(authority_epoch),
            HashPart::Str(authority_set_root),
            HashPart::Str(upgrade_authority_root),
            HashPart::Str(signer_key_root),
            HashPart::Str(signature_root),
            HashPart::Str(signed_execution_receipt_root),
            HashPart::Str(signed_pq_privacy_receipt_root),
            HashPart::U64(quorum_weight),
            HashPart::U64(security_bits),
            HashPart::Str(status.as_str()),
            HashPart::Str(release_hold_root),
            HashPart::Str(fail_closed_root),
        ],
        32,
    )
}

fn receipt_id(authority_id: &str, ordinal: u64, receipt_root: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-AUTHORITY-RECEIPT-ID",
        &[
            HashPart::Str(authority_id),
            HashPart::U64(ordinal),
            HashPart::Str(receipt_root),
        ],
        16,
    )
}

fn required_outcome(status: PqAuthorityReceiptStatus) -> &'static str {
    match status {
        PqAuthorityReceiptStatus::Verified => {
            "fresh PQ authority signature contributes to the force-exit execution quorum"
        }
        PqAuthorityReceiptStatus::StaleEpoch => {
            "reject stale authority epoch and keep release held fail-closed"
        }
        PqAuthorityReceiptStatus::UpgradeAuthorityConflict => {
            "reject receipt because PQ signing authority overlaps protocol upgrade authority"
        }
        PqAuthorityReceiptStatus::InvalidSignature => {
            "reject mismatched or weak PQ signature and preserve recovery evidence"
        }
        PqAuthorityReceiptStatus::ReleaseHeld => {
            "release remains held until execution receipts and PQ authority quorum are live"
        }
        PqAuthorityReceiptStatus::FailClosed => {
            "fail closed and require wallet recovery path before release"
        }
    }
}

fn authority_receipt_vector_root(receipts: &[PqAuthorityReceipt]) -> String {
    merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-AUTHORITY-RECEIPTS",
        &receipts
            .iter()
            .map(PqAuthorityReceipt::public_record)
            .collect::<Vec<_>>(),
    )
}

fn quorum_receipt_root(
    config: &Config,
    source: &SourceBundle,
    receipts: &[PqAuthorityReceipt],
    verdict: &PqAuthorityReceiptVerdict,
) -> String {
    let quorum_records = receipts
        .iter()
        .map(|receipt| {
            json!({
                "receipt_id": &receipt.receipt_id,
                "authority_id": &receipt.authority_id,
                "quorum_weight": receipt.quorum_weight,
                "status": receipt.status.as_str(),
                "signature_root": &receipt.signature_root,
            })
        })
        .collect::<Vec<_>>();
    let quorum_vector_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-AUTHORITY-QUORUM-VECTOR",
        &quorum_records,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-AUTHORITY-QUORUM",
        &[
            HashPart::Str(&config.verifier_suite),
            HashPart::Str(&source.execution_receipt_root),
            HashPart::Str(&quorum_vector_root),
            HashPart::U64(verdict.observed_quorum_weight),
            HashPart::U64(verdict.required_quorum_weight),
            HashPart::Str(bool_str(verdict.quorum_verified)),
        ],
        32,
    )
}

fn epoch_guard_root(
    config: &Config,
    source: &SourceBundle,
    receipts: &[PqAuthorityReceipt],
    verdict: &PqAuthorityReceiptVerdict,
) -> String {
    let epoch_records = receipts
        .iter()
        .map(|receipt| {
            json!({
                "receipt_id": &receipt.receipt_id,
                "authority_epoch": receipt.authority_epoch,
                "current_authority_epoch": config.current_authority_epoch,
                "epoch_fresh": receipt.epoch_fresh,
            })
        })
        .collect::<Vec<_>>();
    let epoch_vector_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-AUTHORITY-EPOCHS",
        &epoch_records,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-AUTHORITY-EPOCH-GUARD",
        &[
            HashPart::Str(&config.verifier_suite),
            HashPart::Str(&source.execution_state_root),
            HashPart::Str(&epoch_vector_root),
            HashPart::U64(config.current_authority_epoch),
            HashPart::U64(config.max_epoch_lag),
            HashPart::U64(verdict.stale_epoch_count),
            HashPart::Str(bool_str(verdict.all_epochs_fresh)),
        ],
        32,
    )
}

fn upgrade_separation_root(
    config: &Config,
    source: &SourceBundle,
    receipts: &[PqAuthorityReceipt],
    verdict: &PqAuthorityReceiptVerdict,
) -> String {
    let separation_records = receipts
        .iter()
        .map(|receipt| {
            json!({
                "receipt_id": &receipt.receipt_id,
                "authority_set_root": &receipt.authority_set_root,
                "upgrade_authority_root": &receipt.upgrade_authority_root,
                "upgrade_authority_separated": receipt.upgrade_authority_separated,
            })
        })
        .collect::<Vec<_>>();
    let separation_vector_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-AUTHORITY-UPGRADE-SEPARATION",
        &separation_records,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-AUTHORITY-UPGRADE-SEPARATION-GUARD",
        &[
            HashPart::Str(&config.verifier_suite),
            HashPart::Str(&source.execution_state_root),
            HashPart::Str(&separation_vector_root),
            HashPart::U64(verdict.upgrade_conflict_count),
            HashPart::Str(bool_str(verdict.authority_upgrade_separated)),
        ],
        32,
    )
}

fn release_hold_bundle_root(
    config: &Config,
    source: &SourceBundle,
    receipts: &[PqAuthorityReceipt],
    verdict: &PqAuthorityReceiptVerdict,
) -> String {
    let release_holds = receipts
        .iter()
        .map(|receipt| {
            json!({
                "receipt_id": &receipt.receipt_id,
                "status": receipt.status.as_str(),
                "release_hold_root": &receipt.release_hold_root,
            })
        })
        .collect::<Vec<_>>();
    let release_hold_vector_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-AUTHORITY-RELEASE-HOLDS",
        &release_holds,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-AUTHORITY-RELEASE-HOLD-BUNDLE",
        &[
            HashPart::Str(&config.verifier_suite),
            HashPart::Str(&source.production_hold_root),
            HashPart::Str(&release_hold_vector_root),
            HashPart::U64(verdict.release_held_count),
            HashPart::Str(bool_str(verdict.release_held)),
            HashPart::Str(bool_str(verdict.production_blocked)),
        ],
        32,
    )
}

fn fail_closed_bundle_root(
    config: &Config,
    source: &SourceBundle,
    receipts: &[PqAuthorityReceipt],
    verdict: &PqAuthorityReceiptVerdict,
) -> String {
    let fail_closed_records = receipts
        .iter()
        .map(|receipt| {
            json!({
                "receipt_id": &receipt.receipt_id,
                "status": receipt.status.as_str(),
                "fail_closed_root": &receipt.fail_closed_root,
                "signature_verified": receipt.signature_verified,
                "execution_binding_verified": receipt.execution_binding_verified,
            })
        })
        .collect::<Vec<_>>();
    let fail_closed_vector_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-AUTHORITY-FAIL-CLOSED",
        &fail_closed_records,
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-AUTHORITY-FAIL-CLOSED-BUNDLE",
        &[
            HashPart::Str(&config.verifier_suite),
            HashPart::Str(&source.recovery_receipt_root),
            HashPart::Str(&fail_closed_vector_root),
            HashPart::U64(verdict.fail_closed_count),
            HashPart::Str(bool_str(verdict.fail_closed)),
        ],
        32,
    )
}

fn state_commitment_root(
    config: &Config,
    source: &SourceBundle,
    authority_receipt_root: &str,
    quorum_receipt_root: &str,
    epoch_guard_root: &str,
    upgrade_separation_root: &str,
    release_hold_root: &str,
    fail_closed_root: &str,
    verdict: &PqAuthorityReceiptVerdict,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-AUTHORITY-RECEIPT-VERIFIER-STATE",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&source.state_root()),
            HashPart::Str(authority_receipt_root),
            HashPart::Str(quorum_receipt_root),
            HashPart::Str(epoch_guard_root),
            HashPart::Str(upgrade_separation_root),
            HashPart::Str(release_hold_root),
            HashPart::Str(fail_closed_root),
            HashPart::Str(&verdict.verdict_root),
        ],
        32,
    )
}

fn verdict_root(
    config: &Config,
    source: &SourceBundle,
    receipt_count: u64,
    verified_receipt_count: u64,
    stale_epoch_count: u64,
    upgrade_conflict_count: u64,
    invalid_signature_count: u64,
    observed_quorum_weight: u64,
    quorum_verified: bool,
    signature_threshold_met: bool,
    all_epochs_fresh: bool,
    authority_upgrade_separated: bool,
    execution_binding_verified: bool,
    fail_closed: bool,
    production_blocked: bool,
    verifier_status: &str,
    user_escape_answer: &str,
    production_answer: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-AUTHORITY-RECEIPT-VERIFIER-VERDICT",
        &[
            HashPart::Str(&config.verifier_suite),
            HashPart::Str(&source.execution_state_root),
            HashPart::Str(&source.execution_receipt_root),
            HashPart::U64(receipt_count),
            HashPart::U64(verified_receipt_count),
            HashPart::U64(stale_epoch_count),
            HashPart::U64(upgrade_conflict_count),
            HashPart::U64(invalid_signature_count),
            HashPart::U64(observed_quorum_weight),
            HashPart::Str(bool_str(quorum_verified)),
            HashPart::Str(bool_str(signature_threshold_met)),
            HashPart::Str(bool_str(all_epochs_fresh)),
            HashPart::Str(bool_str(authority_upgrade_separated)),
            HashPart::Str(bool_str(execution_binding_verified)),
            HashPart::Str(bool_str(fail_closed)),
            HashPart::Str(bool_str(production_blocked)),
            HashPart::Str(verifier_status),
            HashPart::Str(user_escape_answer),
            HashPart::Str(production_answer),
        ],
        32,
    )
}

fn count_status(receipts: &[PqAuthorityReceipt], status: PqAuthorityReceiptStatus) -> u64 {
    receipts
        .iter()
        .filter(|receipt| receipt.status == status)
        .count() as u64
}

fn epoch_is_fresh(config: &Config, authority_epoch: u64) -> bool {
    authority_epoch <= config.current_authority_epoch
        && config.current_authority_epoch - authority_epoch <= config.max_epoch_lag
}

fn devnet_weight(ordinal: u64) -> u64 {
    match ordinal {
        1 => 34,
        2 => 33,
        _ => 33,
    }
}

fn validate_config(config: &Config) -> Result<()> {
    ensure(
        config.chain_id == CHAIN_ID,
        "PQ authority receipt verifier chain mismatch",
    )?;
    ensure(
        config.protocol_version == PROTOCOL_VERSION,
        "PQ authority receipt verifier protocol mismatch",
    )?;
    ensure(
        config.required_quorum_weight > 0,
        "PQ authority receipt verifier requires quorum weight",
    )?;
    ensure(
        config.min_signature_count > 0,
        "PQ authority receipt verifier requires signatures",
    )?;
    Ok(())
}

fn validate_source(source: &SourceBundle) -> Result<()> {
    ensure(
        !source.execution_state_root.is_empty(),
        "PQ authority receipt verifier missing execution state root",
    )?;
    ensure(
        !source.execution_receipt_root.is_empty(),
        "PQ authority receipt verifier missing execution receipt root",
    )?;
    ensure(
        !source.pq_privacy_receipt_root.is_empty(),
        "PQ authority receipt verifier missing PQ privacy receipt root",
    )?;
    ensure(
        source.execution_receipt_count > 0,
        "PQ authority receipt verifier missing execution receipts",
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
        execution_state_root: record_root("fallback-execution-state", &json!({"reason": &reason})),
        execution_receipt_root: record_root(
            "fallback-execution-receipt",
            &json!({"reason": &reason}),
        ),
        pq_privacy_receipt_root: record_root("fallback-pq-privacy", &json!({"reason": &reason})),
        recovery_receipt_root: record_root("fallback-recovery", &json!({"reason": &reason})),
        production_hold_root: record_root("fallback-production-hold", &json!({"reason": &reason})),
        execution_status: "fallback".to_string(),
        execution_user_escape_answer: reason.clone(),
        execution_production_answer: "fallback".to_string(),
        execution_receipt_count: 1,
        observed_receipt_count: 0,
        deferred_receipt_count: 0,
        release_held_count: 0,
        fail_closed_count: 1,
        production_blocker_count: 1,
        package_execution_observed: false,
        user_escape_execution_observed: false,
        execution_production_blocked: true,
    };
    let authority_receipts = vec![PqAuthorityReceipt::devnet(
        &config,
        &source,
        1,
        "pq-authority:fallback",
    )];
    let verdict = PqAuthorityReceiptVerdict::new(&config, &source, &authority_receipts);
    let authority_receipt_root = authority_receipt_vector_root(&authority_receipts);
    let quorum_receipt_root = quorum_receipt_root(&config, &source, &authority_receipts, &verdict);
    let epoch_guard_root = epoch_guard_root(&config, &source, &authority_receipts, &verdict);
    let upgrade_separation_root =
        upgrade_separation_root(&config, &source, &authority_receipts, &verdict);
    let release_hold_root =
        release_hold_bundle_root(&config, &source, &authority_receipts, &verdict);
    let fail_closed_root = fail_closed_bundle_root(&config, &source, &authority_receipts, &verdict);
    let state_commitment_root = state_commitment_root(
        &config,
        &source,
        &authority_receipt_root,
        &quorum_receipt_root,
        &epoch_guard_root,
        &upgrade_separation_root,
        &release_hold_root,
        &fail_closed_root,
        &verdict,
    );
    State {
        config,
        source,
        authority_receipts,
        verdict,
        authority_receipt_root,
        quorum_receipt_root,
        epoch_guard_root,
        upgrade_separation_root,
        release_hold_root,
        fail_closed_root,
        state_commitment_root,
    }
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-AUTHORITY-RECEIPT-VERIFIER-RECORD",
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
