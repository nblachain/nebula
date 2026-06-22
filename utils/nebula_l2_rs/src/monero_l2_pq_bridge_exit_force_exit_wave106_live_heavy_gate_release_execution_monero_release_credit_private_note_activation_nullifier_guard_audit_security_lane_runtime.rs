use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type PublicRecord = Value;
pub type Runtime = State;
pub type MoneroL2PqBridgeExitForceExitWave106LiveHeavyGateReleaseExecutionMoneroReleaseCreditPrivateNoteActivationNullifierGuardAuditSecurityLaneRuntimeResult<
    T,
> = Result<T>;

pub const MONERO_L2_PQ_BRIDGE_EXIT_FORCE_EXIT_WAVE106_LIVE_HEAVY_GATE_RELEASE_EXECUTION_MONERO_RELEASE_CREDIT_PRIVATE_NOTE_ACTIVATION_NULLIFIER_GUARD_AUDIT_SECURITY_LANE_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-force-exit-wave106-live-heavy-gate-release-execution-monero-release-credit-private-note-activation-nullifier-guard-audit-security-lane-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_FORCE_EXIT_WAVE106_LIVE_HEAVY_GATE_RELEASE_EXECUTION_MONERO_RELEASE_CREDIT_PRIVATE_NOTE_ACTIVATION_NULLIFIER_GUARD_AUDIT_SECURITY_LANE_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const SECURITY_LANE_SUITE: &str =
    "monero-release-credit-private-note-activation-nullifier-guard-audit-security-lane-v1";
pub const PRIVACY_BOUNDARY: &str =
    "public-roots-and-redacted-counts-only-no-wallet-history-or-note-plaintext";
pub const WAVE105_ACCOUNTING_SUITE: &str =
    "wave105-confirmed-credit-accounting-roots-and-bridge-liability-closure-v1";
pub const DEFAULT_DEVNET_HEIGHT: u64 = 806_106;
pub const DEFAULT_EVIDENCE_WINDOW_BLOCKS: u64 = 720;
pub const DEFAULT_MIN_ACCOUNTING_ROOTS: usize = 4;
pub const DEFAULT_MIN_NOTE_COMMITMENT_ROOTS: usize = 3;
pub const DEFAULT_MIN_NULLIFIER_RESERVATION_ROOTS: usize = 3;
pub const DEFAULT_MIN_AMOUNT_BUCKETS: usize = 8;
pub const DEFAULT_MIN_WALLET_HISTORY_SETS: usize = 4;
pub const DEFAULT_MIN_LIVE_HEAVY_GATE_TRANSCRIPTS: usize = 2;
pub const DEFAULT_MIN_SIGNOFFS: usize = 2;
pub const DEFAULT_MIN_PQ_SECURITY_BITS: u16 = 192;
pub const DEFAULT_MAX_WALLET_HISTORY_LINKAGE_BPS: u16 = 0;
pub const DEFAULT_MAX_AMOUNT_BUCKET_DISCLOSURE_BPS: u16 = 25;
pub const DEFAULT_MAX_FEE_DRIFT_ATOMIC_UNITS: u64 = 0;
pub const DEFAULT_MAX_REBATE_DRIFT_ATOMIC_UNITS: u64 = 0;
pub const DEFAULT_MAX_BRIDGE_LIABILITY_DRIFT_ATOMIC_UNITS: u64 = 0;
pub const DEFAULT_MAX_OPEN_BREAKERS: usize = 0;
pub const DEFAULT_MAX_STALE_EVIDENCE_BLOCKS: u64 = 36;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneVerdict {
    FailClosed,
    AuditOnly,
    ReleaseCandidate,
    Activated,
}

impl LaneVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FailClosed => "fail_closed",
            Self::AuditOnly => "audit_only",
            Self::ReleaseCandidate => "release_candidate",
            Self::Activated => "activated",
        }
    }

    pub fn allows_activation(self) -> bool {
        matches!(self, Self::Activated)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GateStatus {
    Missing,
    Submitted,
    Bound,
    Rejected,
}

impl GateStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Missing => "missing",
            Self::Submitted => "submitted",
            Self::Bound => "bound",
            Self::Rejected => "rejected",
        }
    }

    pub fn is_bound(self) -> bool {
        matches!(self, Self::Bound)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SignoffRole {
    Security,
    Accounting,
    Privacy,
    Operations,
}

impl SignoffRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Security => "security",
            Self::Accounting => "accounting",
            Self::Privacy => "privacy",
            Self::Operations => "operations",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BreakerState {
    Closed,
    Armed,
    Open,
}

impl BreakerState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::Armed => "armed",
            Self::Open => "open",
        }
    }

    pub fn blocks_release(self) -> bool {
        matches!(self, Self::Open)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub security_lane_suite: String,
    pub privacy_boundary: String,
    pub wave105_accounting_suite: String,
    pub activation_height: u64,
    pub evidence_window_blocks: u64,
    pub min_accounting_roots: usize,
    pub min_note_commitment_roots: usize,
    pub min_nullifier_reservation_roots: usize,
    pub min_amount_buckets: usize,
    pub min_wallet_history_sets: usize,
    pub min_live_heavy_gate_transcripts: usize,
    pub min_signoffs: usize,
    pub min_pq_security_bits: u16,
    pub max_wallet_history_linkage_bps: u16,
    pub max_amount_bucket_disclosure_bps: u16,
    pub max_fee_drift_atomic_units: u64,
    pub max_rebate_drift_atomic_units: u64,
    pub max_bridge_liability_drift_atomic_units: u64,
    pub max_open_breakers: usize,
    pub max_stale_evidence_blocks: u64,
    pub release_credit_allowed: bool,
    pub note_activation_allowed: bool,
    pub heavy_gates_ran: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self::devnet()
    }
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            security_lane_suite: SECURITY_LANE_SUITE.to_string(),
            privacy_boundary: PRIVACY_BOUNDARY.to_string(),
            wave105_accounting_suite: WAVE105_ACCOUNTING_SUITE.to_string(),
            activation_height: DEFAULT_DEVNET_HEIGHT,
            evidence_window_blocks: DEFAULT_EVIDENCE_WINDOW_BLOCKS,
            min_accounting_roots: DEFAULT_MIN_ACCOUNTING_ROOTS,
            min_note_commitment_roots: DEFAULT_MIN_NOTE_COMMITMENT_ROOTS,
            min_nullifier_reservation_roots: DEFAULT_MIN_NULLIFIER_RESERVATION_ROOTS,
            min_amount_buckets: DEFAULT_MIN_AMOUNT_BUCKETS,
            min_wallet_history_sets: DEFAULT_MIN_WALLET_HISTORY_SETS,
            min_live_heavy_gate_transcripts: DEFAULT_MIN_LIVE_HEAVY_GATE_TRANSCRIPTS,
            min_signoffs: DEFAULT_MIN_SIGNOFFS,
            min_pq_security_bits: DEFAULT_MIN_PQ_SECURITY_BITS,
            max_wallet_history_linkage_bps: DEFAULT_MAX_WALLET_HISTORY_LINKAGE_BPS,
            max_amount_bucket_disclosure_bps: DEFAULT_MAX_AMOUNT_BUCKET_DISCLOSURE_BPS,
            max_fee_drift_atomic_units: DEFAULT_MAX_FEE_DRIFT_ATOMIC_UNITS,
            max_rebate_drift_atomic_units: DEFAULT_MAX_REBATE_DRIFT_ATOMIC_UNITS,
            max_bridge_liability_drift_atomic_units:
                DEFAULT_MAX_BRIDGE_LIABILITY_DRIFT_ATOMIC_UNITS,
            max_open_breakers: DEFAULT_MAX_OPEN_BREAKERS,
            max_stale_evidence_blocks: DEFAULT_MAX_STALE_EVIDENCE_BLOCKS,
            release_credit_allowed: false,
            note_activation_allowed: false,
            heavy_gates_ran: false,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "chain_id": self.chain_id,
            "protocol_version": self.protocol_version,
            "schema_version": self.schema_version,
            "hash_suite": self.hash_suite,
            "security_lane_suite": self.security_lane_suite,
            "privacy_boundary": self.privacy_boundary,
            "wave105_accounting_suite": self.wave105_accounting_suite,
            "activation_height": self.activation_height,
            "evidence_window_blocks": self.evidence_window_blocks,
            "thresholds": {
                "min_accounting_roots": self.min_accounting_roots,
                "min_note_commitment_roots": self.min_note_commitment_roots,
                "min_nullifier_reservation_roots": self.min_nullifier_reservation_roots,
                "min_amount_buckets": self.min_amount_buckets,
                "min_wallet_history_sets": self.min_wallet_history_sets,
                "min_live_heavy_gate_transcripts": self.min_live_heavy_gate_transcripts,
                "min_signoffs": self.min_signoffs,
                "min_pq_security_bits": self.min_pq_security_bits,
            },
            "limits": {
                "max_wallet_history_linkage_bps": self.max_wallet_history_linkage_bps,
                "max_amount_bucket_disclosure_bps": self.max_amount_bucket_disclosure_bps,
                "max_fee_drift_atomic_units": self.max_fee_drift_atomic_units,
                "max_rebate_drift_atomic_units": self.max_rebate_drift_atomic_units,
                "max_bridge_liability_drift_atomic_units": self.max_bridge_liability_drift_atomic_units,
                "max_open_breakers": self.max_open_breakers,
                "max_stale_evidence_blocks": self.max_stale_evidence_blocks,
            },
            "release_credit_allowed": false,
            "note_activation_allowed": false,
            "heavy_gates_ran": false,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Wave105AccountingRoots {
    pub wave105_transcript_id: String,
    pub confirmed_credit_root: String,
    pub bridge_custody_root: String,
    pub fee_rebate_root: String,
    pub liability_closure_root: String,
    pub runtime_replay_root: String,
    pub wallet_watchtower_root: String,
    pub pq_reserve_privacy_root: String,
    pub accounting_height: u64,
    pub bound_to_release_execution: bool,
}

impl Wave105AccountingRoots {
    pub fn devnet(config: &Config) -> Self {
        let wave105_transcript_id = "devnet-wave105-confirmed-credit-accounting-0001".to_string();
        let confirmed_credit_root = sample_root(
            "WAVE105-CONFIRMED-CREDIT",
            &wave105_transcript_id,
            &[
                "monero-confirmed-credit-accounted",
                "release-credit-not-double-issued",
                "custody-debit-linked",
                "accounting-frontier-final",
            ],
        );
        let bridge_custody_root = sample_root(
            "WAVE105-BRIDGE-CUSTODY",
            &wave105_transcript_id,
            &[
                "bridge-custody-asset-root",
                "locked-output-root",
                "operator-custody-quorum-root",
                "custody-replay-fence-root",
            ],
        );
        let fee_rebate_root = sample_root(
            "WAVE105-FEE-REBATE",
            &wave105_transcript_id,
            &[
                "fee-collected-root",
                "rebate-payable-root",
                "dust-rounding-zero-drift",
                "settlement-queue-closed",
            ],
        );
        let liability_closure_root = sample_root(
            "WAVE105-LIABILITY-CLOSURE",
            &wave105_transcript_id,
            &[
                "liability-open-zero",
                "release-execution-payable-equals-custody",
                "forced-exit-claims-netted",
                "audit-ledger-balanced",
            ],
        );
        let runtime_replay_root = sample_root(
            "WAVE105-RUNTIME-REPLAY",
            &wave105_transcript_id,
            &[
                "runtime-replay-deterministic",
                "state-transition-root-matched",
                "operator-log-root-matched",
                "no-stale-release-path",
            ],
        );
        let wallet_watchtower_root = sample_root(
            "WAVE105-WALLET-WATCHTOWER",
            &wave105_transcript_id,
            &[
                "watchtower-redacted-receipt-root",
                "wallet-history-hidden",
                "view-tag-only-scan-hints",
                "no-address-linkage",
            ],
        );
        let pq_reserve_privacy_root = sample_root(
            "WAVE105-PQ-RESERVE-PRIVACY",
            &wave105_transcript_id,
            &[
                "pq-reserve-authorized",
                "post-quantum-signature-root",
                "reserve-privacy-envelope-root",
                "key-rotation-notice-root",
            ],
        );

        Self {
            wave105_transcript_id,
            confirmed_credit_root,
            bridge_custody_root,
            fee_rebate_root,
            liability_closure_root,
            runtime_replay_root,
            wallet_watchtower_root,
            pq_reserve_privacy_root,
            accounting_height: config.activation_height.saturating_sub(12),
            bound_to_release_execution: true,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "wave105_transcript_id": self.wave105_transcript_id,
            "confirmed_credit_root": self.confirmed_credit_root,
            "bridge_custody_root": self.bridge_custody_root,
            "fee_rebate_root": self.fee_rebate_root,
            "liability_closure_root": self.liability_closure_root,
            "runtime_replay_root": self.runtime_replay_root,
            "wallet_watchtower_root": self.wallet_watchtower_root,
            "pq_reserve_privacy_root": self.pq_reserve_privacy_root,
            "accounting_height": self.accounting_height,
            "bound_to_release_execution": self.bound_to_release_execution,
        })
    }

    pub fn evidence_count(&self) -> usize {
        non_empty_count(&[
            &self.confirmed_credit_root,
            &self.bridge_custody_root,
            &self.fee_rebate_root,
            &self.liability_closure_root,
            &self.runtime_replay_root,
            &self.wallet_watchtower_root,
            &self.pq_reserve_privacy_root,
        ])
    }

    pub fn state_root(&self) -> String {
        record_root("wave105_accounting_roots", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NoteCommitmentEvidence {
    pub commitment_set_id: String,
    pub note_commitment_root: String,
    pub note_activation_request_root: String,
    pub encrypted_note_payload_root: String,
    pub membership_witness_root: String,
    pub output_reconciliation_root: String,
    pub commitment_count: u64,
    pub activation_request_count: u64,
    pub plaintext_note_fields_exposed: bool,
    pub bound_to_wave105_accounting: bool,
}

impl NoteCommitmentEvidence {
    pub fn devnet(config: &Config, accounting: &Wave105AccountingRoots) -> Self {
        let commitment_set_id = "devnet-release-credit-private-note-commitments-0001".to_string();
        let note_commitment_root = sample_root(
            "WAVE106-NOTE-COMMITMENT",
            &commitment_set_id,
            &[
                "release-credit-note-commitment",
                "amount-committed",
                "owner-spend-committed",
                "activation-domain-separated",
            ],
        );
        let note_activation_request_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-EXIT-WAVE106-NOTE-ACTIVATION-REQUEST",
            &[
                HashPart::Str(&config.chain_id),
                HashPart::Str(&config.protocol_version),
                HashPart::Str(&accounting.state_root()),
                HashPart::Str(&note_commitment_root),
            ],
            32,
        );
        let encrypted_note_payload_root = sample_root(
            "WAVE106-ENCRYPTED-NOTE-PAYLOAD",
            &commitment_set_id,
            &[
                "recipient-payload-ciphertext",
                "view-tag-scan-hint",
                "no-wallet-address-plaintext",
                "no-amount-plaintext",
            ],
        );
        let membership_witness_root = sample_root(
            "WAVE106-MEMBERSHIP-WITNESS",
            &commitment_set_id,
            &[
                "frontier-before-activation",
                "frontier-after-activation",
                "inclusion-proof-redacted",
                "release-credit-domain",
            ],
        );
        let output_reconciliation_root = sample_root(
            "WAVE106-OUTPUT-RECONCILIATION",
            &commitment_set_id,
            &[
                "activated-output-root",
                "runtime-output-reconciliation-root",
                "release-credit-receipt-root",
                "forced-exit-compatibility-root",
            ],
        );

        Self {
            commitment_set_id,
            note_commitment_root,
            note_activation_request_root,
            encrypted_note_payload_root,
            membership_witness_root,
            output_reconciliation_root,
            commitment_count: 16_384,
            activation_request_count: 16_384,
            plaintext_note_fields_exposed: false,
            bound_to_wave105_accounting: true,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "commitment_set_id": self.commitment_set_id,
            "note_commitment_root": self.note_commitment_root,
            "note_activation_request_root": self.note_activation_request_root,
            "encrypted_note_payload_root": self.encrypted_note_payload_root,
            "membership_witness_root": self.membership_witness_root,
            "output_reconciliation_root": self.output_reconciliation_root,
            "commitment_count": self.commitment_count,
            "activation_request_count": self.activation_request_count,
            "plaintext_note_fields_exposed": self.plaintext_note_fields_exposed,
            "bound_to_wave105_accounting": self.bound_to_wave105_accounting,
        })
    }

    pub fn evidence_count(&self) -> usize {
        non_empty_count(&[
            &self.note_commitment_root,
            &self.note_activation_request_root,
            &self.encrypted_note_payload_root,
            &self.membership_witness_root,
            &self.output_reconciliation_root,
        ])
    }

    pub fn state_root(&self) -> String {
        record_root("note_commitment_evidence", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NullifierReservationEvidence {
    pub reservation_set_id: String,
    pub nullifier_reservation_root: String,
    pub key_image_guard_root: String,
    pub replay_fence_root: String,
    pub duplicate_spend_scan_root: String,
    pub reservation_count: u64,
    pub collision_count: u64,
    pub reservations_finalized: bool,
    pub bound_to_note_commitments: bool,
}

impl NullifierReservationEvidence {
    pub fn devnet(notes: &NoteCommitmentEvidence) -> Self {
        let reservation_set_id = "devnet-release-credit-nullifier-reservations-0001".to_string();
        let nullifier_reservation_root = sample_root(
            "WAVE106-NULLIFIER-RESERVATION",
            &reservation_set_id,
            &[
                "reserved-before-note-activation",
                "release-credit-spend-domain",
                "force-exit-nullifier-fence",
                "canonical-key-image-root",
            ],
        );
        let key_image_guard_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-EXIT-WAVE106-KEY-IMAGE-GUARD",
            &[
                HashPart::Str(&reservation_set_id),
                HashPart::Str(&notes.note_commitment_root),
                HashPart::Str(&nullifier_reservation_root),
            ],
            32,
        );
        let replay_fence_root = sample_root(
            "WAVE106-REPLAY-FENCE",
            &reservation_set_id,
            &[
                "operator-replay-fence",
                "bridge-exit-domain-fence",
                "release-credit-nonce-root",
                "chain-id-bound",
            ],
        );
        let duplicate_spend_scan_root = sample_root(
            "WAVE106-DUPLICATE-SPEND-SCAN",
            &reservation_set_id,
            &[
                "no-duplicate-nullifier",
                "no-reserved-key-image-reuse",
                "no-late-conflict",
                "audit-log-closed",
            ],
        );

        Self {
            reservation_set_id,
            nullifier_reservation_root,
            key_image_guard_root,
            replay_fence_root,
            duplicate_spend_scan_root,
            reservation_count: notes.activation_request_count,
            collision_count: 0,
            reservations_finalized: true,
            bound_to_note_commitments: true,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "reservation_set_id": self.reservation_set_id,
            "nullifier_reservation_root": self.nullifier_reservation_root,
            "key_image_guard_root": self.key_image_guard_root,
            "replay_fence_root": self.replay_fence_root,
            "duplicate_spend_scan_root": self.duplicate_spend_scan_root,
            "reservation_count": self.reservation_count,
            "collision_count": self.collision_count,
            "reservations_finalized": self.reservations_finalized,
            "bound_to_note_commitments": self.bound_to_note_commitments,
        })
    }

    pub fn evidence_count(&self) -> usize {
        non_empty_count(&[
            &self.nullifier_reservation_root,
            &self.key_image_guard_root,
            &self.replay_fence_root,
            &self.duplicate_spend_scan_root,
        ])
    }

    pub fn state_root(&self) -> String {
        record_root("nullifier_reservation_evidence", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AmountBucketPrivacyEvidence {
    pub bucket_set_id: String,
    pub bucket_commitment_root: String,
    pub range_proof_root: String,
    pub decoy_distribution_root: String,
    pub disclosure_budget_root: String,
    pub bucket_count: usize,
    pub max_bucket_disclosure_bps: u16,
    pub plaintext_amounts_exposed: bool,
}

impl AmountBucketPrivacyEvidence {
    pub fn devnet(config: &Config) -> Self {
        let bucket_set_id = "devnet-release-credit-amount-bucket-privacy-0001".to_string();
        let bucket_commitment_root = sample_root(
            "WAVE106-AMOUNT-BUCKET",
            &bucket_set_id,
            &[
                "bucket-0-committed",
                "bucket-1-committed",
                "bucket-2-committed",
                "bucket-3-committed",
                "bucket-4-committed",
                "bucket-5-committed",
                "bucket-6-committed",
                "bucket-7-committed",
            ],
        );
        let range_proof_root = sample_root(
            "WAVE106-AMOUNT-RANGE-PROOF",
            &bucket_set_id,
            &[
                "range-proof-root",
                "conservation-proof-root",
                "fee-exclusion-proof-root",
                "rebate-exclusion-proof-root",
            ],
        );
        let decoy_distribution_root = sample_root(
            "WAVE106-AMOUNT-DECOY-DISTRIBUTION",
            &bucket_set_id,
            &[
                "decoy-histogram-root",
                "minimum-anonymity-set-root",
                "amount-bucket-shuffle-root",
                "no-singleton-bucket",
            ],
        );
        let disclosure_budget_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-EXIT-WAVE106-AMOUNT-BUCKET-DISCLOSURE-BUDGET",
            &[
                HashPart::Str(&bucket_set_id),
                HashPart::U64(config.max_amount_bucket_disclosure_bps as u64),
                HashPart::Str(&bucket_commitment_root),
                HashPart::Str(&range_proof_root),
                HashPart::Str(&decoy_distribution_root),
            ],
            32,
        );

        Self {
            bucket_set_id,
            bucket_commitment_root,
            range_proof_root,
            decoy_distribution_root,
            disclosure_budget_root,
            bucket_count: config.min_amount_buckets,
            max_bucket_disclosure_bps: config.max_amount_bucket_disclosure_bps,
            plaintext_amounts_exposed: false,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "bucket_set_id": self.bucket_set_id,
            "bucket_commitment_root": self.bucket_commitment_root,
            "range_proof_root": self.range_proof_root,
            "decoy_distribution_root": self.decoy_distribution_root,
            "disclosure_budget_root": self.disclosure_budget_root,
            "bucket_count": self.bucket_count,
            "max_bucket_disclosure_bps": self.max_bucket_disclosure_bps,
            "plaintext_amounts_exposed": self.plaintext_amounts_exposed,
        })
    }

    pub fn evidence_count(&self) -> usize {
        non_empty_count(&[
            &self.bucket_commitment_root,
            &self.range_proof_root,
            &self.decoy_distribution_root,
            &self.disclosure_budget_root,
        ])
    }

    pub fn state_root(&self) -> String {
        record_root("amount_bucket_privacy_evidence", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WalletHistoryPrivacyEvidence {
    pub wallet_privacy_set_id: String,
    pub wallet_history_commitment_root: String,
    pub scan_hint_root: String,
    pub unlinkability_proof_root: String,
    pub watchtower_receipt_root: String,
    pub wallet_history_set_count: usize,
    pub max_linkage_bps: u16,
    pub wallet_addresses_exposed: bool,
    pub wallet_history_plaintext_exposed: bool,
}

impl WalletHistoryPrivacyEvidence {
    pub fn devnet(config: &Config) -> Self {
        let wallet_privacy_set_id = "devnet-release-credit-wallet-history-privacy-0001".to_string();
        let wallet_history_commitment_root = sample_root(
            "WAVE106-WALLET-HISTORY-COMMITMENT",
            &wallet_privacy_set_id,
            &[
                "history-window-0-committed",
                "history-window-1-committed",
                "history-window-2-committed",
                "history-window-3-committed",
            ],
        );
        let scan_hint_root = sample_root(
            "WAVE106-WALLET-SCAN-HINT",
            &wallet_privacy_set_id,
            &[
                "view-tag-root",
                "subaddress-hint-root",
                "receipt-hint-root",
                "forced-exit-guard-hint-root",
            ],
        );
        let unlinkability_proof_root = sample_root(
            "WAVE106-WALLET-UNLINKABILITY",
            &wallet_privacy_set_id,
            &[
                "no-deposit-to-note-link",
                "no-note-to-wallet-link",
                "no-wallet-history-disclosure",
                "redacted-watchtower-audit",
            ],
        );
        let watchtower_receipt_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-EXIT-WAVE106-WALLET-WATCHTOWER-RECEIPT",
            &[
                HashPart::Str(&wallet_privacy_set_id),
                HashPart::Str(&wallet_history_commitment_root),
                HashPart::Str(&scan_hint_root),
                HashPart::Str(&unlinkability_proof_root),
            ],
            32,
        );

        Self {
            wallet_privacy_set_id,
            wallet_history_commitment_root,
            scan_hint_root,
            unlinkability_proof_root,
            watchtower_receipt_root,
            wallet_history_set_count: config.min_wallet_history_sets,
            max_linkage_bps: config.max_wallet_history_linkage_bps,
            wallet_addresses_exposed: false,
            wallet_history_plaintext_exposed: false,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "wallet_privacy_set_id": self.wallet_privacy_set_id,
            "wallet_history_commitment_root": self.wallet_history_commitment_root,
            "scan_hint_root": self.scan_hint_root,
            "unlinkability_proof_root": self.unlinkability_proof_root,
            "watchtower_receipt_root": self.watchtower_receipt_root,
            "wallet_history_set_count": self.wallet_history_set_count,
            "max_linkage_bps": self.max_linkage_bps,
            "wallet_addresses_exposed": self.wallet_addresses_exposed,
            "wallet_history_plaintext_exposed": self.wallet_history_plaintext_exposed,
        })
    }

    pub fn evidence_count(&self) -> usize {
        non_empty_count(&[
            &self.wallet_history_commitment_root,
            &self.scan_hint_root,
            &self.unlinkability_proof_root,
            &self.watchtower_receipt_root,
        ])
    }

    pub fn state_root(&self) -> String {
        record_root("wallet_history_privacy_evidence", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BridgeLiabilityClosureEvidence {
    pub closure_id: String,
    pub bridge_liability_root: String,
    pub custody_balance_root: String,
    pub release_credit_liability_root: String,
    pub forced_exit_claim_root: String,
    pub residual_liability_atomic_units: u64,
    pub bridge_liability_closed: bool,
}

impl BridgeLiabilityClosureEvidence {
    pub fn devnet(accounting: &Wave105AccountingRoots) -> Self {
        let closure_id = "devnet-release-credit-bridge-liability-closure-0001".to_string();
        let bridge_liability_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-EXIT-WAVE106-BRIDGE-LIABILITY",
            &[
                HashPart::Str(&closure_id),
                HashPart::Str(&accounting.liability_closure_root),
                HashPart::Str(&accounting.bridge_custody_root),
            ],
            32,
        );
        let custody_balance_root = sample_root(
            "WAVE106-CUSTODY-BALANCE",
            &closure_id,
            &[
                "custody-balance-equals-release-credit",
                "confirmed-credit-covered",
                "no-orphan-liability",
                "custody-receipt-frontier",
            ],
        );
        let release_credit_liability_root = sample_root(
            "WAVE106-RELEASE-CREDIT-LIABILITY",
            &closure_id,
            &[
                "release-credit-payable-zero-after-activation",
                "note-activation-liability-covered",
                "operator-shortfall-zero",
                "accounting-domain-bound",
            ],
        );
        let forced_exit_claim_root = sample_root(
            "WAVE106-FORCED-EXIT-CLAIM",
            &closure_id,
            &[
                "forced-exit-claim-netted",
                "claim-window-closed",
                "bridge-exit-claim-root",
                "no-late-claim-open",
            ],
        );

        Self {
            closure_id,
            bridge_liability_root,
            custody_balance_root,
            release_credit_liability_root,
            forced_exit_claim_root,
            residual_liability_atomic_units: 0,
            bridge_liability_closed: true,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "closure_id": self.closure_id,
            "bridge_liability_root": self.bridge_liability_root,
            "custody_balance_root": self.custody_balance_root,
            "release_credit_liability_root": self.release_credit_liability_root,
            "forced_exit_claim_root": self.forced_exit_claim_root,
            "residual_liability_atomic_units": self.residual_liability_atomic_units,
            "bridge_liability_closed": self.bridge_liability_closed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("bridge_liability_closure_evidence", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FeeRebateSettlementEvidence {
    pub settlement_id: String,
    pub fee_collection_root: String,
    pub rebate_settlement_root: String,
    pub operator_fee_root: String,
    pub dust_rounding_root: String,
    pub fee_drift_atomic_units: u64,
    pub rebate_drift_atomic_units: u64,
    pub settlement_closed: bool,
}

impl FeeRebateSettlementEvidence {
    pub fn devnet(accounting: &Wave105AccountingRoots) -> Self {
        let settlement_id = "devnet-release-credit-fee-rebate-settlement-0001".to_string();
        let fee_collection_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-EXIT-WAVE106-FEE-COLLECTION",
            &[
                HashPart::Str(&settlement_id),
                HashPart::Str(&accounting.fee_rebate_root),
                HashPart::Str(&accounting.confirmed_credit_root),
            ],
            32,
        );
        let rebate_settlement_root = sample_root(
            "WAVE106-REBATE-SETTLEMENT",
            &settlement_id,
            &[
                "rebate-payable-settled",
                "rebate-recipient-commitment-root",
                "rebate-queue-empty",
                "rebate-rounding-zero",
            ],
        );
        let operator_fee_root = sample_root(
            "WAVE106-OPERATOR-FEE",
            &settlement_id,
            &[
                "operator-fee-collected",
                "operator-fee-cap-respected",
                "relayer-fee-redacted",
                "fee-receipt-root",
            ],
        );
        let dust_rounding_root = sample_root(
            "WAVE106-DUST-ROUNDING",
            &settlement_id,
            &[
                "dust-residual-zero",
                "rounding-mode-canonical",
                "fee-rebate-net-zero",
                "settlement-audit-root",
            ],
        );

        Self {
            settlement_id,
            fee_collection_root,
            rebate_settlement_root,
            operator_fee_root,
            dust_rounding_root,
            fee_drift_atomic_units: 0,
            rebate_drift_atomic_units: 0,
            settlement_closed: true,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "settlement_id": self.settlement_id,
            "fee_collection_root": self.fee_collection_root,
            "rebate_settlement_root": self.rebate_settlement_root,
            "operator_fee_root": self.operator_fee_root,
            "dust_rounding_root": self.dust_rounding_root,
            "fee_drift_atomic_units": self.fee_drift_atomic_units,
            "rebate_drift_atomic_units": self.rebate_drift_atomic_units,
            "settlement_closed": self.settlement_closed,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("fee_rebate_settlement_evidence", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PqAuthorizationEvidence {
    pub authorization_id: String,
    pub pq_signature_root: String,
    pub pq_key_epoch_root: String,
    pub signer_quorum_root: String,
    pub release_policy_root: String,
    pub pq_security_bits: u16,
    pub authorization_final: bool,
}

impl PqAuthorizationEvidence {
    pub fn devnet(config: &Config, accounting: &Wave105AccountingRoots) -> Self {
        let authorization_id = "devnet-release-credit-pq-authorization-0001".to_string();
        let pq_signature_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-EXIT-WAVE106-PQ-SIGNATURE",
            &[
                HashPart::Str(&authorization_id),
                HashPart::Str(&accounting.pq_reserve_privacy_root),
                HashPart::Str(&config.protocol_version),
            ],
            32,
        );
        let pq_key_epoch_root = sample_root(
            "WAVE106-PQ-KEY-EPOCH",
            &authorization_id,
            &[
                "pq-key-epoch-active",
                "retired-key-rejected",
                "rotation-root-bound",
                "authorization-domain-separated",
            ],
        );
        let signer_quorum_root = sample_root(
            "WAVE106-PQ-SIGNER-QUORUM",
            &authorization_id,
            &[
                "security-signer-present",
                "accounting-signer-present",
                "privacy-signer-present",
                "operations-signer-present",
            ],
        );
        let release_policy_root = sample_root(
            "WAVE106-PQ-RELEASE-POLICY",
            &authorization_id,
            &[
                "release-credit-policy-bound",
                "private-note-activation-policy-bound",
                "nullifier-reservation-policy-bound",
                "fail-closed-policy-bound",
            ],
        );

        Self {
            authorization_id,
            pq_signature_root,
            pq_key_epoch_root,
            signer_quorum_root,
            release_policy_root,
            pq_security_bits: config.min_pq_security_bits,
            authorization_final: true,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "authorization_id": self.authorization_id,
            "pq_signature_root": self.pq_signature_root,
            "pq_key_epoch_root": self.pq_key_epoch_root,
            "signer_quorum_root": self.signer_quorum_root,
            "release_policy_root": self.release_policy_root,
            "pq_security_bits": self.pq_security_bits,
            "authorization_final": self.authorization_final,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("pq_authorization_evidence", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CircuitBreakerEvidence {
    pub breaker_set_id: String,
    pub accounting_breaker_state: BreakerState,
    pub nullifier_breaker_state: BreakerState,
    pub privacy_breaker_state: BreakerState,
    pub bridge_liability_breaker_state: BreakerState,
    pub heavy_gate_breaker_state: BreakerState,
    pub breaker_digest_root: String,
    pub open_breaker_count: usize,
}

impl CircuitBreakerEvidence {
    pub fn devnet() -> Self {
        let breaker_set_id = "devnet-release-credit-circuit-breakers-0001".to_string();
        let accounting_breaker_state = BreakerState::Closed;
        let nullifier_breaker_state = BreakerState::Closed;
        let privacy_breaker_state = BreakerState::Closed;
        let bridge_liability_breaker_state = BreakerState::Closed;
        let heavy_gate_breaker_state = BreakerState::Closed;
        let breaker_digest_root = sample_root(
            "WAVE106-CIRCUIT-BREAKER",
            &breaker_set_id,
            &[
                accounting_breaker_state.as_str(),
                nullifier_breaker_state.as_str(),
                privacy_breaker_state.as_str(),
                bridge_liability_breaker_state.as_str(),
                heavy_gate_breaker_state.as_str(),
            ],
        );

        Self {
            breaker_set_id,
            accounting_breaker_state,
            nullifier_breaker_state,
            privacy_breaker_state,
            bridge_liability_breaker_state,
            heavy_gate_breaker_state,
            breaker_digest_root,
            open_breaker_count: 0,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "breaker_set_id": self.breaker_set_id,
            "accounting_breaker_state": self.accounting_breaker_state.as_str(),
            "nullifier_breaker_state": self.nullifier_breaker_state.as_str(),
            "privacy_breaker_state": self.privacy_breaker_state.as_str(),
            "bridge_liability_breaker_state": self.bridge_liability_breaker_state.as_str(),
            "heavy_gate_breaker_state": self.heavy_gate_breaker_state.as_str(),
            "breaker_digest_root": self.breaker_digest_root,
            "open_breaker_count": self.open_breaker_count,
        })
    }

    pub fn any_open(&self) -> bool {
        self.accounting_breaker_state.blocks_release()
            || self.nullifier_breaker_state.blocks_release()
            || self.privacy_breaker_state.blocks_release()
            || self.bridge_liability_breaker_state.blocks_release()
            || self.heavy_gate_breaker_state.blocks_release()
            || self.open_breaker_count > 0
    }

    pub fn state_root(&self) -> String {
        record_root("circuit_breaker_evidence", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LiveHeavyGateEvidence {
    pub heavy_gate_id: String,
    pub gate_status: GateStatus,
    pub transcript_root: String,
    pub replay_root: String,
    pub operator_observation_root: String,
    pub monitor_quorum_root: String,
    pub transcript_count: usize,
    pub ran_at_height: u64,
    pub bound_to_release_credit: bool,
}

impl LiveHeavyGateEvidence {
    pub fn devnet(config: &Config) -> Self {
        let heavy_gate_id = "devnet-release-credit-live-heavy-gate-0001".to_string();
        let transcript_root = sample_root(
            "WAVE106-LIVE-HEAVY-GATE-TRANSCRIPT",
            &heavy_gate_id,
            &[
                "live-heavy-gate-executed",
                "release-credit-activation-path",
                "private-note-nullifier-guard",
                "bridge-liability-closure-observed",
            ],
        );
        let replay_root = sample_root(
            "WAVE106-LIVE-HEAVY-GATE-REPLAY",
            &heavy_gate_id,
            &[
                "heavy-gate-replay-deterministic",
                "runtime-output-root-stable",
                "audit-lane-root-stable",
                "no-skip-path",
            ],
        );
        let operator_observation_root = sample_root(
            "WAVE106-LIVE-HEAVY-GATE-OPERATOR",
            &heavy_gate_id,
            &[
                "operator-observed-live-gate",
                "sequencer-batch-bound",
                "bridge-exit-batch-bound",
                "monitor-event-root",
            ],
        );
        let monitor_quorum_root = sample_root(
            "WAVE106-LIVE-HEAVY-GATE-MONITOR",
            &heavy_gate_id,
            &[
                "monitor-a-present",
                "monitor-b-present",
                "monitor-c-present",
                "quorum-timestamp-bound",
            ],
        );

        Self {
            heavy_gate_id,
            gate_status: GateStatus::Bound,
            transcript_root,
            replay_root,
            operator_observation_root,
            monitor_quorum_root,
            transcript_count: config.min_live_heavy_gate_transcripts,
            ran_at_height: config.activation_height,
            bound_to_release_credit: true,
        }
    }

    pub fn fail_closed(config: &Config) -> Self {
        Self {
            heavy_gate_id: "fail-closed-heavy-gate-evidence-missing".to_string(),
            gate_status: GateStatus::Missing,
            transcript_root: String::new(),
            replay_root: String::new(),
            operator_observation_root: String::new(),
            monitor_quorum_root: String::new(),
            transcript_count: 0,
            ran_at_height: config.activation_height,
            bound_to_release_credit: false,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "heavy_gate_id": self.heavy_gate_id,
            "gate_status": self.gate_status.as_str(),
            "transcript_root": self.transcript_root,
            "replay_root": self.replay_root,
            "operator_observation_root": self.operator_observation_root,
            "monitor_quorum_root": self.monitor_quorum_root,
            "transcript_count": self.transcript_count,
            "ran_at_height": self.ran_at_height,
            "bound_to_release_credit": self.bound_to_release_credit,
            "heavy_gates_ran": self.gate_status.is_bound(),
        })
    }

    pub fn evidence_count(&self) -> usize {
        non_empty_count(&[
            &self.transcript_root,
            &self.replay_root,
            &self.operator_observation_root,
            &self.monitor_quorum_root,
        ])
    }

    pub fn state_root(&self) -> String {
        record_root("live_heavy_gate_evidence", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DualSignoffEvidence {
    pub signoff_set_id: String,
    pub primary_role: SignoffRole,
    pub secondary_role: SignoffRole,
    pub primary_signoff_root: String,
    pub secondary_signoff_root: String,
    pub signoff_policy_root: String,
    pub signoff_count: usize,
    pub dual_signoff_final: bool,
}

impl DualSignoffEvidence {
    pub fn devnet(config: &Config, pq: &PqAuthorizationEvidence) -> Self {
        let signoff_set_id = "devnet-release-credit-dual-signoff-0001".to_string();
        let primary_role = SignoffRole::Security;
        let secondary_role = SignoffRole::Accounting;
        let primary_signoff_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-EXIT-WAVE106-PRIMARY-SIGNOFF",
            &[
                HashPart::Str(&signoff_set_id),
                HashPart::Str(primary_role.as_str()),
                HashPart::Str(&pq.pq_signature_root),
            ],
            32,
        );
        let secondary_signoff_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-EXIT-WAVE106-SECONDARY-SIGNOFF",
            &[
                HashPart::Str(&signoff_set_id),
                HashPart::Str(secondary_role.as_str()),
                HashPart::Str(&pq.release_policy_root),
            ],
            32,
        );
        let signoff_policy_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-EXIT-WAVE106-DUAL-SIGNOFF-POLICY",
            &[
                HashPart::Str(&signoff_set_id),
                HashPart::Str(&config.protocol_version),
                HashPart::U64(config.min_signoffs as u64),
                HashPart::Str(&primary_signoff_root),
                HashPart::Str(&secondary_signoff_root),
            ],
            32,
        );

        Self {
            signoff_set_id,
            primary_role,
            secondary_role,
            primary_signoff_root,
            secondary_signoff_root,
            signoff_policy_root,
            signoff_count: config.min_signoffs,
            dual_signoff_final: true,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "signoff_set_id": self.signoff_set_id,
            "primary_role": self.primary_role.as_str(),
            "secondary_role": self.secondary_role.as_str(),
            "primary_signoff_root": self.primary_signoff_root,
            "secondary_signoff_root": self.secondary_signoff_root,
            "signoff_policy_root": self.signoff_policy_root,
            "signoff_count": self.signoff_count,
            "dual_signoff_final": self.dual_signoff_final,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("dual_signoff_evidence", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ActivationDecision {
    pub decision_id: String,
    pub verdict: LaneVerdict,
    pub release_credit_allowed: bool,
    pub note_activation_allowed: bool,
    pub heavy_gates_ran: bool,
    pub refusal_reasons: Vec<String>,
    pub satisfied_controls: Vec<String>,
    pub audit_evidence_root: String,
    pub activation_state_root: String,
}

impl ActivationDecision {
    pub fn fail_closed(config: &Config, reason: &str) -> Self {
        let refusal_reasons = vec![reason.to_string()];
        let satisfied_controls = Vec::new();
        let audit_evidence_root = merkle_root(
            "MONERO-L2-PQ-BRIDGE-EXIT-WAVE106-FAIL-CLOSED-AUDIT-EVIDENCE",
            &[json!({
                "release_credit_allowed": false,
                "note_activation_allowed": false,
                "heavy_gates_ran": false,
                "reason": reason,
            })],
        );
        let activation_state_root = domain_hash(
            "MONERO-L2-PQ-BRIDGE-EXIT-WAVE106-FAIL-CLOSED-ACTIVATION-STATE",
            &[
                HashPart::Str(&config.protocol_version),
                HashPart::Str(&audit_evidence_root),
                HashPart::Str(reason),
            ],
            32,
        );

        Self {
            decision_id: "fail-closed-private-note-activation-refused".to_string(),
            verdict: LaneVerdict::FailClosed,
            release_credit_allowed: false,
            note_activation_allowed: false,
            heavy_gates_ran: false,
            refusal_reasons,
            satisfied_controls,
            audit_evidence_root,
            activation_state_root,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "decision_id": self.decision_id,
            "verdict": self.verdict.as_str(),
            "release_credit_allowed": self.release_credit_allowed,
            "note_activation_allowed": self.note_activation_allowed,
            "heavy_gates_ran": self.heavy_gates_ran,
            "refusal_reasons": self.refusal_reasons,
            "satisfied_controls": self.satisfied_controls,
            "audit_evidence_root": self.audit_evidence_root,
            "activation_state_root": self.activation_state_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("activation_decision", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub wave105_accounting_roots: Wave105AccountingRoots,
    pub note_commitment_evidence: NoteCommitmentEvidence,
    pub nullifier_reservation_evidence: NullifierReservationEvidence,
    pub amount_bucket_privacy_evidence: AmountBucketPrivacyEvidence,
    pub wallet_history_privacy_evidence: WalletHistoryPrivacyEvidence,
    pub bridge_liability_closure_evidence: BridgeLiabilityClosureEvidence,
    pub fee_rebate_settlement_evidence: FeeRebateSettlementEvidence,
    pub pq_authorization_evidence: PqAuthorizationEvidence,
    pub circuit_breaker_evidence: CircuitBreakerEvidence,
    pub live_heavy_gate_evidence: LiveHeavyGateEvidence,
    pub dual_signoff_evidence: DualSignoffEvidence,
    pub activation_decision: ActivationDecision,
}

impl Default for State {
    fn default() -> Self {
        Self::devnet()
    }
}

impl State {
    pub fn devnet() -> Self {
        let config = Config::devnet();
        let wave105_accounting_roots = Wave105AccountingRoots::devnet(&config);
        let note_commitment_evidence =
            NoteCommitmentEvidence::devnet(&config, &wave105_accounting_roots);
        let nullifier_reservation_evidence =
            NullifierReservationEvidence::devnet(&note_commitment_evidence);
        let amount_bucket_privacy_evidence = AmountBucketPrivacyEvidence::devnet(&config);
        let wallet_history_privacy_evidence = WalletHistoryPrivacyEvidence::devnet(&config);
        let bridge_liability_closure_evidence =
            BridgeLiabilityClosureEvidence::devnet(&wave105_accounting_roots);
        let fee_rebate_settlement_evidence =
            FeeRebateSettlementEvidence::devnet(&wave105_accounting_roots);
        let pq_authorization_evidence =
            PqAuthorizationEvidence::devnet(&config, &wave105_accounting_roots);
        let circuit_breaker_evidence = CircuitBreakerEvidence::devnet();
        let live_heavy_gate_evidence = LiveHeavyGateEvidence::devnet(&config);
        let dual_signoff_evidence =
            DualSignoffEvidence::devnet(&config, &pq_authorization_evidence);

        Self::from_parts(
            config,
            wave105_accounting_roots,
            note_commitment_evidence,
            nullifier_reservation_evidence,
            amount_bucket_privacy_evidence,
            wallet_history_privacy_evidence,
            bridge_liability_closure_evidence,
            fee_rebate_settlement_evidence,
            pq_authorization_evidence,
            circuit_breaker_evidence,
            live_heavy_gate_evidence,
            dual_signoff_evidence,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_parts(
        config: Config,
        wave105_accounting_roots: Wave105AccountingRoots,
        note_commitment_evidence: NoteCommitmentEvidence,
        nullifier_reservation_evidence: NullifierReservationEvidence,
        amount_bucket_privacy_evidence: AmountBucketPrivacyEvidence,
        wallet_history_privacy_evidence: WalletHistoryPrivacyEvidence,
        bridge_liability_closure_evidence: BridgeLiabilityClosureEvidence,
        fee_rebate_settlement_evidence: FeeRebateSettlementEvidence,
        pq_authorization_evidence: PqAuthorizationEvidence,
        circuit_breaker_evidence: CircuitBreakerEvidence,
        live_heavy_gate_evidence: LiveHeavyGateEvidence,
        dual_signoff_evidence: DualSignoffEvidence,
    ) -> Self {
        let activation_decision = evaluate_activation(
            &config,
            &wave105_accounting_roots,
            &note_commitment_evidence,
            &nullifier_reservation_evidence,
            &amount_bucket_privacy_evidence,
            &wallet_history_privacy_evidence,
            &bridge_liability_closure_evidence,
            &fee_rebate_settlement_evidence,
            &pq_authorization_evidence,
            &circuit_breaker_evidence,
            &live_heavy_gate_evidence,
            &dual_signoff_evidence,
        );

        Self {
            config,
            wave105_accounting_roots,
            note_commitment_evidence,
            nullifier_reservation_evidence,
            amount_bucket_privacy_evidence,
            wallet_history_privacy_evidence,
            bridge_liability_closure_evidence,
            fee_rebate_settlement_evidence,
            pq_authorization_evidence,
            circuit_breaker_evidence,
            live_heavy_gate_evidence,
            dual_signoff_evidence,
            activation_decision,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        json!({
            "config": self.config.public_record(),
            "wave105_accounting_roots": self.wave105_accounting_roots.public_record(),
            "note_commitment_evidence": self.note_commitment_evidence.public_record(),
            "nullifier_reservation_evidence": self.nullifier_reservation_evidence.public_record(),
            "amount_bucket_privacy_evidence": self.amount_bucket_privacy_evidence.public_record(),
            "wallet_history_privacy_evidence": self.wallet_history_privacy_evidence.public_record(),
            "bridge_liability_closure_evidence": self.bridge_liability_closure_evidence.public_record(),
            "fee_rebate_settlement_evidence": self.fee_rebate_settlement_evidence.public_record(),
            "pq_authorization_evidence": self.pq_authorization_evidence.public_record(),
            "circuit_breaker_evidence": self.circuit_breaker_evidence.public_record(),
            "live_heavy_gate_evidence": self.live_heavy_gate_evidence.public_record(),
            "dual_signoff_evidence": self.dual_signoff_evidence.public_record(),
            "activation_decision": self.activation_decision.public_record(),
            "release_credit_allowed": self.activation_decision.release_credit_allowed,
            "note_activation_allowed": self.activation_decision.note_activation_allowed,
            "heavy_gates_ran": self.activation_decision.heavy_gates_ran,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("state", &self.public_record())
    }

    pub fn validate(&self) -> Result<()> {
        if self.activation_decision.release_credit_allowed
            || self.activation_decision.note_activation_allowed
        {
            if !self.activation_decision.verdict.allows_activation() {
                return Err("activation flags cannot open without activated verdict".to_string());
            }
        }
        if self.activation_decision.release_credit_allowed
            != self.activation_decision.note_activation_allowed
        {
            return Err("release credit and note activation must move together".to_string());
        }
        if self.activation_decision.heavy_gates_ran
            && !self.live_heavy_gate_evidence.gate_status.is_bound()
        {
            return Err("heavy gate flag cannot open without bound live evidence".to_string());
        }
        if !self.activation_decision.refusal_reasons.is_empty()
            && self.activation_decision.verdict.allows_activation()
        {
            return Err("activated verdict cannot carry refusal reasons".to_string());
        }
        Ok(())
    }
}

pub fn devnet() -> Runtime {
    State::devnet()
}

pub fn public_record() -> PublicRecord {
    devnet().public_record()
}

pub fn state_root() -> String {
    devnet().state_root()
}

#[allow(clippy::too_many_arguments)]
fn evaluate_activation(
    config: &Config,
    accounting: &Wave105AccountingRoots,
    notes: &NoteCommitmentEvidence,
    nullifiers: &NullifierReservationEvidence,
    amount_privacy: &AmountBucketPrivacyEvidence,
    wallet_privacy: &WalletHistoryPrivacyEvidence,
    bridge_liability: &BridgeLiabilityClosureEvidence,
    fee_rebate: &FeeRebateSettlementEvidence,
    pq: &PqAuthorizationEvidence,
    breakers: &CircuitBreakerEvidence,
    heavy_gate: &LiveHeavyGateEvidence,
    signoffs: &DualSignoffEvidence,
) -> ActivationDecision {
    let mut refusal_reasons = Vec::new();
    let mut satisfied_controls = Vec::new();

    record_control(
        accounting.bound_to_release_execution
            && accounting.evidence_count() >= config.min_accounting_roots,
        "wave105-accounting-roots-bound",
        "wave105 accounting roots are missing or not bound",
        &mut satisfied_controls,
        &mut refusal_reasons,
    );
    record_control(
        notes.bound_to_wave105_accounting
            && !notes.plaintext_note_fields_exposed
            && notes.evidence_count() >= config.min_note_commitment_roots,
        "note-commitments-bound-and-redacted",
        "note commitment evidence is incomplete or exposes private fields",
        &mut satisfied_controls,
        &mut refusal_reasons,
    );
    record_control(
        nullifiers.reservations_finalized
            && nullifiers.bound_to_note_commitments
            && nullifiers.collision_count == 0
            && nullifiers.evidence_count() >= config.min_nullifier_reservation_roots,
        "nullifier-reservations-final-and-collision-free",
        "nullifier reservation guard is incomplete or has collisions",
        &mut satisfied_controls,
        &mut refusal_reasons,
    );
    record_control(
        amount_privacy.bucket_count >= config.min_amount_buckets
            && amount_privacy.max_bucket_disclosure_bps <= config.max_amount_bucket_disclosure_bps
            && !amount_privacy.plaintext_amounts_exposed,
        "amount-bucket-privacy-bound",
        "amount bucket privacy evidence is below threshold",
        &mut satisfied_controls,
        &mut refusal_reasons,
    );
    record_control(
        wallet_privacy.wallet_history_set_count >= config.min_wallet_history_sets
            && wallet_privacy.max_linkage_bps <= config.max_wallet_history_linkage_bps
            && !wallet_privacy.wallet_addresses_exposed
            && !wallet_privacy.wallet_history_plaintext_exposed,
        "wallet-history-privacy-bound",
        "wallet history privacy evidence leaks linkage or is below threshold",
        &mut satisfied_controls,
        &mut refusal_reasons,
    );
    record_control(
        bridge_liability.bridge_liability_closed
            && bridge_liability.residual_liability_atomic_units
                <= config.max_bridge_liability_drift_atomic_units,
        "bridge-liability-closure-bound",
        "bridge liability closure is not final",
        &mut satisfied_controls,
        &mut refusal_reasons,
    );
    record_control(
        fee_rebate.settlement_closed
            && fee_rebate.fee_drift_atomic_units <= config.max_fee_drift_atomic_units
            && fee_rebate.rebate_drift_atomic_units <= config.max_rebate_drift_atomic_units,
        "fee-rebate-settlement-bound",
        "fee or rebate settlement drift is above threshold",
        &mut satisfied_controls,
        &mut refusal_reasons,
    );
    record_control(
        pq.authorization_final && pq.pq_security_bits >= config.min_pq_security_bits,
        "pq-authorization-final",
        "post-quantum authorization is missing or below security threshold",
        &mut satisfied_controls,
        &mut refusal_reasons,
    );
    record_control(
        !breakers.any_open() && breakers.open_breaker_count <= config.max_open_breakers,
        "circuit-breakers-closed",
        "one or more circuit breakers are open",
        &mut satisfied_controls,
        &mut refusal_reasons,
    );
    record_control(
        heavy_gate.gate_status.is_bound()
            && heavy_gate.bound_to_release_credit
            && heavy_gate.transcript_count >= config.min_live_heavy_gate_transcripts
            && heavy_gate.evidence_count() >= config.min_live_heavy_gate_transcripts,
        "live-heavy-gates-ran-and-bound",
        "live heavy-gate evidence is missing or not bound",
        &mut satisfied_controls,
        &mut refusal_reasons,
    );
    record_control(
        signoffs.dual_signoff_final && signoffs.signoff_count >= config.min_signoffs,
        "dual-signoffs-final",
        "dual signoffs are missing or not final",
        &mut satisfied_controls,
        &mut refusal_reasons,
    );

    let heavy_gates_ran = heavy_gate.gate_status.is_bound()
        && heavy_gate.bound_to_release_credit
        && heavy_gate.transcript_count >= config.min_live_heavy_gate_transcripts;
    let all_controls_pass = refusal_reasons.is_empty();
    let release_credit_allowed =
        all_controls_pass && config.release_credit_allowed && config.note_activation_allowed;
    let note_activation_allowed =
        release_credit_allowed && heavy_gates_ran && config.heavy_gates_ran;
    let verdict = if note_activation_allowed {
        LaneVerdict::Activated
    } else if all_controls_pass && heavy_gates_ran {
        LaneVerdict::ReleaseCandidate
    } else if !satisfied_controls.is_empty() {
        LaneVerdict::AuditOnly
    } else {
        LaneVerdict::FailClosed
    };

    if !release_credit_allowed || !note_activation_allowed {
        refusal_reasons.push(
            "fail-closed config keeps release credit private-note activation off".to_string(),
        );
    }
    if !heavy_gates_ran {
        refusal_reasons.push("fail-closed heavy gate evidence did not run".to_string());
    }

    let audit_evidence_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-WAVE106-AUDIT-SECURITY-LANE-EVIDENCE",
        &[
            accounting.public_record(),
            notes.public_record(),
            nullifiers.public_record(),
            amount_privacy.public_record(),
            wallet_privacy.public_record(),
            bridge_liability.public_record(),
            fee_rebate.public_record(),
            pq.public_record(),
            breakers.public_record(),
            heavy_gate.public_record(),
            signoffs.public_record(),
        ],
    );
    let activation_state_root = domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-WAVE106-PRIVATE-NOTE-ACTIVATION-DECISION",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&config.protocol_version),
            HashPart::Str(&audit_evidence_root),
            HashPart::Str(verdict.as_str()),
            HashPart::Str(if release_credit_allowed {
                "true"
            } else {
                "false"
            }),
            HashPart::Str(if note_activation_allowed {
                "true"
            } else {
                "false"
            }),
            HashPart::Str(if heavy_gates_ran { "true" } else { "false" }),
        ],
        32,
    );

    ActivationDecision {
        decision_id:
            "wave106-release-credit-private-note-activation-nullifier-guard-audit-security-lane"
                .to_string(),
        verdict,
        release_credit_allowed,
        note_activation_allowed,
        heavy_gates_ran,
        refusal_reasons,
        satisfied_controls,
        audit_evidence_root,
        activation_state_root,
    }
}

fn record_control(
    passed: bool,
    control: &str,
    failure: &str,
    satisfied_controls: &mut Vec<String>,
    refusal_reasons: &mut Vec<String>,
) {
    if passed {
        satisfied_controls.push(control.to_string());
    } else {
        refusal_reasons.push(failure.to_string());
    }
}

fn non_empty_count(values: &[&String]) -> usize {
    values.iter().filter(|value| !value.is_empty()).count()
}

fn sample_root(domain: &str, id: &str, leaves: &[&str]) -> String {
    let values = leaves
        .iter()
        .enumerate()
        .map(|(index, leaf)| {
            json!({
                "id": id,
                "index": index,
                "leaf": leaf,
            })
        })
        .collect::<Vec<_>>();
    merkle_root(domain, &values)
}

fn record_root(domain: &str, value: &PublicRecord) -> String {
    domain_hash(
        &format!(
            "MONERO-L2-PQ-BRIDGE-EXIT-WAVE106-RELEASE-CREDIT-PRIVATE-NOTE-ACTIVATION-NULLIFIER-GUARD-AUDIT-SECURITY-LANE-{domain}"
        ),
        &[HashPart::Json(value)],
        32,
    )
}
