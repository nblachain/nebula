use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageBridgeCustodyLiveReleaseReceiptVerifierRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_BRIDGE_CUSTODY_LIVE_RELEASE_RECEIPT_VERIFIER_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-bridge-custody-live-release-receipt-verifier-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_BRIDGE_CUSTODY_LIVE_RELEASE_RECEIPT_VERIFIER_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const LIVE_RELEASE_RECEIPT_VERIFIER_SUITE: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-force-exit-package-bridge-custody-live-release-receipt-verifier-v1";
pub const DEFAULT_MIN_CUSTODY_SIGNER_ROOTS: u64 = 3;
pub const DEFAULT_MIN_RESERVE_PROOF_ROOTS: u64 = 2;
pub const DEFAULT_MIN_CHALLENGE_WINDOW_ROOTS: u64 = 2;
pub const DEFAULT_MIN_MONERO_RELEASE_OBSERVATION_ROOTS: u64 = 2;
pub const DEFAULT_MIN_WALLET_FINALITY_ROOTS: u64 = 2;
pub const DEFAULT_MIN_RELEASE_AUTHORIZATION_ROOTS: u64 = 2;
pub const DEFAULT_REQUIRED_FINALITY_CONFIRMATIONS: u64 = 18;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub verifier_suite: String,
    pub min_custody_signer_roots: u64,
    pub min_reserve_proof_roots: u64,
    pub min_challenge_window_roots: u64,
    pub min_monero_release_observation_roots: u64,
    pub min_wallet_finality_roots: u64,
    pub min_release_authorization_roots: u64,
    pub required_finality_confirmations: u64,
    pub require_custody_signer_roots: bool,
    pub require_reserve_proof_roots: bool,
    pub require_challenge_window_roots: bool,
    pub require_monero_release_tx_observation_roots: bool,
    pub require_wallet_finality_roots: bool,
    pub require_release_authorization_roots: bool,
    pub require_matching_release_amount: bool,
    pub require_matching_destination: bool,
    pub require_fail_closed_verdict: bool,
    pub hold_production_until_live_release_verified: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            verifier_suite: LIVE_RELEASE_RECEIPT_VERIFIER_SUITE.to_string(),
            min_custody_signer_roots: DEFAULT_MIN_CUSTODY_SIGNER_ROOTS,
            min_reserve_proof_roots: DEFAULT_MIN_RESERVE_PROOF_ROOTS,
            min_challenge_window_roots: DEFAULT_MIN_CHALLENGE_WINDOW_ROOTS,
            min_monero_release_observation_roots: DEFAULT_MIN_MONERO_RELEASE_OBSERVATION_ROOTS,
            min_wallet_finality_roots: DEFAULT_MIN_WALLET_FINALITY_ROOTS,
            min_release_authorization_roots: DEFAULT_MIN_RELEASE_AUTHORIZATION_ROOTS,
            required_finality_confirmations: DEFAULT_REQUIRED_FINALITY_CONFIRMATIONS,
            require_custody_signer_roots: true,
            require_reserve_proof_roots: true,
            require_challenge_window_roots: true,
            require_monero_release_tx_observation_roots: true,
            require_wallet_finality_roots: true,
            require_release_authorization_roots: true,
            require_matching_release_amount: true,
            require_matching_destination: true,
            require_fail_closed_verdict: true,
            hold_production_until_live_release_verified: true,
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
            "min_custody_signer_roots": self.min_custody_signer_roots,
            "min_reserve_proof_roots": self.min_reserve_proof_roots,
            "min_challenge_window_roots": self.min_challenge_window_roots,
            "min_monero_release_observation_roots": self.min_monero_release_observation_roots,
            "min_wallet_finality_roots": self.min_wallet_finality_roots,
            "min_release_authorization_roots": self.min_release_authorization_roots,
            "required_finality_confirmations": self.required_finality_confirmations,
            "require_custody_signer_roots": self.require_custody_signer_roots,
            "require_reserve_proof_roots": self.require_reserve_proof_roots,
            "require_challenge_window_roots": self.require_challenge_window_roots,
            "require_monero_release_tx_observation_roots": self.require_monero_release_tx_observation_roots,
            "require_wallet_finality_roots": self.require_wallet_finality_roots,
            "require_release_authorization_roots": self.require_release_authorization_roots,
            "require_matching_release_amount": self.require_matching_release_amount,
            "require_matching_destination": self.require_matching_destination,
            "require_fail_closed_verdict": self.require_fail_closed_verdict,
            "hold_production_until_live_release_verified": self.hold_production_until_live_release_verified,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Roots {
    pub custody_signer_root: String,
    pub custody_signer_bundle_root: String,
    pub reserve_proof_root: String,
    pub reserve_proof_bundle_root: String,
    pub challenge_window_root: String,
    pub challenge_window_bundle_root: String,
    pub monero_release_tx_observation_root: String,
    pub monero_release_tx_bundle_root: String,
    pub wallet_finality_root: String,
    pub wallet_finality_bundle_root: String,
    pub release_authorization_root: String,
    pub release_authorization_bundle_root: String,
    pub mismatch_finding_root: String,
    pub fail_closed_verdict_root: String,
    pub live_release_receipt_root: String,
}

impl Roots {
    pub fn new(
        _config: &Config,
        receipts: &[LiveReleaseReceipt],
        findings: &[MismatchFinding],
        verdict: &FailClosedVerdict,
    ) -> Self {
        Self {
            custody_signer_root: typed_receipt_root("custody-signer-roots", receipts, |receipt| {
                receipt.custody_signer_root.clone()
            }),
            custody_signer_bundle_root: typed_receipt_root(
                "custody-signer-bundle-roots",
                receipts,
                |receipt| receipt.custody_signer_bundle_root.clone(),
            ),
            reserve_proof_root: typed_receipt_root("reserve-proof-roots", receipts, |receipt| {
                receipt.reserve_proof_root.clone()
            }),
            reserve_proof_bundle_root: typed_receipt_root(
                "reserve-proof-bundle-roots",
                receipts,
                |receipt| receipt.reserve_proof_bundle_root.clone(),
            ),
            challenge_window_root: typed_receipt_root(
                "challenge-window-roots",
                receipts,
                |receipt| receipt.challenge_window_root.clone(),
            ),
            challenge_window_bundle_root: typed_receipt_root(
                "challenge-window-bundle-roots",
                receipts,
                |receipt| receipt.challenge_window_bundle_root.clone(),
            ),
            monero_release_tx_observation_root: typed_receipt_root(
                "monero-release-tx-observation-roots",
                receipts,
                |receipt| receipt.monero_release_tx_observation_root.clone(),
            ),
            monero_release_tx_bundle_root: typed_receipt_root(
                "monero-release-tx-bundle-roots",
                receipts,
                |receipt| receipt.monero_release_tx_bundle_root.clone(),
            ),
            wallet_finality_root: typed_receipt_root(
                "wallet-finality-roots",
                receipts,
                |receipt| receipt.wallet_finality_root.clone(),
            ),
            wallet_finality_bundle_root: typed_receipt_root(
                "wallet-finality-bundle-roots",
                receipts,
                |receipt| receipt.wallet_finality_bundle_root.clone(),
            ),
            release_authorization_root: typed_receipt_root(
                "release-authorization-roots",
                receipts,
                |receipt| receipt.release_authorization_root.clone(),
            ),
            release_authorization_bundle_root: typed_receipt_root(
                "release-authorization-bundle-roots",
                receipts,
                |receipt| receipt.release_authorization_bundle_root.clone(),
            ),
            mismatch_finding_root: mismatch_finding_vector_root(findings),
            fail_closed_verdict_root: record_root("fail-closed-verdict", &verdict.public_record()),
            live_release_receipt_root: live_release_receipt_vector_root(receipts),
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "custody_signer_root": self.custody_signer_root,
            "custody_signer_bundle_root": self.custody_signer_bundle_root,
            "reserve_proof_root": self.reserve_proof_root,
            "reserve_proof_bundle_root": self.reserve_proof_bundle_root,
            "challenge_window_root": self.challenge_window_root,
            "challenge_window_bundle_root": self.challenge_window_bundle_root,
            "monero_release_tx_observation_root": self.monero_release_tx_observation_root,
            "monero_release_tx_bundle_root": self.monero_release_tx_bundle_root,
            "wallet_finality_root": self.wallet_finality_root,
            "wallet_finality_bundle_root": self.wallet_finality_bundle_root,
            "release_authorization_root": self.release_authorization_root,
            "release_authorization_bundle_root": self.release_authorization_bundle_root,
            "mismatch_finding_root": self.mismatch_finding_root,
            "fail_closed_verdict_root": self.fail_closed_verdict_root,
            "live_release_receipt_root": self.live_release_receipt_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("roots", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub receipt_count: u64,
    pub verified_receipt_count: u64,
    pub custody_signer_root_count: u64,
    pub reserve_proof_root_count: u64,
    pub challenge_window_root_count: u64,
    pub monero_release_observation_root_count: u64,
    pub wallet_finality_root_count: u64,
    pub release_authorization_root_count: u64,
    pub mismatch_finding_count: u64,
    pub amount_mismatch_count: u64,
    pub destination_mismatch_count: u64,
    pub root_mismatch_count: u64,
    pub finality_shortfall_count: u64,
    pub challenge_window_blocker_count: u64,
    pub fail_closed_count: u64,
    pub user_release_blocker_count: u64,
    pub production_blocker_count: u64,
}

impl Counters {
    pub fn new(receipts: &[LiveReleaseReceipt], findings: &[MismatchFinding]) -> Self {
        let verified_receipt_count = receipts
            .iter()
            .filter(|receipt| receipt.receipt_verified)
            .count() as u64;
        let amount_mismatch_count = findings
            .iter()
            .filter(|finding| finding.kind == MismatchKind::Amount)
            .count() as u64;
        let destination_mismatch_count = findings
            .iter()
            .filter(|finding| finding.kind == MismatchKind::Destination)
            .count() as u64;
        let root_mismatch_count = findings
            .iter()
            .filter(|finding| finding.kind == MismatchKind::Root)
            .count() as u64;
        let finality_shortfall_count = findings
            .iter()
            .filter(|finding| finding.kind == MismatchKind::Finality)
            .count() as u64;
        let challenge_window_blocker_count = findings
            .iter()
            .filter(|finding| finding.kind == MismatchKind::ChallengeWindow)
            .count() as u64;
        let fail_closed_count = receipts
            .iter()
            .filter(|receipt| receipt.fail_closed_required)
            .count() as u64;
        let production_blocker_count = fail_closed_count + challenge_window_blocker_count;
        Self {
            receipt_count: receipts.len() as u64,
            verified_receipt_count,
            custody_signer_root_count: populated_count(receipts, |receipt| {
                receipt.custody_signer_root.clone()
            }),
            reserve_proof_root_count: populated_count(receipts, |receipt| {
                receipt.reserve_proof_root.clone()
            }),
            challenge_window_root_count: populated_count(receipts, |receipt| {
                receipt.challenge_window_root.clone()
            }),
            monero_release_observation_root_count: populated_count(receipts, |receipt| {
                receipt.monero_release_tx_observation_root.clone()
            }),
            wallet_finality_root_count: populated_count(receipts, |receipt| {
                receipt.wallet_finality_root.clone()
            }),
            release_authorization_root_count: populated_count(receipts, |receipt| {
                receipt.release_authorization_root.clone()
            }),
            mismatch_finding_count: findings.len() as u64,
            amount_mismatch_count,
            destination_mismatch_count,
            root_mismatch_count,
            finality_shortfall_count,
            challenge_window_blocker_count,
            fail_closed_count,
            user_release_blocker_count: fail_closed_count + findings.len() as u64,
            production_blocker_count,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "receipt_count": self.receipt_count,
            "verified_receipt_count": self.verified_receipt_count,
            "custody_signer_root_count": self.custody_signer_root_count,
            "reserve_proof_root_count": self.reserve_proof_root_count,
            "challenge_window_root_count": self.challenge_window_root_count,
            "monero_release_observation_root_count": self.monero_release_observation_root_count,
            "wallet_finality_root_count": self.wallet_finality_root_count,
            "release_authorization_root_count": self.release_authorization_root_count,
            "mismatch_finding_count": self.mismatch_finding_count,
            "amount_mismatch_count": self.amount_mismatch_count,
            "destination_mismatch_count": self.destination_mismatch_count,
            "root_mismatch_count": self.root_mismatch_count,
            "finality_shortfall_count": self.finality_shortfall_count,
            "challenge_window_blocker_count": self.challenge_window_blocker_count,
            "fail_closed_count": self.fail_closed_count,
            "user_release_blocker_count": self.user_release_blocker_count,
            "production_blocker_count": self.production_blocker_count,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("counters", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MismatchKind {
    Amount,
    Destination,
    Root,
    Finality,
    ChallengeWindow,
    Authorization,
}

impl MismatchKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Amount => "amount",
            Self::Destination => "destination",
            Self::Root => "root",
            Self::Finality => "finality",
            Self::ChallengeWindow => "challenge_window",
            Self::Authorization => "authorization",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LiveReleaseReceipt {
    pub receipt_id: String,
    pub ordinal: u64,
    pub release_id: String,
    pub expected_release_amount_piconero: u64,
    pub observed_release_amount_piconero: u64,
    pub expected_destination_root: String,
    pub observed_destination_root: String,
    pub custody_signer_root: String,
    pub custody_signer_bundle_root: String,
    pub reserve_proof_root: String,
    pub reserve_proof_bundle_root: String,
    pub challenge_window_root: String,
    pub challenge_window_bundle_root: String,
    pub monero_release_tx_observation_root: String,
    pub monero_release_tx_bundle_root: String,
    pub wallet_finality_root: String,
    pub wallet_finality_bundle_root: String,
    pub release_authorization_root: String,
    pub release_authorization_bundle_root: String,
    pub finality_confirmations: u64,
    pub custody_signers_verified: bool,
    pub reserve_proof_verified: bool,
    pub challenge_window_clear: bool,
    pub monero_release_tx_observed: bool,
    pub wallet_finality_verified: bool,
    pub release_authorized: bool,
    pub receipt_verified: bool,
    pub fail_closed_required: bool,
    pub receipt_root: String,
}

impl LiveReleaseReceipt {
    pub fn devnet(config: &Config, ordinal: u64) -> Self {
        let release_id = format!("force-exit-live-release-devnet-{ordinal:04}");
        let expected_release_amount_piconero = 1_000_000_000_000 + ordinal;
        let observed_release_amount_piconero = expected_release_amount_piconero;
        let expected_destination_root = record_root(
            "devnet-expected-destination",
            &json!({"release_id": release_id, "ordinal": ordinal}),
        );
        let observed_destination_root = expected_destination_root.clone();
        let custody_signer_root = receipt_part_root("custody-signer", &release_id, ordinal);
        let custody_signer_bundle_root =
            receipt_part_root("custody-signer-bundle", &release_id, ordinal);
        let reserve_proof_root = receipt_part_root("reserve-proof", &release_id, ordinal);
        let reserve_proof_bundle_root =
            receipt_part_root("reserve-proof-bundle", &release_id, ordinal);
        let challenge_window_root = receipt_part_root("challenge-window", &release_id, ordinal);
        let challenge_window_bundle_root =
            receipt_part_root("challenge-window-bundle", &release_id, ordinal);
        let monero_release_tx_observation_root =
            receipt_part_root("monero-release-tx-observation", &release_id, ordinal);
        let monero_release_tx_bundle_root =
            receipt_part_root("monero-release-tx-bundle", &release_id, ordinal);
        let wallet_finality_root = receipt_part_root("wallet-finality", &release_id, ordinal);
        let wallet_finality_bundle_root =
            receipt_part_root("wallet-finality-bundle", &release_id, ordinal);
        let release_authorization_root =
            receipt_part_root("release-authorization", &release_id, ordinal);
        let release_authorization_bundle_root =
            receipt_part_root("release-authorization-bundle", &release_id, ordinal);
        let finality_confirmations = config.required_finality_confirmations + ordinal;
        let custody_signers_verified = true;
        let reserve_proof_verified = true;
        let challenge_window_clear = true;
        let monero_release_tx_observed = true;
        let wallet_finality_verified = true;
        let release_authorized = true;
        let receipt_verified = true;
        let fail_closed_required = false;
        let mut receipt = Self {
            receipt_id: record_root(
                "devnet-live-release-receipt-id",
                &json!({"release_id": release_id, "ordinal": ordinal}),
            ),
            ordinal,
            release_id,
            expected_release_amount_piconero,
            observed_release_amount_piconero,
            expected_destination_root,
            observed_destination_root,
            custody_signer_root,
            custody_signer_bundle_root,
            reserve_proof_root,
            reserve_proof_bundle_root,
            challenge_window_root,
            challenge_window_bundle_root,
            monero_release_tx_observation_root,
            monero_release_tx_bundle_root,
            wallet_finality_root,
            wallet_finality_bundle_root,
            release_authorization_root,
            release_authorization_bundle_root,
            finality_confirmations,
            custody_signers_verified,
            reserve_proof_verified,
            challenge_window_clear,
            monero_release_tx_observed,
            wallet_finality_verified,
            release_authorized,
            receipt_verified,
            fail_closed_required,
            receipt_root: String::new(),
        };
        receipt.receipt_root = record_root("live-release-receipt", &receipt.public_record());
        receipt
    }

    pub fn public_record(&self) -> Value {
        json!({
            "receipt_id": self.receipt_id,
            "ordinal": self.ordinal,
            "release_id": self.release_id,
            "expected_release_amount_piconero": self.expected_release_amount_piconero,
            "observed_release_amount_piconero": self.observed_release_amount_piconero,
            "expected_destination_root": self.expected_destination_root,
            "observed_destination_root": self.observed_destination_root,
            "custody_signer_root": self.custody_signer_root,
            "custody_signer_bundle_root": self.custody_signer_bundle_root,
            "reserve_proof_root": self.reserve_proof_root,
            "reserve_proof_bundle_root": self.reserve_proof_bundle_root,
            "challenge_window_root": self.challenge_window_root,
            "challenge_window_bundle_root": self.challenge_window_bundle_root,
            "monero_release_tx_observation_root": self.monero_release_tx_observation_root,
            "monero_release_tx_bundle_root": self.monero_release_tx_bundle_root,
            "wallet_finality_root": self.wallet_finality_root,
            "wallet_finality_bundle_root": self.wallet_finality_bundle_root,
            "release_authorization_root": self.release_authorization_root,
            "release_authorization_bundle_root": self.release_authorization_bundle_root,
            "finality_confirmations": self.finality_confirmations,
            "custody_signers_verified": self.custody_signers_verified,
            "reserve_proof_verified": self.reserve_proof_verified,
            "challenge_window_clear": self.challenge_window_clear,
            "monero_release_tx_observed": self.monero_release_tx_observed,
            "wallet_finality_verified": self.wallet_finality_verified,
            "release_authorized": self.release_authorized,
            "receipt_verified": self.receipt_verified,
            "fail_closed_required": self.fail_closed_required,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MismatchFinding {
    pub finding_id: String,
    pub receipt_id: String,
    pub release_id: String,
    pub kind: MismatchKind,
    pub expected_root: String,
    pub observed_root: String,
    pub expected_amount_piconero: u64,
    pub observed_amount_piconero: u64,
    pub fail_closed_required: bool,
    pub user_release_blocked: bool,
    pub production_blocked: bool,
}

impl MismatchFinding {
    pub fn public_record(&self) -> Value {
        json!({
            "finding_id": self.finding_id,
            "receipt_id": self.receipt_id,
            "release_id": self.release_id,
            "kind": self.kind.as_str(),
            "expected_root": self.expected_root,
            "observed_root": self.observed_root,
            "expected_amount_piconero": self.expected_amount_piconero,
            "observed_amount_piconero": self.observed_amount_piconero,
            "fail_closed_required": self.fail_closed_required,
            "user_release_blocked": self.user_release_blocked,
            "production_blocked": self.production_blocked,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FailClosedVerdict {
    pub release_receipts_verified: bool,
    pub all_required_roots_present: bool,
    pub custody_signers_verified: bool,
    pub reserve_proofs_verified: bool,
    pub challenge_windows_clear: bool,
    pub monero_release_transactions_observed: bool,
    pub wallet_finality_verified: bool,
    pub release_authorizations_verified: bool,
    pub release_amounts_match: bool,
    pub destinations_match: bool,
    pub mismatch_free: bool,
    pub fail_closed: bool,
    pub user_release_allowed: bool,
    pub production_allowed: bool,
    pub user_escape_answer: String,
    pub production_answer: String,
}

impl FailClosedVerdict {
    pub fn new(config: &Config, counters: &Counters, findings: &[MismatchFinding]) -> Self {
        let release_receipts_verified =
            counters.receipt_count > 0 && counters.verified_receipt_count == counters.receipt_count;
        let all_required_roots_present = (!config.require_custody_signer_roots
            || counters.custody_signer_root_count >= config.min_custody_signer_roots)
            && (!config.require_reserve_proof_roots
                || counters.reserve_proof_root_count >= config.min_reserve_proof_roots)
            && (!config.require_challenge_window_roots
                || counters.challenge_window_root_count >= config.min_challenge_window_roots)
            && (!config.require_monero_release_tx_observation_roots
                || counters.monero_release_observation_root_count
                    >= config.min_monero_release_observation_roots)
            && (!config.require_wallet_finality_roots
                || counters.wallet_finality_root_count >= config.min_wallet_finality_roots)
            && (!config.require_release_authorization_roots
                || counters.release_authorization_root_count
                    >= config.min_release_authorization_roots);
        let custody_signers_verified =
            counters.custody_signer_root_count >= config.min_custody_signer_roots;
        let reserve_proofs_verified =
            counters.reserve_proof_root_count >= config.min_reserve_proof_roots;
        let challenge_windows_clear = counters.challenge_window_blocker_count == 0
            && counters.challenge_window_root_count >= config.min_challenge_window_roots;
        let monero_release_transactions_observed = counters.monero_release_observation_root_count
            >= config.min_monero_release_observation_roots;
        let wallet_finality_verified = counters.finality_shortfall_count == 0
            && counters.wallet_finality_root_count >= config.min_wallet_finality_roots;
        let release_authorizations_verified =
            counters.release_authorization_root_count >= config.min_release_authorization_roots;
        let release_amounts_match =
            !config.require_matching_release_amount || counters.amount_mismatch_count == 0;
        let destinations_match =
            !config.require_matching_destination || counters.destination_mismatch_count == 0;
        let mismatch_free = findings.is_empty();
        let fail_closed = config.require_fail_closed_verdict
            && (!release_receipts_verified
                || !all_required_roots_present
                || !custody_signers_verified
                || !reserve_proofs_verified
                || !challenge_windows_clear
                || !monero_release_transactions_observed
                || !wallet_finality_verified
                || !release_authorizations_verified
                || !release_amounts_match
                || !destinations_match
                || !mismatch_free);
        let user_release_allowed = !fail_closed && counters.user_release_blocker_count == 0;
        let production_allowed = !config.hold_production_until_live_release_verified
            || (!fail_closed && counters.production_blocker_count == 0);
        let user_escape_answer = if user_release_allowed {
            "live bridge custody release receipt verified for user escape".to_string()
        } else {
            "fail closed: live bridge custody release receipt is not user-releaseable".to_string()
        };
        let production_answer = if production_allowed {
            "production live release receipt verification may proceed".to_string()
        } else {
            "hold production until live release receipt verification is mismatch-free".to_string()
        };
        Self {
            release_receipts_verified,
            all_required_roots_present,
            custody_signers_verified,
            reserve_proofs_verified,
            challenge_windows_clear,
            monero_release_transactions_observed,
            wallet_finality_verified,
            release_authorizations_verified,
            release_amounts_match,
            destinations_match,
            mismatch_free,
            fail_closed,
            user_release_allowed,
            production_allowed,
            user_escape_answer,
            production_answer,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "release_receipts_verified": self.release_receipts_verified,
            "all_required_roots_present": self.all_required_roots_present,
            "custody_signers_verified": self.custody_signers_verified,
            "reserve_proofs_verified": self.reserve_proofs_verified,
            "challenge_windows_clear": self.challenge_windows_clear,
            "monero_release_transactions_observed": self.monero_release_transactions_observed,
            "wallet_finality_verified": self.wallet_finality_verified,
            "release_authorizations_verified": self.release_authorizations_verified,
            "release_amounts_match": self.release_amounts_match,
            "destinations_match": self.destinations_match,
            "mismatch_free": self.mismatch_free,
            "fail_closed": self.fail_closed,
            "user_release_allowed": self.user_release_allowed,
            "production_allowed": self.production_allowed,
            "user_escape_answer": self.user_escape_answer,
            "production_answer": self.production_answer,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub roots: Roots,
    pub counters: Counters,
    pub live_release_receipts: Vec<LiveReleaseReceipt>,
    pub mismatch_findings: Vec<MismatchFinding>,
    pub verdict: FailClosedVerdict,
    pub state_commitment_root: String,
}

impl State {
    pub fn new(config: Config, live_release_receipts: Vec<LiveReleaseReceipt>) -> Result<Self> {
        validate_config(&config)?;
        validate_receipts(&live_release_receipts)?;
        let mismatch_findings = derive_mismatch_findings(&config, &live_release_receipts);
        let counters = Counters::new(&live_release_receipts, &mismatch_findings);
        let verdict = FailClosedVerdict::new(&config, &counters, &mismatch_findings);
        let roots = Roots::new(
            &config,
            &live_release_receipts,
            &mismatch_findings,
            &verdict,
        );
        let state_commitment_root = state_commitment_root(
            &config,
            &roots,
            &live_release_receipts,
            &mismatch_findings,
            &verdict,
        );
        Ok(Self {
            config,
            roots,
            counters,
            live_release_receipts,
            mismatch_findings,
            verdict,
            state_commitment_root,
        })
    }

    pub fn devnet() -> Self {
        devnet()
    }

    pub fn public_record(&self) -> Value {
        json!({
            "config": self.config.public_record(),
            "roots": self.roots.public_record(),
            "counters": self.counters.public_record(),
            "live_release_receipts": self.live_release_receipts.iter().map(LiveReleaseReceipt::public_record).collect::<Vec<_>>(),
            "mismatch_findings": self.mismatch_findings.iter().map(MismatchFinding::public_record).collect::<Vec<_>>(),
            "verdict": self.verdict.public_record(),
            "state_commitment_root": self.state_commitment_root,
        })
    }

    pub fn state_root(&self) -> String {
        self.state_commitment_root.clone()
    }
}

pub fn devnet() -> State {
    let config = Config::devnet();
    let receipts = (1..=config.min_custody_signer_roots)
        .map(|ordinal| LiveReleaseReceipt::devnet(&config, ordinal))
        .collect::<Vec<_>>();
    match State::new(config, receipts) {
        Ok(state) => state,
        Err(reason) => fallback_state(reason),
    }
}

pub fn public_record() -> Value {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

fn derive_mismatch_findings(
    config: &Config,
    receipts: &[LiveReleaseReceipt],
) -> Vec<MismatchFinding> {
    let mut findings = Vec::new();
    for receipt in receipts {
        if receipt.expected_release_amount_piconero != receipt.observed_release_amount_piconero {
            findings.push(mismatch_finding(
                receipt,
                MismatchKind::Amount,
                "expected-release-amount",
                "observed-release-amount",
            ));
        }
        if receipt.expected_destination_root != receipt.observed_destination_root {
            findings.push(mismatch_finding(
                receipt,
                MismatchKind::Destination,
                &receipt.expected_destination_root,
                &receipt.observed_destination_root,
            ));
        }
        if receipt.custody_signer_root.is_empty()
            || receipt.reserve_proof_root.is_empty()
            || receipt.monero_release_tx_observation_root.is_empty()
            || receipt.wallet_finality_root.is_empty()
            || receipt.release_authorization_root.is_empty()
        {
            findings.push(mismatch_finding(
                receipt,
                MismatchKind::Root,
                "present",
                "missing",
            ));
        }
        if !receipt.challenge_window_clear || receipt.challenge_window_root.is_empty() {
            findings.push(mismatch_finding(
                receipt,
                MismatchKind::ChallengeWindow,
                "clear",
                "blocked",
            ));
        }
        if receipt.finality_confirmations < config.required_finality_confirmations
            || !receipt.wallet_finality_verified
        {
            findings.push(mismatch_finding(
                receipt,
                MismatchKind::Finality,
                "required-finality",
                "shortfall",
            ));
        }
        if !receipt.release_authorized || receipt.release_authorization_root.is_empty() {
            findings.push(mismatch_finding(
                receipt,
                MismatchKind::Authorization,
                "authorized",
                "not-authorized",
            ));
        }
    }
    findings
}

fn mismatch_finding(
    receipt: &LiveReleaseReceipt,
    kind: MismatchKind,
    expected_root: &str,
    observed_root: &str,
) -> MismatchFinding {
    let finding_id = record_root(
        "mismatch-finding-id",
        &json!({
            "receipt_id": receipt.receipt_id,
            "kind": kind.as_str(),
            "expected_root": expected_root,
            "observed_root": observed_root,
        }),
    );
    MismatchFinding {
        finding_id,
        receipt_id: receipt.receipt_id.clone(),
        release_id: receipt.release_id.clone(),
        kind,
        expected_root: expected_root.to_string(),
        observed_root: observed_root.to_string(),
        expected_amount_piconero: receipt.expected_release_amount_piconero,
        observed_amount_piconero: receipt.observed_release_amount_piconero,
        fail_closed_required: true,
        user_release_blocked: true,
        production_blocked: true,
    }
}

fn validate_config(config: &Config) -> Result<()> {
    ensure(
        config.chain_id == CHAIN_ID,
        "bridge custody live release receipt verifier chain mismatch",
    )?;
    ensure(
        config.protocol_version == PROTOCOL_VERSION,
        "bridge custody live release receipt verifier protocol mismatch",
    )?;
    ensure(
        config.min_custody_signer_roots > 0,
        "bridge custody live release receipt verifier requires custody signer roots",
    )?;
    ensure(
        config.min_release_authorization_roots > 0,
        "bridge custody live release receipt verifier requires release authorization roots",
    )?;
    Ok(())
}

fn validate_receipts(receipts: &[LiveReleaseReceipt]) -> Result<()> {
    ensure(
        !receipts.is_empty(),
        "bridge custody live release receipt verifier requires receipts",
    )?;
    for receipt in receipts {
        ensure(
            !receipt.receipt_id.is_empty(),
            "bridge custody live release receipt verifier receipt id is required",
        )?;
        ensure(
            !receipt.release_id.is_empty(),
            "bridge custody live release receipt verifier release id is required",
        )?;
    }
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
    let mut receipt = LiveReleaseReceipt::devnet(&config, 1);
    receipt.receipt_verified = false;
    receipt.fail_closed_required = true;
    receipt.release_authorized = false;
    receipt.receipt_root = record_root("fallback-live-release-receipt", &json!({"reason": reason}));
    let receipts = vec![receipt];
    let findings = derive_mismatch_findings(&config, &receipts);
    let counters = Counters::new(&receipts, &findings);
    let verdict = FailClosedVerdict::new(&config, &counters, &findings);
    let roots = Roots::new(&config, &receipts, &findings, &verdict);
    let state_commitment_root =
        state_commitment_root(&config, &roots, &receipts, &findings, &verdict);
    State {
        config,
        roots,
        counters,
        live_release_receipts: receipts,
        mismatch_findings: findings,
        verdict,
        state_commitment_root,
    }
}

fn typed_receipt_root<F>(domain: &str, receipts: &[LiveReleaseReceipt], select: F) -> String
where
    F: Fn(&LiveReleaseReceipt) -> String,
{
    let leaves = receipts
        .iter()
        .map(|receipt| {
            json!({
                "receipt_id": receipt.receipt_id,
                "release_id": receipt.release_id,
                "root": select(receipt),
            })
        })
        .collect::<Vec<_>>();
    merkle_root(domain, &leaves)
}

fn populated_count<F>(receipts: &[LiveReleaseReceipt], select: F) -> u64
where
    F: Fn(&LiveReleaseReceipt) -> String,
{
    receipts
        .iter()
        .filter(|receipt| !select(receipt).is_empty())
        .count() as u64
}

fn live_release_receipt_vector_root(receipts: &[LiveReleaseReceipt]) -> String {
    let leaves = receipts
        .iter()
        .map(LiveReleaseReceipt::public_record)
        .collect::<Vec<_>>();
    merkle_root(
        "monero-l2-pq-bridge-force-exit-bridge-custody-live-release-receipts",
        &leaves,
    )
}

fn mismatch_finding_vector_root(findings: &[MismatchFinding]) -> String {
    let leaves = findings
        .iter()
        .map(MismatchFinding::public_record)
        .collect::<Vec<_>>();
    merkle_root(
        "monero-l2-pq-bridge-force-exit-bridge-custody-live-release-mismatch-findings",
        &leaves,
    )
}

fn receipt_part_root(kind: &str, release_id: &str, ordinal: u64) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-BRIDGE-CUSTODY-LIVE-RELEASE-RECEIPT-PART",
        &[
            HashPart::Str(kind),
            HashPart::Str(release_id),
            HashPart::U64(ordinal),
        ],
        32,
    )
}

fn state_commitment_root(
    config: &Config,
    roots: &Roots,
    receipts: &[LiveReleaseReceipt],
    findings: &[MismatchFinding],
    verdict: &FailClosedVerdict,
) -> String {
    let receipt_root = live_release_receipt_vector_root(receipts);
    let finding_root = mismatch_finding_vector_root(findings);
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-BRIDGE-CUSTODY-LIVE-RELEASE-RECEIPT-VERIFIER-STATE",
        &[
            HashPart::Json(&config.public_record()),
            HashPart::Json(&roots.public_record()),
            HashPart::Str(&receipt_root),
            HashPart::Str(&finding_root),
            HashPart::Json(&verdict.public_record()),
            HashPart::Str(bool_str(verdict.fail_closed)),
        ],
        32,
    )
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-BRIDGE-CUSTODY-LIVE-RELEASE-RECEIPT-VERIFIER-RECORD",
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
