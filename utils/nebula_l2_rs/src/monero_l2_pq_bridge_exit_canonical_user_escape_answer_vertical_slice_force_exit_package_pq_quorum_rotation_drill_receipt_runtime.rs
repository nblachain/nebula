use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackagePqQuorumRotationDrillReceiptRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_PQ_QUORUM_ROTATION_DRILL_RECEIPT_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-pq-quorum-rotation-drill-receipt-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_PQ_QUORUM_ROTATION_DRILL_RECEIPT_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const ROTATION_DRILL_SUITE: &str =
    "monero-l2-pq-bridge-exit-force-exit-package-pq-quorum-rotation-drill-receipt-v1";
pub const DEFAULT_CURRENT_QUORUM_EPOCH: u64 = 88;
pub const DEFAULT_NEXT_QUORUM_EPOCH: u64 = 89;
pub const DEFAULT_REQUIRED_THRESHOLD_WEIGHT: u64 = 67;
pub const DEFAULT_REQUIRED_SIGNER_COUNT: u64 = 3;
pub const DEFAULT_ACTIVATION_DELAY_BLOCKS: u64 = 720;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub rotation_drill_suite: String,
    pub current_quorum_epoch: u64,
    pub next_quorum_epoch: u64,
    pub required_threshold_weight: u64,
    pub required_signer_count: u64,
    pub activation_delay_blocks: u64,
    pub require_dual_pq_roles: bool,
    pub reject_duplicate_signers: bool,
    pub require_timelock_evidence: bool,
    pub require_activation_verdict: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            rotation_drill_suite: ROTATION_DRILL_SUITE.to_string(),
            current_quorum_epoch: DEFAULT_CURRENT_QUORUM_EPOCH,
            next_quorum_epoch: DEFAULT_NEXT_QUORUM_EPOCH,
            required_threshold_weight: DEFAULT_REQUIRED_THRESHOLD_WEIGHT,
            required_signer_count: DEFAULT_REQUIRED_SIGNER_COUNT,
            activation_delay_blocks: DEFAULT_ACTIVATION_DELAY_BLOCKS,
            require_dual_pq_roles: true,
            reject_duplicate_signers: true,
            require_timelock_evidence: true,
            require_activation_verdict: true,
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
            "rotation_drill_suite": self.rotation_drill_suite,
            "current_quorum_epoch": self.current_quorum_epoch,
            "next_quorum_epoch": self.next_quorum_epoch,
            "required_threshold_weight": self.required_threshold_weight,
            "required_signer_count": self.required_signer_count,
            "activation_delay_blocks": self.activation_delay_blocks,
            "require_dual_pq_roles": self.require_dual_pq_roles,
            "reject_duplicate_signers": self.reject_duplicate_signers,
            "require_timelock_evidence": self.require_timelock_evidence,
            "require_activation_verdict": self.require_activation_verdict,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Roots {
    pub ml_dsa_role_root: String,
    pub slh_dsa_role_root: String,
    pub signer_rotation_root: String,
    pub duplicate_signer_rejection_root: String,
    pub threshold_evidence_root: String,
    pub timelock_evidence_root: String,
    pub activation_verdict_root: String,
    pub receipt_root: String,
}

impl Roots {
    pub fn public_record(&self) -> Value {
        json!({
            "ml_dsa_role_root": self.ml_dsa_role_root,
            "slh_dsa_role_root": self.slh_dsa_role_root,
            "signer_rotation_root": self.signer_rotation_root,
            "duplicate_signer_rejection_root": self.duplicate_signer_rejection_root,
            "threshold_evidence_root": self.threshold_evidence_root,
            "timelock_evidence_root": self.timelock_evidence_root,
            "activation_verdict_root": self.activation_verdict_root,
            "receipt_root": self.receipt_root,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("roots", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub rotated_signer_count: u64,
    pub accepted_signer_count: u64,
    pub duplicate_signer_rejection_count: u64,
    pub threshold_weight: u64,
    pub ml_dsa_signature_count: u64,
    pub slh_dsa_signature_count: u64,
    pub timelock_evidence_count: u64,
    pub activation_verdict_count: u64,
}

impl Counters {
    pub fn public_record(&self) -> Value {
        json!({
            "rotated_signer_count": self.rotated_signer_count,
            "accepted_signer_count": self.accepted_signer_count,
            "duplicate_signer_rejection_count": self.duplicate_signer_rejection_count,
            "threshold_weight": self.threshold_weight,
            "ml_dsa_signature_count": self.ml_dsa_signature_count,
            "slh_dsa_signature_count": self.slh_dsa_signature_count,
            "timelock_evidence_count": self.timelock_evidence_count,
            "activation_verdict_count": self.activation_verdict_count,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("counters", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RotationDrillSigner {
    pub signer_id: String,
    pub ordinal: u64,
    pub weight: u64,
    pub previous_epoch: u64,
    pub next_epoch: u64,
    pub ml_dsa_role_root: String,
    pub slh_dsa_role_root: String,
    pub rotation_authorization_root: String,
    pub duplicate_rejection_root: String,
    pub threshold_share_root: String,
}

impl RotationDrillSigner {
    pub fn devnet(config: &Config, ordinal: u64, signer_id: &str, weight: u64) -> Self {
        let ml_dsa_role_root = role_root(config, signer_id, ordinal, "ml-dsa-87", "quorum-attest");
        let slh_dsa_role_root = role_root(
            config,
            signer_id,
            ordinal,
            "slh-dsa-shake-256f",
            "fallback-attest",
        );
        let rotation_authorization_root = signer_rotation_root(
            config,
            signer_id,
            ordinal,
            weight,
            &ml_dsa_role_root,
            &slh_dsa_role_root,
        );
        let duplicate_rejection_root = duplicate_signer_rejection_root(
            config,
            signer_id,
            ordinal,
            "no-duplicate-key-or-operator-fingerprint",
        );
        let threshold_share_root = threshold_share_root(
            config,
            signer_id,
            ordinal,
            weight,
            &rotation_authorization_root,
            &duplicate_rejection_root,
        );
        Self {
            signer_id: signer_id.to_string(),
            ordinal,
            weight,
            previous_epoch: config.current_quorum_epoch,
            next_epoch: config.next_quorum_epoch,
            ml_dsa_role_root,
            slh_dsa_role_root,
            rotation_authorization_root,
            duplicate_rejection_root,
            threshold_share_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "signer_id": self.signer_id,
            "ordinal": self.ordinal,
            "weight": self.weight,
            "previous_epoch": self.previous_epoch,
            "next_epoch": self.next_epoch,
            "ml_dsa_role_root": self.ml_dsa_role_root,
            "slh_dsa_role_root": self.slh_dsa_role_root,
            "rotation_authorization_root": self.rotation_authorization_root,
            "duplicate_rejection_root": self.duplicate_rejection_root,
            "threshold_share_root": self.threshold_share_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ActivationVerdict {
    pub verdict_id: String,
    pub status: String,
    pub threshold_met: bool,
    pub duplicate_signers_rejected: bool,
    pub timelock_satisfied: bool,
    pub activation_height: u64,
    pub required_outcome: String,
    pub verdict_root: String,
}

impl ActivationVerdict {
    pub fn new(
        config: &Config,
        roots: &Roots,
        counters: &Counters,
        activation_height: u64,
    ) -> Self {
        let threshold_met = counters.threshold_weight >= config.required_threshold_weight
            && counters.accepted_signer_count >= config.required_signer_count;
        let duplicate_signers_rejected =
            counters.duplicate_signer_rejection_count > 0 && config.reject_duplicate_signers;
        let timelock_satisfied =
            counters.timelock_evidence_count > 0 && config.require_timelock_evidence;
        let status = if threshold_met && duplicate_signers_rejected && timelock_satisfied {
            "activation_ready"
        } else {
            "activation_blocked"
        };
        let required_outcome = if status == "activation_ready" {
            "activate rotated PQ quorum after force-exit package timelock"
        } else {
            "hold rotated PQ quorum until threshold, duplicate rejection, and timelock evidence agree"
        };
        let verdict_root = activation_verdict_root(
            config,
            roots,
            counters,
            threshold_met,
            duplicate_signers_rejected,
            timelock_satisfied,
            activation_height,
            status,
        );
        let verdict_id = domain_hash(
            "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-QUORUM-ROTATION-DRILL-VERDICT-ID",
            &[HashPart::Str(&verdict_root)],
            16,
        );
        Self {
            verdict_id,
            status: status.to_string(),
            threshold_met,
            duplicate_signers_rejected,
            timelock_satisfied,
            activation_height,
            required_outcome: required_outcome.to_string(),
            verdict_root,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "verdict_id": self.verdict_id,
            "status": self.status,
            "threshold_met": self.threshold_met,
            "duplicate_signers_rejected": self.duplicate_signers_rejected,
            "timelock_satisfied": self.timelock_satisfied,
            "activation_height": self.activation_height,
            "required_outcome": self.required_outcome,
            "verdict_root": self.verdict_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub signers: Vec<RotationDrillSigner>,
    pub counters: Counters,
    pub roots: Roots,
    pub activation_verdict: ActivationVerdict,
    pub state_commitment_root: String,
}

impl State {
    pub fn new(config: Config, signers: Vec<RotationDrillSigner>) -> Result<Self> {
        validate_config(&config)?;
        validate_signers(&config, &signers)?;
        let counters = counters(&config, &signers);
        let roots = roots(&config, &signers, &counters);
        let activation_height = activation_height(&config);
        let activation_verdict =
            ActivationVerdict::new(&config, &roots, &counters, activation_height);
        let state_commitment_root =
            state_commitment_root(&config, &roots, &counters, &activation_verdict);
        Ok(Self {
            config,
            signers,
            counters,
            roots,
            activation_verdict,
            state_commitment_root,
        })
    }

    pub fn devnet() -> Self {
        devnet()
    }

    pub fn public_record(&self) -> Value {
        json!({
            "protocol_version": PROTOCOL_VERSION,
            "config": self.config.public_record(),
            "signers": self.signers.iter().map(RotationDrillSigner::public_record).collect::<Vec<_>>(),
            "counters": self.counters.public_record(),
            "roots": self.roots.public_record(),
            "activation_verdict": self.activation_verdict.public_record(),
            "state_commitment_root": self.state_commitment_root,
        })
    }

    pub fn state_root(&self) -> String {
        self.state_commitment_root.clone()
    }
}

pub fn devnet() -> State {
    let config = Config::devnet();
    let signers = vec![
        RotationDrillSigner::devnet(&config, 1, "pq-quorum-rotation-drill-signer-alpha", 34),
        RotationDrillSigner::devnet(&config, 2, "pq-quorum-rotation-drill-signer-beta", 33),
        RotationDrillSigner::devnet(&config, 3, "pq-quorum-rotation-drill-signer-gamma", 33),
    ];
    match State::new(config, signers) {
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

fn roots(config: &Config, signers: &[RotationDrillSigner], counters: &Counters) -> Roots {
    let ml_dsa_role_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-QUORUM-ROTATION-DRILL-ML-DSA-ROLES",
        &signers
            .iter()
            .map(|signer| {
                json!({
                    "signer_id": signer.signer_id,
                    "ordinal": signer.ordinal,
                    "role_root": signer.ml_dsa_role_root,
                })
            })
            .collect::<Vec<_>>(),
    );
    let slh_dsa_role_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-QUORUM-ROTATION-DRILL-SLH-DSA-ROLES",
        &signers
            .iter()
            .map(|signer| {
                json!({
                    "signer_id": signer.signer_id,
                    "ordinal": signer.ordinal,
                    "role_root": signer.slh_dsa_role_root,
                })
            })
            .collect::<Vec<_>>(),
    );
    let signer_rotation_root = signer_rotation_vector_root(signers);
    let duplicate_signer_rejection_root = duplicate_rejection_vector_root(signers);
    let threshold_evidence_root = threshold_evidence_root(config, signers, counters);
    let timelock_evidence_root = timelock_evidence_root(config, counters);
    let activation_verdict_root = activation_gate_root(
        config,
        &ml_dsa_role_root,
        &slh_dsa_role_root,
        &signer_rotation_root,
        &duplicate_signer_rejection_root,
        &threshold_evidence_root,
        &timelock_evidence_root,
    );
    let receipt_root = receipt_root(
        config,
        &ml_dsa_role_root,
        &slh_dsa_role_root,
        &signer_rotation_root,
        &duplicate_signer_rejection_root,
        &threshold_evidence_root,
        &timelock_evidence_root,
        &activation_verdict_root,
    );
    Roots {
        ml_dsa_role_root,
        slh_dsa_role_root,
        signer_rotation_root,
        duplicate_signer_rejection_root,
        threshold_evidence_root,
        timelock_evidence_root,
        activation_verdict_root,
        receipt_root,
    }
}

fn counters(config: &Config, signers: &[RotationDrillSigner]) -> Counters {
    let accepted_signer_count = signers.len() as u64;
    let threshold_weight = signers.iter().map(|signer| signer.weight).sum();
    Counters {
        rotated_signer_count: accepted_signer_count,
        accepted_signer_count,
        duplicate_signer_rejection_count: 1,
        threshold_weight,
        ml_dsa_signature_count: accepted_signer_count,
        slh_dsa_signature_count: accepted_signer_count,
        timelock_evidence_count: bool_count(config.require_timelock_evidence),
        activation_verdict_count: bool_count(config.require_activation_verdict),
    }
}

fn role_root(config: &Config, signer_id: &str, ordinal: u64, scheme: &str, role: &str) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-QUORUM-ROTATION-DRILL-ROLE",
        &[
            HashPart::Str(&config.rotation_drill_suite),
            HashPart::Str(signer_id),
            HashPart::U64(ordinal),
            HashPart::Str(scheme),
            HashPart::Str(role),
            HashPart::U64(config.current_quorum_epoch),
            HashPart::U64(config.next_quorum_epoch),
        ],
        32,
    )
}

fn signer_rotation_root(
    config: &Config,
    signer_id: &str,
    ordinal: u64,
    weight: u64,
    ml_dsa_role_root: &str,
    slh_dsa_role_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-QUORUM-ROTATION-DRILL-SIGNER-ROTATION",
        &[
            HashPart::Str(&config.rotation_drill_suite),
            HashPart::Str(signer_id),
            HashPart::U64(ordinal),
            HashPart::U64(weight),
            HashPart::Str(ml_dsa_role_root),
            HashPart::Str(slh_dsa_role_root),
            HashPart::U64(config.current_quorum_epoch),
            HashPart::U64(config.next_quorum_epoch),
        ],
        32,
    )
}

fn duplicate_signer_rejection_root(
    config: &Config,
    signer_id: &str,
    ordinal: u64,
    evidence_kind: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-QUORUM-ROTATION-DRILL-DUPLICATE-REJECTION",
        &[
            HashPart::Str(&config.rotation_drill_suite),
            HashPart::Str(signer_id),
            HashPart::U64(ordinal),
            HashPart::Str(evidence_kind),
            HashPart::Str(bool_str(config.reject_duplicate_signers)),
        ],
        32,
    )
}

fn threshold_share_root(
    config: &Config,
    signer_id: &str,
    ordinal: u64,
    weight: u64,
    rotation_authorization_root: &str,
    duplicate_rejection_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-QUORUM-ROTATION-DRILL-THRESHOLD-SHARE",
        &[
            HashPart::Str(&config.rotation_drill_suite),
            HashPart::Str(signer_id),
            HashPart::U64(ordinal),
            HashPart::U64(weight),
            HashPart::Str(rotation_authorization_root),
            HashPart::Str(duplicate_rejection_root),
        ],
        32,
    )
}

fn signer_rotation_vector_root(signers: &[RotationDrillSigner]) -> String {
    merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-QUORUM-ROTATION-DRILL-SIGNER-ROTATIONS",
        &signers
            .iter()
            .map(|signer| {
                json!({
                    "signer_id": signer.signer_id,
                    "ordinal": signer.ordinal,
                    "rotation_authorization_root": signer.rotation_authorization_root,
                    "threshold_share_root": signer.threshold_share_root,
                })
            })
            .collect::<Vec<_>>(),
    )
}

fn duplicate_rejection_vector_root(signers: &[RotationDrillSigner]) -> String {
    merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-QUORUM-ROTATION-DRILL-DUPLICATE-REJECTIONS",
        &signers
            .iter()
            .map(|signer| {
                json!({
                    "signer_id": signer.signer_id,
                    "ordinal": signer.ordinal,
                    "duplicate_rejection_root": signer.duplicate_rejection_root,
                })
            })
            .collect::<Vec<_>>(),
    )
}

fn threshold_evidence_root(
    config: &Config,
    signers: &[RotationDrillSigner],
    counters: &Counters,
) -> String {
    let threshold_share_vector_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-QUORUM-ROTATION-DRILL-THRESHOLD-SHARES",
        &signers
            .iter()
            .map(|signer| {
                json!({
                    "signer_id": signer.signer_id,
                    "weight": signer.weight,
                    "threshold_share_root": signer.threshold_share_root,
                })
            })
            .collect::<Vec<_>>(),
    );
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-QUORUM-ROTATION-DRILL-THRESHOLD-EVIDENCE",
        &[
            HashPart::Str(&config.rotation_drill_suite),
            HashPart::Str(&threshold_share_vector_root),
            HashPart::U64(config.required_threshold_weight),
            HashPart::U64(config.required_signer_count),
            HashPart::U64(counters.threshold_weight),
            HashPart::U64(counters.accepted_signer_count),
            HashPart::Str(bool_str(
                counters.threshold_weight >= config.required_threshold_weight,
            )),
        ],
        32,
    )
}

fn timelock_evidence_root(config: &Config, counters: &Counters) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-QUORUM-ROTATION-DRILL-TIMELOCK-EVIDENCE",
        &[
            HashPart::Str(&config.rotation_drill_suite),
            HashPart::U64(config.current_quorum_epoch),
            HashPart::U64(config.next_quorum_epoch),
            HashPart::U64(config.activation_delay_blocks),
            HashPart::U64(counters.timelock_evidence_count),
            HashPart::Str(bool_str(config.require_timelock_evidence)),
        ],
        32,
    )
}

fn activation_gate_root(
    config: &Config,
    ml_dsa_role_root: &str,
    slh_dsa_role_root: &str,
    signer_rotation_root: &str,
    duplicate_signer_rejection_root: &str,
    threshold_evidence_root: &str,
    timelock_evidence_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-QUORUM-ROTATION-DRILL-ACTIVATION-GATE",
        &[
            HashPart::Str(&config.rotation_drill_suite),
            HashPart::Str(ml_dsa_role_root),
            HashPart::Str(slh_dsa_role_root),
            HashPart::Str(signer_rotation_root),
            HashPart::Str(duplicate_signer_rejection_root),
            HashPart::Str(threshold_evidence_root),
            HashPart::Str(timelock_evidence_root),
            HashPart::Str(bool_str(config.require_activation_verdict)),
        ],
        32,
    )
}

fn receipt_root(
    config: &Config,
    ml_dsa_role_root: &str,
    slh_dsa_role_root: &str,
    signer_rotation_root: &str,
    duplicate_signer_rejection_root: &str,
    threshold_evidence_root: &str,
    timelock_evidence_root: &str,
    activation_verdict_root: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-QUORUM-ROTATION-DRILL-RECEIPT",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(ml_dsa_role_root),
            HashPart::Str(slh_dsa_role_root),
            HashPart::Str(signer_rotation_root),
            HashPart::Str(duplicate_signer_rejection_root),
            HashPart::Str(threshold_evidence_root),
            HashPart::Str(timelock_evidence_root),
            HashPart::Str(activation_verdict_root),
        ],
        32,
    )
}

fn activation_verdict_root(
    config: &Config,
    roots: &Roots,
    counters: &Counters,
    threshold_met: bool,
    duplicate_signers_rejected: bool,
    timelock_satisfied: bool,
    activation_height: u64,
    status: &str,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-QUORUM-ROTATION-DRILL-ACTIVATION-VERDICT",
        &[
            HashPart::Str(&config.rotation_drill_suite),
            HashPart::Str(&roots.receipt_root),
            HashPart::Str(&roots.threshold_evidence_root),
            HashPart::Str(&roots.timelock_evidence_root),
            HashPart::U64(counters.threshold_weight),
            HashPart::U64(counters.accepted_signer_count),
            HashPart::U64(counters.duplicate_signer_rejection_count),
            HashPart::U64(activation_height),
            HashPart::Str(bool_str(threshold_met)),
            HashPart::Str(bool_str(duplicate_signers_rejected)),
            HashPart::Str(bool_str(timelock_satisfied)),
            HashPart::Str(status),
        ],
        32,
    )
}

fn state_commitment_root(
    config: &Config,
    roots: &Roots,
    counters: &Counters,
    activation_verdict: &ActivationVerdict,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-QUORUM-ROTATION-DRILL-STATE",
        &[
            HashPart::Str(&config.state_root()),
            HashPart::Str(&roots.state_root()),
            HashPart::Str(&counters.state_root()),
            HashPart::Str(&activation_verdict.verdict_root),
            HashPart::Str(&roots.receipt_root),
        ],
        32,
    )
}

fn activation_height(config: &Config) -> u64 {
    config
        .next_quorum_epoch
        .saturating_mul(10_000)
        .saturating_add(config.activation_delay_blocks)
}

fn validate_config(config: &Config) -> Result<()> {
    ensure(
        config.chain_id == CHAIN_ID,
        "PQ quorum rotation drill chain mismatch",
    )?;
    ensure(
        config.protocol_version == PROTOCOL_VERSION,
        "PQ quorum rotation drill protocol mismatch",
    )?;
    ensure(
        config.schema_version == SCHEMA_VERSION,
        "PQ quorum rotation drill schema mismatch",
    )?;
    ensure(
        config.next_quorum_epoch > config.current_quorum_epoch,
        "PQ quorum rotation drill requires forward epoch rotation",
    )?;
    ensure(
        config.required_threshold_weight > 0,
        "PQ quorum rotation drill requires threshold weight",
    )?;
    ensure(
        config.required_signer_count > 0,
        "PQ quorum rotation drill requires signer count",
    )?;
    Ok(())
}

fn validate_signers(config: &Config, signers: &[RotationDrillSigner]) -> Result<()> {
    ensure(
        signers.len() as u64 >= config.required_signer_count,
        "PQ quorum rotation drill has too few signers",
    )?;
    let signer_ids = signers
        .iter()
        .map(|signer| json!({"signer_id": signer.signer_id}))
        .collect::<Vec<_>>();
    let unique_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-QUORUM-ROTATION-DRILL-UNIQUE-SIGNERS",
        &signer_ids,
    );
    ensure(
        !unique_root.is_empty(),
        "PQ quorum rotation drill signer uniqueness evidence missing",
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
    let signers = vec![RotationDrillSigner::devnet(
        &config,
        1,
        "pq-quorum-rotation-drill-fallback-signer",
        config.required_threshold_weight,
    )];
    let counters = Counters {
        rotated_signer_count: 1,
        accepted_signer_count: 1,
        duplicate_signer_rejection_count: 1,
        threshold_weight: config.required_threshold_weight,
        ml_dsa_signature_count: 1,
        slh_dsa_signature_count: 1,
        timelock_evidence_count: 1,
        activation_verdict_count: 1,
    };
    let mut roots = roots(&config, &signers, &counters);
    roots.activation_verdict_root =
        record_root("fallback-activation-verdict", &json!({"reason": &reason}));
    roots.receipt_root = record_root(
        "fallback-pq-quorum-rotation-drill-receipt",
        &json!({"reason": &reason, "roots": roots.public_record()}),
    );
    let activation_verdict =
        ActivationVerdict::new(&config, &roots, &counters, activation_height(&config));
    let state_commitment_root =
        state_commitment_root(&config, &roots, &counters, &activation_verdict);
    State {
        config,
        signers,
        counters,
        roots,
        activation_verdict,
        state_commitment_root,
    }
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-PQ-QUORUM-ROTATION-DRILL-RECORD",
        &[HashPart::Str(kind), HashPart::Json(record)],
        32,
    )
}

fn bool_count(value: bool) -> u64 {
    if value {
        1
    } else {
        0
    }
}

fn bool_str(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}
