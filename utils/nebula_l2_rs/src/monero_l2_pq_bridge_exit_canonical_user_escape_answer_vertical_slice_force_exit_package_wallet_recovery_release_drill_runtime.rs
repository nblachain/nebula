use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    hash::{domain_hash, merkle_root, HashPart},
    CHAIN_ID,
};

pub type Result<T> = std::result::Result<T, String>;
pub type MoneroL2PqBridgeExitCanonicalUserEscapeAnswerVerticalSliceForceExitPackageWalletRecoveryReleaseDrillRuntimeResult<
    T,
> = Result<T>;
pub type Runtime = State;

pub const MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_WALLET_RECOVERY_RELEASE_DRILL_RUNTIME_PROTOCOL_VERSION: &str =
    "nebula-monero-l2-pq-bridge-exit-canonical-user-escape-answer-vertical-slice-force-exit-package-wallet-recovery-release-drill-runtime-v1";
pub const PROTOCOL_VERSION: &str =
    MONERO_L2_PQ_BRIDGE_EXIT_CANONICAL_USER_ESCAPE_ANSWER_VERTICAL_SLICE_FORCE_EXIT_PACKAGE_WALLET_RECOVERY_RELEASE_DRILL_RUNTIME_PROTOCOL_VERSION;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_SUITE: &str = "SHAKE256-domain-separated-canonical-json";
pub const WALLET_RECOVERY_RELEASE_DRILL_SUITE: &str =
    "monero-l2-pq-bridge-exit-canonical-user-escape-answer-wallet-recovery-release-drill-v1";
pub const DEFAULT_RECOVERY_HEIGHT: u64 = 1_935_184;
pub const DEFAULT_RELEASE_REQUEST_HEIGHT: u64 = 1_935_328;
pub const DEFAULT_MIN_SCAN_CONFIRMATIONS: u64 = 10;
pub const DEFAULT_MIN_ROTATION_SIGNERS: u64 = 5;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub chain_id: String,
    pub protocol_version: String,
    pub schema_version: u64,
    pub hash_suite: String,
    pub drill_suite: String,
    pub recovery_height: u64,
    pub release_request_height: u64,
    pub min_scan_confirmations: u64,
    pub min_rotation_signers: u64,
    pub require_wallet_transcript: bool,
    pub require_recovery_key_rotation: bool,
    pub require_release_request: bool,
    pub require_scan_proof: bool,
    pub require_private_note_continuity: bool,
    pub require_user_safe_release_verdict: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID.to_string(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            schema_version: SCHEMA_VERSION,
            hash_suite: HASH_SUITE.to_string(),
            drill_suite: WALLET_RECOVERY_RELEASE_DRILL_SUITE.to_string(),
            recovery_height: DEFAULT_RECOVERY_HEIGHT,
            release_request_height: DEFAULT_RELEASE_REQUEST_HEIGHT,
            min_scan_confirmations: DEFAULT_MIN_SCAN_CONFIRMATIONS,
            min_rotation_signers: DEFAULT_MIN_ROTATION_SIGNERS,
            require_wallet_transcript: true,
            require_recovery_key_rotation: true,
            require_release_request: true,
            require_scan_proof: true,
            require_private_note_continuity: true,
            require_user_safe_release_verdict: true,
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
            "drill_suite": self.drill_suite,
            "recovery_height": self.recovery_height,
            "release_request_height": self.release_request_height,
            "min_scan_confirmations": self.min_scan_confirmations,
            "min_rotation_signers": self.min_rotation_signers,
            "require_wallet_transcript": self.require_wallet_transcript,
            "require_recovery_key_rotation": self.require_recovery_key_rotation,
            "require_release_request": self.require_release_request,
            "require_scan_proof": self.require_scan_proof,
            "require_private_note_continuity": self.require_private_note_continuity,
            "require_user_safe_release_verdict": self.require_user_safe_release_verdict,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("config", &self.public_record())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillStepKind {
    WalletTranscript,
    RecoveryKeyRotation,
    ReleaseRequest,
    ScanProof,
    PrivateNoteContinuity,
    UserSafeReleaseVerdict,
}

impl DrillStepKind {
    pub fn ordered() -> &'static [Self] {
        &[
            Self::WalletTranscript,
            Self::RecoveryKeyRotation,
            Self::ReleaseRequest,
            Self::ScanProof,
            Self::PrivateNoteContinuity,
            Self::UserSafeReleaseVerdict,
        ]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::WalletTranscript => "wallet_transcript",
            Self::RecoveryKeyRotation => "recovery_key_rotation",
            Self::ReleaseRequest => "release_request",
            Self::ScanProof => "scan_proof",
            Self::PrivateNoteContinuity => "private_note_continuity",
            Self::UserSafeReleaseVerdict => "user_safe_release_verdict",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillStepStatus {
    Ready,
    Observed,
    HeldForEvidence,
    UserSafe,
}

impl DrillStepStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Observed => "observed",
            Self::HeldForEvidence => "held_for_evidence",
            Self::UserSafe => "user_safe",
        }
    }

    pub fn counts_as_safe(self) -> bool {
        matches!(self, Self::Ready | Self::Observed | Self::UserSafe)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DrillStep {
    pub step_id: String,
    pub ordinal: u64,
    pub kind: DrillStepKind,
    pub status: DrillStepStatus,
    pub evidence_root: String,
    pub wallet_instruction_root: String,
    pub release_guard_root: String,
    pub statement: String,
}

impl DrillStep {
    pub fn public_record(&self) -> Value {
        json!({
            "step_id": self.step_id,
            "ordinal": self.ordinal,
            "kind": self.kind.as_str(),
            "status": self.status.as_str(),
            "evidence_root": self.evidence_root,
            "wallet_instruction_root": self.wallet_instruction_root,
            "release_guard_root": self.release_guard_root,
            "statement": self.statement,
        })
    }

    pub fn state_root(&self) -> String {
        record_root("drill-step", &self.public_record())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Roots {
    pub wallet_transcript_root: String,
    pub recovery_key_rotation_root: String,
    pub release_request_root: String,
    pub scan_proof_root: String,
    pub private_note_continuity_root: String,
    pub user_safe_release_verdict_root: String,
    pub drill_step_root: String,
    pub state_commitment_root: String,
}

impl Roots {
    pub fn public_record(&self) -> Value {
        json!({
            "wallet_transcript_root": self.wallet_transcript_root,
            "recovery_key_rotation_root": self.recovery_key_rotation_root,
            "release_request_root": self.release_request_root,
            "scan_proof_root": self.scan_proof_root,
            "private_note_continuity_root": self.private_note_continuity_root,
            "user_safe_release_verdict_root": self.user_safe_release_verdict_root,
            "drill_step_root": self.drill_step_root,
            "state_commitment_root": self.state_commitment_root,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Counters {
    pub total_steps: u64,
    pub safe_steps: u64,
    pub held_steps: u64,
    pub scan_confirmations: u64,
    pub rotation_signer_count: u64,
    pub release_request_count: u64,
    pub private_note_count: u64,
}

impl Counters {
    pub fn from_steps(config: &Config, steps: &[DrillStep]) -> Self {
        Self {
            total_steps: steps.len() as u64,
            safe_steps: steps
                .iter()
                .filter(|step| step.status.counts_as_safe())
                .count() as u64,
            held_steps: steps
                .iter()
                .filter(|step| step.status == DrillStepStatus::HeldForEvidence)
                .count() as u64,
            scan_confirmations: config.min_scan_confirmations,
            rotation_signer_count: config.min_rotation_signers,
            release_request_count: steps
                .iter()
                .filter(|step| step.kind == DrillStepKind::ReleaseRequest)
                .count() as u64,
            private_note_count: steps
                .iter()
                .filter(|step| step.kind == DrillStepKind::PrivateNoteContinuity)
                .count() as u64,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "total_steps": self.total_steps,
            "safe_steps": self.safe_steps,
            "held_steps": self.held_steps,
            "scan_confirmations": self.scan_confirmations,
            "rotation_signer_count": self.rotation_signer_count,
            "release_request_count": self.release_request_count,
            "private_note_count": self.private_note_count,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    pub config: Config,
    pub roots: Roots,
    pub counters: Counters,
    pub drill_steps: Vec<DrillStep>,
    pub user_release_safe: bool,
    pub release_answer: String,
}

impl State {
    pub fn devnet() -> Self {
        let config = Config::devnet();
        let drill_steps = DrillStepKind::ordered()
            .iter()
            .enumerate()
            .map(|(index, kind)| drill_step(&config, *kind, index as u64 + 1))
            .collect::<Vec<_>>();
        let counters = Counters::from_steps(&config, &drill_steps);
        let user_release_safe = counters.held_steps == 0
            && counters.safe_steps == counters.total_steps
            && counters.scan_confirmations >= config.min_scan_confirmations
            && counters.rotation_signer_count >= config.min_rotation_signers;
        let release_answer = if user_release_safe {
            "yes_wallet_recovery_release_drill_is_user_safe".to_string()
        } else {
            "no_wallet_recovery_release_drill_remains_held".to_string()
        };
        let roots = deterministic_roots(&config, &counters, &drill_steps, user_release_safe);

        Self {
            config,
            roots,
            counters,
            drill_steps,
            user_release_safe,
            release_answer,
        }
    }

    pub fn public_record(&self) -> Value {
        json!({
            "config": self.config.public_record(),
            "roots": self.roots.public_record(),
            "counters": self.counters.public_record(),
            "drill_step_root": self.roots.drill_step_root,
            "user_release_safe": self.user_release_safe,
            "release_answer": self.release_answer,
        })
    }

    pub fn state_root(&self) -> String {
        self.roots.state_commitment_root.clone()
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

fn deterministic_roots(
    config: &Config,
    counters: &Counters,
    drill_steps: &[DrillStep],
    user_release_safe: bool,
) -> Roots {
    let wallet_transcript_root = step_kind_root(drill_steps, DrillStepKind::WalletTranscript);
    let recovery_key_rotation_root =
        step_kind_root(drill_steps, DrillStepKind::RecoveryKeyRotation);
    let release_request_root = step_kind_root(drill_steps, DrillStepKind::ReleaseRequest);
    let scan_proof_root = step_kind_root(drill_steps, DrillStepKind::ScanProof);
    let private_note_continuity_root =
        step_kind_root(drill_steps, DrillStepKind::PrivateNoteContinuity);
    let user_safe_release_verdict_root = user_safe_release_verdict_root(
        config,
        counters,
        &wallet_transcript_root,
        &recovery_key_rotation_root,
        &release_request_root,
        &scan_proof_root,
        &private_note_continuity_root,
        user_release_safe,
    );
    let drill_step_root = merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-RECOVERY-RELEASE-DRILL-STEP",
        &drill_steps
            .iter()
            .map(DrillStep::public_record)
            .collect::<Vec<_>>(),
    );
    let state_commitment_root = state_commitment_root(
        config,
        counters,
        &wallet_transcript_root,
        &recovery_key_rotation_root,
        &release_request_root,
        &scan_proof_root,
        &private_note_continuity_root,
        &user_safe_release_verdict_root,
        &drill_step_root,
        user_release_safe,
    );

    Roots {
        wallet_transcript_root,
        recovery_key_rotation_root,
        release_request_root,
        scan_proof_root,
        private_note_continuity_root,
        user_safe_release_verdict_root,
        drill_step_root,
        state_commitment_root,
    }
}

fn drill_step(config: &Config, kind: DrillStepKind, ordinal: u64) -> DrillStep {
    let status = match kind {
        DrillStepKind::WalletTranscript => DrillStepStatus::Observed,
        DrillStepKind::RecoveryKeyRotation => DrillStepStatus::Ready,
        DrillStepKind::ReleaseRequest => DrillStepStatus::Ready,
        DrillStepKind::ScanProof => DrillStepStatus::Observed,
        DrillStepKind::PrivateNoteContinuity => DrillStepStatus::Observed,
        DrillStepKind::UserSafeReleaseVerdict => DrillStepStatus::UserSafe,
    };
    let evidence_root = evidence_root(config, kind, ordinal);
    let wallet_instruction_root = record_root(
        "wallet-instruction",
        &json!({
            "kind": kind.as_str(),
            "ordinal": ordinal,
            "instruction": wallet_instruction(kind),
        }),
    );
    let release_guard_root = record_root(
        "release-guard",
        &json!({
            "kind": kind.as_str(),
            "ordinal": ordinal,
            "status": status.as_str(),
            "guard": release_guard(kind),
        }),
    );
    let step_id = domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-RECOVERY-RELEASE-DRILL-STEP-ID",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::U64(ordinal),
            HashPart::Str(kind.as_str()),
            HashPart::Str(&evidence_root),
        ],
        32,
    );

    DrillStep {
        step_id,
        ordinal,
        kind,
        status,
        evidence_root,
        wallet_instruction_root,
        release_guard_root,
        statement: drill_statement(kind).to_string(),
    }
}

fn evidence_root(config: &Config, kind: DrillStepKind, ordinal: u64) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-RECOVERY-RELEASE-DRILL-EVIDENCE",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(&config.drill_suite),
            HashPart::U64(config.recovery_height),
            HashPart::U64(config.release_request_height),
            HashPart::U64(ordinal),
            HashPart::Str(kind.as_str()),
        ],
        32,
    )
}

fn step_kind_root(drill_steps: &[DrillStep], kind: DrillStepKind) -> String {
    let records = drill_steps
        .iter()
        .filter(|step| step.kind == kind)
        .map(DrillStep::public_record)
        .collect::<Vec<_>>();
    merkle_root(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-RECOVERY-RELEASE-DRILL-KIND",
        &records,
    )
}

fn user_safe_release_verdict_root(
    config: &Config,
    counters: &Counters,
    wallet_transcript_root: &str,
    recovery_key_rotation_root: &str,
    release_request_root: &str,
    scan_proof_root: &str,
    private_note_continuity_root: &str,
    user_release_safe: bool,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-RECOVERY-RELEASE-DRILL-USER-SAFE-VERDICT",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(wallet_transcript_root),
            HashPart::Str(recovery_key_rotation_root),
            HashPart::Str(release_request_root),
            HashPart::Str(scan_proof_root),
            HashPart::Str(private_note_continuity_root),
            HashPart::U64(counters.total_steps),
            HashPart::U64(counters.safe_steps),
            HashPart::U64(counters.held_steps),
            HashPart::Str(bool_str(user_release_safe)),
        ],
        32,
    )
}

fn state_commitment_root(
    config: &Config,
    counters: &Counters,
    wallet_transcript_root: &str,
    recovery_key_rotation_root: &str,
    release_request_root: &str,
    scan_proof_root: &str,
    private_note_continuity_root: &str,
    user_safe_release_verdict_root: &str,
    drill_step_root: &str,
    user_release_safe: bool,
) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-RECOVERY-RELEASE-DRILL-STATE",
        &[
            HashPart::Str(&config.chain_id),
            HashPart::Str(PROTOCOL_VERSION),
            HashPart::Str(&config.state_root()),
            HashPart::Str(wallet_transcript_root),
            HashPart::Str(recovery_key_rotation_root),
            HashPart::Str(release_request_root),
            HashPart::Str(scan_proof_root),
            HashPart::Str(private_note_continuity_root),
            HashPart::Str(user_safe_release_verdict_root),
            HashPart::Str(drill_step_root),
            HashPart::U64(counters.total_steps),
            HashPart::U64(counters.safe_steps),
            HashPart::Str(bool_str(user_release_safe)),
        ],
        32,
    )
}

fn wallet_instruction(kind: DrillStepKind) -> &'static str {
    match kind {
        DrillStepKind::WalletTranscript => {
            "preserve wallet transcript and bind it to the recovery release drill"
        }
        DrillStepKind::RecoveryKeyRotation => {
            "rotate recovery keys before authorizing release from the force-exit package"
        }
        DrillStepKind::ReleaseRequest => {
            "submit release request only after recovery and scan roots are available"
        }
        DrillStepKind::ScanProof => {
            "verify scan proof confirmations against private wallet recovery evidence"
        }
        DrillStepKind::PrivateNoteContinuity => {
            "retain private note continuity before marking the release as user-safe"
        }
        DrillStepKind::UserSafeReleaseVerdict => {
            "record user-safe release verdict for wallet recovery release audit"
        }
    }
}

fn release_guard(kind: DrillStepKind) -> &'static str {
    match kind {
        DrillStepKind::WalletTranscript => "release blocked unless transcript root is present",
        DrillStepKind::RecoveryKeyRotation => "release blocked unless rotated recovery keys quorum",
        DrillStepKind::ReleaseRequest => "release blocked unless request root matches wallet drill",
        DrillStepKind::ScanProof => "release blocked unless scan proof meets confirmation floor",
        DrillStepKind::PrivateNoteContinuity => {
            "release blocked unless private note continuity remains intact"
        }
        DrillStepKind::UserSafeReleaseVerdict => "release allowed only with user-safe verdict root",
    }
}

fn drill_statement(kind: DrillStepKind) -> &'static str {
    match kind {
        DrillStepKind::WalletTranscript => {
            "wallet transcript is deterministically committed for recovery release"
        }
        DrillStepKind::RecoveryKeyRotation => {
            "recovery key rotation is bound before release request execution"
        }
        DrillStepKind::ReleaseRequest => {
            "release request is committed after recovery evidence is ready"
        }
        DrillStepKind::ScanProof => "scan proof confirms the wallet can observe the recovered note",
        DrillStepKind::PrivateNoteContinuity => {
            "private note continuity is preserved through the release drill"
        }
        DrillStepKind::UserSafeReleaseVerdict => {
            "user-safe release verdict is derived from all wallet recovery drill roots"
        }
    }
}

pub fn record_root(kind: &str, record: &Value) -> String {
    domain_hash(
        "MONERO-L2-PQ-BRIDGE-EXIT-FORCE-EXIT-PACKAGE-WALLET-RECOVERY-RELEASE-DRILL-RECORD",
        &[
            HashPart::Str(CHAIN_ID),
            HashPart::Str(PROTOCOL_VERSION),
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
