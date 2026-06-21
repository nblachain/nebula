use std::collections::BTreeMap;
use std::fmt;

pub type Result<T> = std::result::Result<T, RuntimeError>;
pub type Runtime = State;

const MODULE_ID: &str =
    "monero-l2-pq-bridge-exit-force-exit-wave90-production-readiness-denial-manifest";
const VERSION: &str = "wave90.production-readiness-denial.v1";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuntimeError {
    EmptyRoot { field: &'static str },
    EmptyLabel { field: &'static str },
    InvalidThreshold { field: &'static str },
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRoot { field } => write!(f, "missing root for {}", field),
            Self::EmptyLabel { field } => write!(f, "missing label for {}", field),
            Self::InvalidThreshold { field } => write!(f, "invalid threshold for {}", field),
        }
    }
}

impl std::error::Error for RuntimeError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PublicValue {
    Text(String),
    Number(u64),
    Bool(bool),
    List(Vec<PublicValue>),
    Map(BTreeMap<String, PublicValue>),
}

impl PublicValue {
    pub fn text(value: impl Into<String>) -> Self {
        Self::Text(value.into())
    }

    pub fn list(values: Vec<PublicValue>) -> Self {
        Self::List(values)
    }

    pub fn map(values: BTreeMap<String, PublicValue>) -> Self {
        Self::Map(values)
    }

    fn canonical(&self) -> String {
        match self {
            Self::Text(value) => format!("s{}:{}", value.len(), value),
            Self::Number(value) => format!("n{}", value),
            Self::Bool(value) => format!("b{}", value),
            Self::List(values) => {
                let mut out = String::from("[");
                for value in values {
                    out.push_str(&value.canonical());
                    out.push(';');
                }
                out.push(']');
                out
            }
            Self::Map(values) => {
                let mut out = String::from("{");
                for (key, value) in values {
                    out.push_str(&format!("k{}:{}=", key.len(), key));
                    out.push_str(&value.canonical());
                    out.push(';');
                }
                out.push('}');
                out
            }
        }
    }
}

pub type PublicRecord = BTreeMap<String, PublicValue>;

fn record(entries: Vec<(&str, PublicValue)>) -> PublicRecord {
    let mut map = BTreeMap::new();
    for (key, value) in entries {
        map.insert(key.to_string(), value);
    }
    map
}

fn text(value: impl Into<String>) -> PublicValue {
    PublicValue::text(value)
}

fn number(value: u64) -> PublicValue {
    PublicValue::Number(value)
}

fn flag(value: bool) -> PublicValue {
    PublicValue::Bool(value)
}

fn map(value: PublicRecord) -> PublicValue {
    PublicValue::map(value)
}

fn list(values: Vec<PublicValue>) -> PublicValue {
    PublicValue::list(values)
}

fn root_from_record(domain: &str, public: &PublicRecord) -> String {
    stable_root(domain, &PublicValue::map(public.clone()).canonical())
}

fn stable_root(domain: &str, body: &str) -> String {
    let mut a = 0xcbf29ce484222325u64;
    let mut b = 0x9e3779b97f4a7c15u64;
    let mut c = 0x100000001b3u64;
    for byte in domain.as_bytes().iter().chain(body.as_bytes()) {
        let x = u64::from(*byte);
        a ^= x;
        a = a.wrapping_mul(0x100000001b3);
        b ^= a.rotate_left(13).wrapping_add(x);
        b = b.wrapping_mul(0xff51afd7ed558ccd);
        c ^= b.rotate_right(7).wrapping_add(a);
        c = c.wrapping_mul(0xc4ceb9fe1a85ec53);
    }
    format!("{:016x}{:016x}{:016x}", a, b, c)
}

fn status_root(label: &str, status: EvidenceStatus, reason: &str) -> String {
    stable_root(label, &format!("{:?}:{}", status, reason))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EvidenceStatus {
    Present,
    Deferred,
    Missing,
    Stale,
    Ambiguous,
}

impl EvidenceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Present => "present",
            Self::Deferred => "deferred",
            Self::Missing => "missing",
            Self::Stale => "stale",
            Self::Ambiguous => "ambiguous",
        }
    }

    pub fn blocks_readiness(self) -> bool {
        !matches!(self, Self::Present)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DenialCriterion {
    BridgeCustodyReceipt,
    WatcherQuorumReceipt,
    WithdrawalReleaseReceipt,
    ReserveCoverageReceipt,
    SignerQuorumReceipt,
    ChallengeHoldReceipt,
}

impl DenialCriterion {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BridgeCustodyReceipt => "bridge_custody_receipt",
            Self::WatcherQuorumReceipt => "watcher_quorum_receipt",
            Self::WithdrawalReleaseReceipt => "withdrawal_release_receipt",
            Self::ReserveCoverageReceipt => "reserve_coverage_receipt",
            Self::SignerQuorumReceipt => "signer_quorum_receipt",
            Self::ChallengeHoldReceipt => "challenge_hold_receipt",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    Advisory,
    Hold,
    Deny,
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Advisory => "advisory",
            Self::Hold => "hold",
            Self::Deny => "deny",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VerdictKind {
    DenyProductionReadiness,
    HoldForBridgeCustody,
}

impl VerdictKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DenyProductionReadiness => "deny_production_readiness",
            Self::HoldForBridgeCustody => "hold_for_bridge_custody",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperatorActionKind {
    ProduceCustodyReceipt,
    ProduceWatcherQuorumReceipt,
    ProduceWithdrawalReleaseReceipt,
    ProduceReserveReceipt,
    ProduceSignerQuorumReceipt,
    ProduceChallengeHoldReceipt,
    KeepBridgeExitPaused,
    PublishNoGoArchiveDelta,
}

impl OperatorActionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ProduceCustodyReceipt => "produce_custody_receipt",
            Self::ProduceWatcherQuorumReceipt => "produce_watcher_quorum_receipt",
            Self::ProduceWithdrawalReleaseReceipt => "produce_withdrawal_release_receipt",
            Self::ProduceReserveReceipt => "produce_reserve_receipt",
            Self::ProduceSignerQuorumReceipt => "produce_signer_quorum_receipt",
            Self::ProduceChallengeHoldReceipt => "produce_challenge_hold_receipt",
            Self::KeepBridgeExitPaused => "keep_bridge_exit_paused",
            Self::PublishNoGoArchiveDelta => "publish_no_go_archive_delta",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub module_id: String,
    pub version: String,
    pub wave: u64,
    pub prior_wave: u64,
    pub min_watcher_quorum: u64,
    pub min_signer_quorum: u64,
    pub min_reserve_ratio_bps: u64,
    pub require_custody_receipt: bool,
    pub require_withdrawal_release_receipt: bool,
    pub require_challenge_hold_receipt: bool,
    pub privacy_roots_only: bool,
}

impl Config {
    pub fn devnet() -> Self {
        Self {
            module_id: MODULE_ID.to_string(),
            version: VERSION.to_string(),
            wave: 90,
            prior_wave: 89,
            min_watcher_quorum: 4,
            min_signer_quorum: 3,
            min_reserve_ratio_bps: 10_000,
            require_custody_receipt: true,
            require_withdrawal_release_receipt: true,
            require_challenge_hold_receipt: true,
            privacy_roots_only: true,
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.module_id.is_empty() {
            return Err(RuntimeError::EmptyLabel { field: "module_id" });
        }
        if self.version.is_empty() {
            return Err(RuntimeError::EmptyLabel { field: "version" });
        }
        if self.min_watcher_quorum == 0 {
            return Err(RuntimeError::InvalidThreshold {
                field: "min_watcher_quorum",
            });
        }
        if self.min_signer_quorum == 0 {
            return Err(RuntimeError::InvalidThreshold {
                field: "min_signer_quorum",
            });
        }
        if self.min_reserve_ratio_bps == 0 {
            return Err(RuntimeError::InvalidThreshold {
                field: "min_reserve_ratio_bps",
            });
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        record(vec![
            ("module_id", text(&self.module_id)),
            ("version", text(&self.version)),
            ("wave", number(self.wave)),
            ("prior_wave", number(self.prior_wave)),
            ("min_watcher_quorum", number(self.min_watcher_quorum)),
            ("min_signer_quorum", number(self.min_signer_quorum)),
            ("min_reserve_ratio_bps", number(self.min_reserve_ratio_bps)),
            (
                "require_custody_receipt",
                flag(self.require_custody_receipt),
            ),
            (
                "require_withdrawal_release_receipt",
                flag(self.require_withdrawal_release_receipt),
            ),
            (
                "require_challenge_hold_receipt",
                flag(self.require_challenge_hold_receipt),
            ),
            ("privacy_roots_only", flag(self.privacy_roots_only)),
        ])
    }

    pub fn state_root(&self) -> String {
        root_from_record("CONFIG", &self.public_record())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvidenceRoot {
    pub label: String,
    pub criterion: DenialCriterion,
    pub status: EvidenceStatus,
    pub root: String,
    pub note: String,
}

impl EvidenceRoot {
    pub fn new(
        label: impl Into<String>,
        criterion: DenialCriterion,
        status: EvidenceStatus,
        root: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            label: label.into(),
            criterion,
            status,
            root: root.into(),
            note: note.into(),
        }
    }

    pub fn deferred(label: &'static str, criterion: DenialCriterion, note: &'static str) -> Self {
        Self::new(
            label,
            criterion,
            EvidenceStatus::Deferred,
            status_root(label, EvidenceStatus::Deferred, note),
            note,
        )
    }

    pub fn missing(label: &'static str, criterion: DenialCriterion, note: &'static str) -> Self {
        Self::new(
            label,
            criterion,
            EvidenceStatus::Missing,
            status_root(label, EvidenceStatus::Missing, note),
            note,
        )
    }

    pub fn is_blocker(&self) -> bool {
        self.status.blocks_readiness()
    }

    pub fn validate(&self) -> Result<()> {
        if self.label.is_empty() {
            return Err(RuntimeError::EmptyLabel {
                field: "evidence_label",
            });
        }
        if self.root.is_empty() {
            return Err(RuntimeError::EmptyRoot {
                field: "evidence_root",
            });
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        record(vec![
            ("label", text(&self.label)),
            ("criterion", text(self.criterion.as_str())),
            ("status", text(self.status.as_str())),
            ("root", text(&self.root)),
            ("note", text(&self.note)),
            ("blocks_readiness", flag(self.is_blocker())),
        ])
    }

    pub fn state_root(&self) -> String {
        root_from_record("EVIDENCE", &self.public_record())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Wave89ArchiveEvidence {
    pub archive_root: String,
    pub custody_root: EvidenceRoot,
    pub watcher_root: EvidenceRoot,
    pub withdrawal_root: EvidenceRoot,
    pub reserve_root: EvidenceRoot,
    pub signer_root: EvidenceRoot,
    pub challenge_hold_root: EvidenceRoot,
    pub no_go_manifest_root: String,
}

impl Wave89ArchiveEvidence {
    pub fn devnet() -> Self {
        let custody = EvidenceRoot::deferred(
            "wave89.bridge-custody.receipt-root",
            DenialCriterion::BridgeCustodyReceipt,
            "custody receipt remains deferred for bridge exit and force exit readiness",
        );
        let watcher = EvidenceRoot::missing(
            "wave89.watcher-quorum.receipt-root",
            DenialCriterion::WatcherQuorumReceipt,
            "watcher quorum receipt is absent from the no-go archive",
        );
        let withdrawal = EvidenceRoot::deferred(
            "wave89.withdrawal-release.receipt-root",
            DenialCriterion::WithdrawalReleaseReceipt,
            "withdrawal release receipt remains deferred",
        );
        let reserve = EvidenceRoot::missing(
            "wave89.reserve-coverage.receipt-root",
            DenialCriterion::ReserveCoverageReceipt,
            "reserve coverage receipt is missing",
        );
        let signer = EvidenceRoot::deferred(
            "wave89.signer-quorum.receipt-root",
            DenialCriterion::SignerQuorumReceipt,
            "signer quorum receipt remains deferred",
        );
        let hold = EvidenceRoot::missing(
            "wave89.challenge-hold.receipt-root",
            DenialCriterion::ChallengeHoldReceipt,
            "challenge hold receipt is absent",
        );
        let seeds = vec![
            custody.state_root(),
            watcher.state_root(),
            withdrawal.state_root(),
            reserve.state_root(),
            signer.state_root(),
            hold.state_root(),
        ];
        let archive_root = stable_root("WAVE89-NO-GO-ARCHIVE", &seeds.join("|"));
        let no_go_manifest_root = stable_root("WAVE89-NO-GO-MANIFEST", &archive_root);
        Self {
            archive_root,
            custody_root: custody,
            watcher_root: watcher,
            withdrawal_root: withdrawal,
            reserve_root: reserve,
            signer_root: signer,
            challenge_hold_root: hold,
            no_go_manifest_root,
        }
    }

    pub fn evidence(&self) -> Vec<&EvidenceRoot> {
        vec![
            &self.custody_root,
            &self.watcher_root,
            &self.withdrawal_root,
            &self.reserve_root,
            &self.signer_root,
            &self.challenge_hold_root,
        ]
    }

    pub fn validate(&self) -> Result<()> {
        if self.archive_root.is_empty() {
            return Err(RuntimeError::EmptyRoot {
                field: "archive_root",
            });
        }
        if self.no_go_manifest_root.is_empty() {
            return Err(RuntimeError::EmptyRoot {
                field: "no_go_manifest_root",
            });
        }
        for evidence in self.evidence() {
            evidence.validate()?;
        }
        Ok(())
    }

    pub fn public_record(&self) -> PublicRecord {
        record(vec![
            ("archive_root", text(&self.archive_root)),
            ("no_go_manifest_root", text(&self.no_go_manifest_root)),
            ("custody_root", map(self.custody_root.public_record())),
            ("watcher_root", map(self.watcher_root.public_record())),
            ("withdrawal_root", map(self.withdrawal_root.public_record())),
            ("reserve_root", map(self.reserve_root.public_record())),
            ("signer_root", map(self.signer_root.public_record())),
            (
                "challenge_hold_root",
                map(self.challenge_hold_root.public_record()),
            ),
        ])
    }

    pub fn state_root(&self) -> String {
        root_from_record("WAVE89-ARCHIVE-EVIDENCE", &self.public_record())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CustodyBlocker {
    pub custody_receipt_root: String,
    pub withdrawal_release_root: String,
    pub custody_status: EvidenceStatus,
    pub release_status: EvidenceStatus,
    pub frozen_exit_window: bool,
}

impl CustodyBlocker {
    pub fn from_archive(archive: &Wave89ArchiveEvidence) -> Self {
        Self {
            custody_receipt_root: archive.custody_root.root.clone(),
            withdrawal_release_root: archive.withdrawal_root.root.clone(),
            custody_status: archive.custody_root.status,
            release_status: archive.withdrawal_root.status,
            frozen_exit_window: true,
        }
    }

    pub fn blocks_readiness(&self) -> bool {
        self.frozen_exit_window
            || self.custody_status.blocks_readiness()
            || self.release_status.blocks_readiness()
    }

    pub fn action_hints(&self) -> Vec<OperatorActionKind> {
        let mut hints = Vec::new();
        if self.custody_status.blocks_readiness() {
            hints.push(OperatorActionKind::ProduceCustodyReceipt);
        }
        if self.release_status.blocks_readiness() {
            hints.push(OperatorActionKind::ProduceWithdrawalReleaseReceipt);
        }
        if self.frozen_exit_window {
            hints.push(OperatorActionKind::KeepBridgeExitPaused);
        }
        hints
    }

    pub fn public_record(&self) -> PublicRecord {
        record(vec![
            ("custody_receipt_root", text(&self.custody_receipt_root)),
            (
                "withdrawal_release_root",
                text(&self.withdrawal_release_root),
            ),
            ("custody_status", text(self.custody_status.as_str())),
            ("release_status", text(self.release_status.as_str())),
            ("frozen_exit_window", flag(self.frozen_exit_window)),
            ("blocks_readiness", flag(self.blocks_readiness())),
            (
                "action_hints",
                list(
                    self.action_hints()
                        .into_iter()
                        .map(|hint| text(hint.as_str()))
                        .collect(),
                ),
            ),
        ])
    }

    pub fn state_root(&self) -> String {
        root_from_record("CUSTODY-BLOCKER", &self.public_record())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReserveBlocker {
    pub reserve_receipt_root: String,
    pub reserve_status: EvidenceStatus,
    pub required_ratio_bps: u64,
    pub attested_ratio_bps: u64,
    pub release_allowed: bool,
}

impl ReserveBlocker {
    pub fn from_archive(archive: &Wave89ArchiveEvidence, config: &Config) -> Self {
        Self {
            reserve_receipt_root: archive.reserve_root.root.clone(),
            reserve_status: archive.reserve_root.status,
            required_ratio_bps: config.min_reserve_ratio_bps,
            attested_ratio_bps: 0,
            release_allowed: false,
        }
    }

    pub fn ratio_shortfall_bps(&self) -> u64 {
        self.required_ratio_bps
            .saturating_sub(self.attested_ratio_bps)
    }

    pub fn blocks_readiness(&self) -> bool {
        self.reserve_status.blocks_readiness()
            || self.attested_ratio_bps < self.required_ratio_bps
            || !self.release_allowed
    }

    pub fn action_hints(&self) -> Vec<OperatorActionKind> {
        let mut hints = Vec::new();
        if self.blocks_readiness() {
            hints.push(OperatorActionKind::ProduceReserveReceipt);
            hints.push(OperatorActionKind::KeepBridgeExitPaused);
        }
        hints
    }

    pub fn public_record(&self) -> PublicRecord {
        record(vec![
            ("reserve_receipt_root", text(&self.reserve_receipt_root)),
            ("reserve_status", text(self.reserve_status.as_str())),
            ("required_ratio_bps", number(self.required_ratio_bps)),
            ("attested_ratio_bps", number(self.attested_ratio_bps)),
            ("ratio_shortfall_bps", number(self.ratio_shortfall_bps())),
            ("release_allowed", flag(self.release_allowed)),
            ("blocks_readiness", flag(self.blocks_readiness())),
            (
                "action_hints",
                list(
                    self.action_hints()
                        .into_iter()
                        .map(|hint| text(hint.as_str()))
                        .collect(),
                ),
            ),
        ])
    }

    pub fn state_root(&self) -> String {
        root_from_record("RESERVE-BLOCKER", &self.public_record())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WatcherQuorumBlocker {
    pub watcher_receipt_root: String,
    pub watcher_status: EvidenceStatus,
    pub required_watchers: u64,
    pub attested_watchers: u64,
    pub independent_views: u64,
}

impl WatcherQuorumBlocker {
    pub fn from_archive(archive: &Wave89ArchiveEvidence, config: &Config) -> Self {
        Self {
            watcher_receipt_root: archive.watcher_root.root.clone(),
            watcher_status: archive.watcher_root.status,
            required_watchers: config.min_watcher_quorum,
            attested_watchers: 0,
            independent_views: 0,
        }
    }

    pub fn missing_watchers(&self) -> u64 {
        self.required_watchers
            .saturating_sub(self.attested_watchers)
    }

    pub fn blocks_readiness(&self) -> bool {
        self.watcher_status.blocks_readiness()
            || self.attested_watchers < self.required_watchers
            || self.independent_views < self.required_watchers
    }

    pub fn action_hints(&self) -> Vec<OperatorActionKind> {
        if self.blocks_readiness() {
            vec![
                OperatorActionKind::ProduceWatcherQuorumReceipt,
                OperatorActionKind::KeepBridgeExitPaused,
            ]
        } else {
            Vec::new()
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        record(vec![
            ("watcher_receipt_root", text(&self.watcher_receipt_root)),
            ("watcher_status", text(self.watcher_status.as_str())),
            ("required_watchers", number(self.required_watchers)),
            ("attested_watchers", number(self.attested_watchers)),
            ("independent_views", number(self.independent_views)),
            ("missing_watchers", number(self.missing_watchers())),
            ("blocks_readiness", flag(self.blocks_readiness())),
            (
                "action_hints",
                list(
                    self.action_hints()
                        .into_iter()
                        .map(|hint| text(hint.as_str()))
                        .collect(),
                ),
            ),
        ])
    }

    pub fn state_root(&self) -> String {
        root_from_record("WATCHER-QUORUM-BLOCKER", &self.public_record())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignerQuorumBlocker {
    pub signer_receipt_root: String,
    pub signer_status: EvidenceStatus,
    pub required_signers: u64,
    pub attested_signers: u64,
    pub key_ceremony_closed: bool,
}

impl SignerQuorumBlocker {
    pub fn from_archive(archive: &Wave89ArchiveEvidence, config: &Config) -> Self {
        Self {
            signer_receipt_root: archive.signer_root.root.clone(),
            signer_status: archive.signer_root.status,
            required_signers: config.min_signer_quorum,
            attested_signers: 0,
            key_ceremony_closed: false,
        }
    }

    pub fn missing_signers(&self) -> u64 {
        self.required_signers.saturating_sub(self.attested_signers)
    }

    pub fn blocks_readiness(&self) -> bool {
        self.signer_status.blocks_readiness()
            || self.attested_signers < self.required_signers
            || !self.key_ceremony_closed
    }

    pub fn action_hints(&self) -> Vec<OperatorActionKind> {
        if self.blocks_readiness() {
            vec![
                OperatorActionKind::ProduceSignerQuorumReceipt,
                OperatorActionKind::KeepBridgeExitPaused,
            ]
        } else {
            Vec::new()
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        record(vec![
            ("signer_receipt_root", text(&self.signer_receipt_root)),
            ("signer_status", text(self.signer_status.as_str())),
            ("required_signers", number(self.required_signers)),
            ("attested_signers", number(self.attested_signers)),
            ("missing_signers", number(self.missing_signers())),
            ("key_ceremony_closed", flag(self.key_ceremony_closed)),
            ("blocks_readiness", flag(self.blocks_readiness())),
            (
                "action_hints",
                list(
                    self.action_hints()
                        .into_iter()
                        .map(|hint| text(hint.as_str()))
                        .collect(),
                ),
            ),
        ])
    }

    pub fn state_root(&self) -> String {
        root_from_record("SIGNER-QUORUM-BLOCKER", &self.public_record())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChallengeHoldBlocker {
    pub challenge_hold_root: String,
    pub hold_status: EvidenceStatus,
    pub active_hold_count: u64,
    pub hold_release_allowed: bool,
}

impl ChallengeHoldBlocker {
    pub fn from_archive(archive: &Wave89ArchiveEvidence) -> Self {
        Self {
            challenge_hold_root: archive.challenge_hold_root.root.clone(),
            hold_status: archive.challenge_hold_root.status,
            active_hold_count: 1,
            hold_release_allowed: false,
        }
    }

    pub fn blocks_readiness(&self) -> bool {
        self.hold_status.blocks_readiness()
            || self.active_hold_count > 0
            || !self.hold_release_allowed
    }

    pub fn action_hints(&self) -> Vec<OperatorActionKind> {
        if self.blocks_readiness() {
            vec![
                OperatorActionKind::ProduceChallengeHoldReceipt,
                OperatorActionKind::KeepBridgeExitPaused,
            ]
        } else {
            Vec::new()
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        record(vec![
            ("challenge_hold_root", text(&self.challenge_hold_root)),
            ("hold_status", text(self.hold_status.as_str())),
            ("active_hold_count", number(self.active_hold_count)),
            ("hold_release_allowed", flag(self.hold_release_allowed)),
            ("blocks_readiness", flag(self.blocks_readiness())),
            (
                "action_hints",
                list(
                    self.action_hints()
                        .into_iter()
                        .map(|hint| text(hint.as_str()))
                        .collect(),
                ),
            ),
        ])
    }

    pub fn state_root(&self) -> String {
        root_from_record("CHALLENGE-HOLD-BLOCKER", &self.public_record())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BridgeDenialCriteria {
    pub criteria: Vec<DenialCriterion>,
    pub severity: Severity,
    pub deny_when_any_missing: bool,
}

impl BridgeDenialCriteria {
    pub fn strict() -> Self {
        Self {
            criteria: vec![
                DenialCriterion::BridgeCustodyReceipt,
                DenialCriterion::WatcherQuorumReceipt,
                DenialCriterion::WithdrawalReleaseReceipt,
                DenialCriterion::ReserveCoverageReceipt,
                DenialCriterion::SignerQuorumReceipt,
                DenialCriterion::ChallengeHoldReceipt,
            ],
            severity: Severity::Deny,
            deny_when_any_missing: true,
        }
    }

    pub fn public_record(&self) -> PublicRecord {
        record(vec![
            (
                "criteria",
                list(
                    self.criteria
                        .iter()
                        .map(|criterion| text(criterion.as_str()))
                        .collect(),
                ),
            ),
            ("severity", text(self.severity.as_str())),
            ("deny_when_any_missing", flag(self.deny_when_any_missing)),
        ])
    }

    pub fn state_root(&self) -> String {
        root_from_record("BRIDGE-DENIAL-CRITERIA", &self.public_record())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperatorActionHint {
    pub action: OperatorActionKind,
    pub criterion: DenialCriterion,
    pub reason_root: String,
    pub release_gate: bool,
}

impl OperatorActionHint {
    pub fn public_record(&self) -> PublicRecord {
        record(vec![
            ("action", text(self.action.as_str())),
            ("criterion", text(self.criterion.as_str())),
            ("reason_root", text(&self.reason_root)),
            ("release_gate", flag(self.release_gate)),
        ])
    }

    pub fn state_root(&self) -> String {
        root_from_record("OPERATOR-ACTION-HINT", &self.public_record())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FinalVerdict {
    pub kind: VerdictKind,
    pub severity: Severity,
    pub denied: bool,
    pub held: bool,
    pub blocker_count: u64,
    pub manifest_root: String,
    pub action_root: String,
}

impl FinalVerdict {
    pub fn public_record(&self) -> PublicRecord {
        record(vec![
            ("kind", text(self.kind.as_str())),
            ("severity", text(self.severity.as_str())),
            ("denied", flag(self.denied)),
            ("held", flag(self.held)),
            ("blocker_count", number(self.blocker_count)),
            ("manifest_root", text(&self.manifest_root)),
            ("action_root", text(&self.action_root)),
        ])
    }

    pub fn state_root(&self) -> String {
        root_from_record("FINAL-VERDICT", &self.public_record())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
    pub config: Config,
    pub archive: Wave89ArchiveEvidence,
    pub criteria: BridgeDenialCriteria,
    pub custody: CustodyBlocker,
    pub reserve: ReserveBlocker,
    pub watcher_quorum: WatcherQuorumBlocker,
    pub signer_quorum: SignerQuorumBlocker,
    pub challenge_hold: ChallengeHoldBlocker,
}

impl State {
    pub fn new(config: Config, archive: Wave89ArchiveEvidence) -> Result<Self> {
        config.validate()?;
        archive.validate()?;
        Ok(Self {
            custody: CustodyBlocker::from_archive(&archive),
            reserve: ReserveBlocker::from_archive(&archive, &config),
            watcher_quorum: WatcherQuorumBlocker::from_archive(&archive, &config),
            signer_quorum: SignerQuorumBlocker::from_archive(&archive, &config),
            challenge_hold: ChallengeHoldBlocker::from_archive(&archive),
            criteria: BridgeDenialCriteria::strict(),
            config,
            archive,
        })
    }

    pub fn devnet() -> Self {
        let config = Config::devnet();
        let archive = Wave89ArchiveEvidence::devnet();
        Self {
            custody: CustodyBlocker::from_archive(&archive),
            reserve: ReserveBlocker::from_archive(&archive, &config),
            watcher_quorum: WatcherQuorumBlocker::from_archive(&archive, &config),
            signer_quorum: SignerQuorumBlocker::from_archive(&archive, &config),
            challenge_hold: ChallengeHoldBlocker::from_archive(&archive),
            criteria: BridgeDenialCriteria::strict(),
            config,
            archive,
        }
    }

    pub fn blocker_count(&self) -> u64 {
        let blockers = [
            self.custody.blocks_readiness(),
            self.reserve.blocks_readiness(),
            self.watcher_quorum.blocks_readiness(),
            self.signer_quorum.blocks_readiness(),
            self.challenge_hold.blocks_readiness(),
        ];
        blockers.iter().filter(|blocked| **blocked).count() as u64
    }

    pub fn denied(&self) -> bool {
        self.blocker_count() > 0
    }

    pub fn held(&self) -> bool {
        self.custody.blocks_readiness() || self.challenge_hold.blocks_readiness()
    }

    pub fn operator_hints(&self) -> Vec<OperatorActionHint> {
        let mut hints = Vec::new();
        if self.custody.custody_status.blocks_readiness() {
            hints.push(OperatorActionHint {
                action: OperatorActionKind::ProduceCustodyReceipt,
                criterion: DenialCriterion::BridgeCustodyReceipt,
                reason_root: self.custody.state_root(),
                release_gate: true,
            });
        }
        if self.custody.release_status.blocks_readiness() {
            hints.push(OperatorActionHint {
                action: OperatorActionKind::ProduceWithdrawalReleaseReceipt,
                criterion: DenialCriterion::WithdrawalReleaseReceipt,
                reason_root: self.custody.state_root(),
                release_gate: true,
            });
        }
        if self.watcher_quorum.blocks_readiness() {
            hints.push(OperatorActionHint {
                action: OperatorActionKind::ProduceWatcherQuorumReceipt,
                criterion: DenialCriterion::WatcherQuorumReceipt,
                reason_root: self.watcher_quorum.state_root(),
                release_gate: true,
            });
        }
        if self.reserve.blocks_readiness() {
            hints.push(OperatorActionHint {
                action: OperatorActionKind::ProduceReserveReceipt,
                criterion: DenialCriterion::ReserveCoverageReceipt,
                reason_root: self.reserve.state_root(),
                release_gate: true,
            });
        }
        if self.signer_quorum.blocks_readiness() {
            hints.push(OperatorActionHint {
                action: OperatorActionKind::ProduceSignerQuorumReceipt,
                criterion: DenialCriterion::SignerQuorumReceipt,
                reason_root: self.signer_quorum.state_root(),
                release_gate: true,
            });
        }
        if self.challenge_hold.blocks_readiness() {
            hints.push(OperatorActionHint {
                action: OperatorActionKind::ProduceChallengeHoldReceipt,
                criterion: DenialCriterion::ChallengeHoldReceipt,
                reason_root: self.challenge_hold.state_root(),
                release_gate: true,
            });
        }
        hints.push(OperatorActionHint {
            action: OperatorActionKind::PublishNoGoArchiveDelta,
            criterion: DenialCriterion::BridgeCustodyReceipt,
            reason_root: self.archive.state_root(),
            release_gate: false,
        });
        hints
    }

    pub fn action_root(&self) -> String {
        let roots = self
            .operator_hints()
            .iter()
            .map(OperatorActionHint::state_root)
            .collect::<Vec<_>>()
            .join("|");
        stable_root("OPERATOR-ACTION-ROOT", &roots)
    }

    pub fn final_verdict(&self) -> FinalVerdict {
        FinalVerdict {
            kind: if self.denied() {
                VerdictKind::DenyProductionReadiness
            } else {
                VerdictKind::HoldForBridgeCustody
            },
            severity: if self.denied() {
                Severity::Deny
            } else {
                Severity::Hold
            },
            denied: self.denied(),
            held: self.held(),
            blocker_count: self.blocker_count(),
            manifest_root: self.manifest_root(),
            action_root: self.action_root(),
        }
    }

    pub fn manifest_root(&self) -> String {
        let parts = vec![
            self.config.state_root(),
            self.archive.state_root(),
            self.criteria.state_root(),
            self.custody.state_root(),
            self.reserve.state_root(),
            self.watcher_quorum.state_root(),
            self.signer_quorum.state_root(),
            self.challenge_hold.state_root(),
        ];
        stable_root("WAVE90-DENIAL-MANIFEST", &parts.join("|"))
    }

    pub fn public_record(&self) -> PublicRecord {
        let hints = self
            .operator_hints()
            .into_iter()
            .map(|hint| map(hint.public_record()))
            .collect();
        record(vec![
            ("module_id", text(&self.config.module_id)),
            ("version", text(&self.config.version)),
            ("config", map(self.config.public_record())),
            ("wave89_archive", map(self.archive.public_record())),
            ("criteria", map(self.criteria.public_record())),
            ("custody_blocker", map(self.custody.public_record())),
            ("reserve_blocker", map(self.reserve.public_record())),
            (
                "watcher_quorum_blocker",
                map(self.watcher_quorum.public_record()),
            ),
            (
                "signer_quorum_blocker",
                map(self.signer_quorum.public_record()),
            ),
            (
                "challenge_hold_blocker",
                map(self.challenge_hold.public_record()),
            ),
            ("operator_hints", list(hints)),
            ("final_verdict", map(self.final_verdict().public_record())),
            ("state_root", text(self.state_root())),
        ])
    }

    pub fn state_root(&self) -> String {
        root_from_record(
            "STATE",
            &record(vec![
                ("manifest_root", text(self.manifest_root())),
                ("action_root", text(self.action_root())),
                ("verdict_root", text(self.final_verdict().state_root())),
            ]),
        )
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
