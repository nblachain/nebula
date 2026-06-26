use serde::Serialize;
use serde_json::{json, Value};
use sha3::{Digest, Sha3_256};
use std::time::{SystemTime, UNIX_EPOCH};

pub const VERSION: &str = "nebula-testnet-runner/0.2.0";
pub const CHAIN_ID: &str = "nebula-private-l2-testnet";
pub const PUBLIC_LAUNCH_BLOCKER: &str = "public-launch-deployment-attestation";
pub const NBLA_SYMBOL: &str = "NBLA";
pub const NXMR_SYMBOL: &str = "nXMR";
pub const NEBULAI_UNIT: &str = "nebulai";
pub const NEBULAI_PER_NBLA: u128 = 1_000_000;
pub const NBLA_TARGET_NXMR_NUMERATOR: u128 = 1;
pub const NBLA_TARGET_NXMR_DENOMINATOR: u128 = 1_000;
pub const TARGET_NXMR_BASE_UNITS_PER_NXMR: u128 =
    NEBULAI_PER_NBLA * NBLA_TARGET_NXMR_DENOMINATOR / NBLA_TARGET_NXMR_NUMERATOR;
pub const TARGET_NXMR_TO_NBLA_RATE_NEBULAI_PER_UNIT: u128 = 1;
pub const FEE_BASIS_POINTS: u128 = 10_000;
pub const NXMR_RESERVE_BACKING_BPS: u128 = 9_000;
pub const NXMR_VALIDATOR_REWARD_BPS: u128 = 1_000;
pub const TESTNET_POINTS_PER_NEBULAI: u128 = 1;

#[derive(Debug, Clone, Serialize)]
pub struct Acceptance {
    pub nebula_guide_mirrored: bool,
    pub testnet_ready: bool,
    pub ci_owned_by_nebula: bool,
    pub legacy_upstream_removed: bool,
    pub local_runtime_buildable: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct PublicLaunchReadiness {
    pub public_launch_ready: bool,
    pub level: String,
    pub blocking_gaps: Vec<String>,
    pub required_attestation: String,
    pub remediation_root: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct NebulaReadiness {
    pub chain_id: String,
    pub version: String,
    pub generated_at_unix_ms: u128,
    pub acceptance: Acceptance,
    pub public_launch_readiness: PublicLaunchReadiness,
    pub status_roots: Value,
    pub economics: HybridFeePolicy,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FeeAsset {
    Nbla,
    NXmr,
}

impl FeeAsset {
    pub fn symbol(self) -> &'static str {
        match self {
            Self::Nbla => NBLA_SYMBOL,
            Self::NXmr => NXMR_SYMBOL,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct HybridFeePolicy {
    pub native_fee_token: &'static str,
    pub bridged_fee_token: &'static str,
    pub native_base_unit: &'static str,
    pub nebulai_per_nbla: u128,
    pub target_nxmr_per_nbla_numerator: u128,
    pub target_nxmr_per_nbla_denominator: u128,
    pub target_nxmr_base_units_per_nxmr: u128,
    pub target_nxmr_to_nbla_rate_nebulai_per_unit: u128,
    pub bridged_fee_conversion: &'static str,
    pub nxmr_reserve_backing_bps: u128,
    pub nxmr_validator_reward_bps: u128,
    pub nbla_validator_reward_bps: u128,
    pub testnet_reward_unit: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct HybridFeeQuote {
    pub payment_asset: FeeAsset,
    pub payment_asset_symbol: &'static str,
    pub gas_units: u128,
    pub gas_price_nebulai: u128,
    pub required_fee_nebulai: u128,
    pub nxmr_to_nbla_rate_nebulai_per_unit: Option<u128>,
    pub paid_amount_units: u128,
    pub converted_nbla_nebulai: u128,
    pub reserve_backing_nebulai: u128,
    pub validator_reward_nebulai: u128,
    pub validator_points: u128,
    pub settlement_note: &'static str,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FeeError {
    ZeroGas,
    ZeroGasPrice,
    MissingNXmrRate,
    ZeroNXmrRate,
    ArithmeticOverflow,
}

pub fn hybrid_fee_policy() -> HybridFeePolicy {
    HybridFeePolicy {
        native_fee_token: NBLA_SYMBOL,
        bridged_fee_token: NXMR_SYMBOL,
        native_base_unit: NEBULAI_UNIT,
        nebulai_per_nbla: NEBULAI_PER_NBLA,
        target_nxmr_per_nbla_numerator: NBLA_TARGET_NXMR_NUMERATOR,
        target_nxmr_per_nbla_denominator: NBLA_TARGET_NXMR_DENOMINATOR,
        target_nxmr_base_units_per_nxmr: TARGET_NXMR_BASE_UNITS_PER_NXMR,
        target_nxmr_to_nbla_rate_nebulai_per_unit: TARGET_NXMR_TO_NBLA_RATE_NEBULAI_PER_UNIT,
        bridged_fee_conversion: "nXMR fees are converted into NBLA accounting value before split",
        nxmr_reserve_backing_bps: NXMR_RESERVE_BACKING_BPS,
        nxmr_validator_reward_bps: NXMR_VALIDATOR_REWARD_BPS,
        nbla_validator_reward_bps: FEE_BASIS_POINTS,
        testnet_reward_unit: "non-transferable validator points",
    }
}

pub fn quote_hybrid_fee(
    payment_asset: FeeAsset,
    gas_units: u128,
    gas_price_nebulai: u128,
    nxmr_to_nbla_rate_nebulai_per_unit: Option<u128>,
) -> Result<HybridFeeQuote, FeeError> {
    if gas_units == 0 {
        return Err(FeeError::ZeroGas);
    }
    if gas_price_nebulai == 0 {
        return Err(FeeError::ZeroGasPrice);
    }

    let required_fee_nebulai = gas_units
        .checked_mul(gas_price_nebulai)
        .ok_or(FeeError::ArithmeticOverflow)?;

    match payment_asset {
        FeeAsset::Nbla => Ok(HybridFeeQuote {
            payment_asset,
            payment_asset_symbol: payment_asset.symbol(),
            gas_units,
            gas_price_nebulai,
            required_fee_nebulai,
            nxmr_to_nbla_rate_nebulai_per_unit: None,
            paid_amount_units: required_fee_nebulai,
            converted_nbla_nebulai: required_fee_nebulai,
            reserve_backing_nebulai: 0,
            validator_reward_nebulai: required_fee_nebulai,
            validator_points: required_fee_nebulai
                .checked_mul(TESTNET_POINTS_PER_NEBULAI)
                .ok_or(FeeError::ArithmeticOverflow)?,
            settlement_note: "NBLA gas is paid directly to the validator reward ledger",
        }),
        FeeAsset::NXmr => {
            let rate = nxmr_to_nbla_rate_nebulai_per_unit.ok_or(FeeError::MissingNXmrRate)?;
            if rate == 0 {
                return Err(FeeError::ZeroNXmrRate);
            }

            let paid_amount_units = ceil_div(required_fee_nebulai, rate);
            let converted_nbla_nebulai = paid_amount_units
                .checked_mul(rate)
                .ok_or(FeeError::ArithmeticOverflow)?;
            let reserve_backing_nebulai =
                split_basis_points(converted_nbla_nebulai, NXMR_RESERVE_BACKING_BPS)?;
            let validator_reward_nebulai = converted_nbla_nebulai - reserve_backing_nebulai;

            Ok(HybridFeeQuote {
                payment_asset,
                payment_asset_symbol: payment_asset.symbol(),
                gas_units,
                gas_price_nebulai,
                required_fee_nebulai,
                nxmr_to_nbla_rate_nebulai_per_unit: Some(rate),
                paid_amount_units,
                converted_nbla_nebulai,
                reserve_backing_nebulai,
                validator_reward_nebulai,
                validator_points: validator_reward_nebulai
                    .checked_mul(TESTNET_POINTS_PER_NEBULAI)
                    .ok_or(FeeError::ArithmeticOverflow)?,
                settlement_note:
                    "nXMR gas is converted to NBLA value: 90% backs NBLA, 10% rewards validators",
            })
        }
    }
}

pub fn quote_hybrid_fee_at_target_rate(
    payment_asset: FeeAsset,
    gas_units: u128,
    gas_price_nebulai: u128,
) -> Result<HybridFeeQuote, FeeError> {
    let nxmr_rate = match payment_asset {
        FeeAsset::Nbla => None,
        FeeAsset::NXmr => Some(TARGET_NXMR_TO_NBLA_RATE_NEBULAI_PER_UNIT),
    };
    quote_hybrid_fee(payment_asset, gas_units, gas_price_nebulai, nxmr_rate)
}

pub fn readiness_report() -> NebulaReadiness {
    let acceptance = Acceptance {
        nebula_guide_mirrored: true,
        testnet_ready: true,
        ci_owned_by_nebula: true,
        legacy_upstream_removed: true,
        local_runtime_buildable: true,
    };

    let blocking_gaps = vec![PUBLIC_LAUNCH_BLOCKER.to_string()];
    let required_attestation =
        "operator-signed public endpoint, surface probe, and rollback evidence".to_string();
    let remediation_root = stable_root(&json!({
        "required_gap": PUBLIC_LAUNCH_BLOCKER,
        "required_attestation": required_attestation,
        "minimum_observer_count": 2,
        "minimum_operator_count": 2,
        "minimum_region_count": 2,
    }));

    let public_launch_readiness = PublicLaunchReadiness {
        public_launch_ready: false,
        level: "public-launch-blocked".to_string(),
        blocking_gaps,
        required_attestation,
        remediation_root,
    };

    NebulaReadiness {
        chain_id: CHAIN_ID.to_string(),
        version: VERSION.to_string(),
        generated_at_unix_ms: unix_ms(),
        status_roots: json!({
            "runtime": stable_root(&json!({
                "chain_id": CHAIN_ID,
                "version": VERSION,
                "mode": "private-l2-testnet",
            })),
            "ci": stable_root(&json!({
                "workflow": "nebula-ci",
                "checks": [
                    "format",
                    "build",
                    "test-suite",
                    "readiness-contract",
                    "guide-mirror"
                ],
            })),
            "economics": stable_root(&json!({
                "native_fee_token": NBLA_SYMBOL,
                "bridged_fee_token": NXMR_SYMBOL,
                "native_base_unit": NEBULAI_UNIT,
                "nebulai_per_nbla": NEBULAI_PER_NBLA,
                "target_nxmr_per_nbla": "0.001",
                "target_nxmr_base_units_per_nxmr": TARGET_NXMR_BASE_UNITS_PER_NXMR,
                "target_nxmr_to_nbla_rate_nebulai_per_unit": TARGET_NXMR_TO_NBLA_RATE_NEBULAI_PER_UNIT,
                "nxmr_reserve_backing_bps": NXMR_RESERVE_BACKING_BPS,
                "nxmr_validator_reward_bps": NXMR_VALIDATOR_REWARD_BPS,
                "testnet_reward_unit": "non-transferable validator points",
            })),
            "guide": stable_root(&json!({
                "root_readme": "README.md",
                "guide": "docs/NEBULA_LAYER2.md",
                "mirror_required": true,
            })),
        }),
        acceptance,
        public_launch_readiness,
        economics: hybrid_fee_policy(),
    }
}

pub fn readiness_json_pretty() -> String {
    serde_json::to_string_pretty(&readiness_report()).expect("readiness report serializes")
}

pub fn readiness_summary() -> String {
    let report = readiness_report();
    format!(
        "Nebula local testnet is ready. Public launch is blocked by: {}",
        report.public_launch_readiness.blocking_gaps.join(", ")
    )
}

fn stable_root(value: &Value) -> String {
    let bytes = serde_json::to_vec(value).expect("status root input serializes");
    let digest = Sha3_256::digest(bytes);
    hex::encode(digest)
}

fn split_basis_points(amount: u128, bps: u128) -> Result<u128, FeeError> {
    amount
        .checked_mul(bps)
        .ok_or(FeeError::ArithmeticOverflow)
        .map(|scaled| scaled / FEE_BASIS_POINTS)
}

fn ceil_div(numerator: u128, denominator: u128) -> u128 {
    numerator / denominator + u128::from(numerator % denominator != 0)
}

fn unix_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default()
}

#[cfg(test)]
mod public_launch {
    use super::*;

    #[test]
    fn public_launch_blocks_without_deployment_attestation() {
        let report = readiness_report();

        assert!(report.acceptance.testnet_ready);
        assert!(!report.public_launch_readiness.public_launch_ready);
        assert_eq!(
            report.public_launch_readiness.level,
            "public-launch-blocked"
        );
        assert_eq!(
            report.public_launch_readiness.blocking_gaps,
            vec![PUBLIC_LAUNCH_BLOCKER.to_string()]
        );
    }

    #[test]
    fn public_launch_remediation_root_is_stable_shape() {
        let report = readiness_report();

        assert_eq!(report.public_launch_readiness.remediation_root.len(), 64);
        assert!(report
            .public_launch_readiness
            .remediation_root
            .chars()
            .all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn public_launch_readiness_includes_hybrid_fee_policy() {
        let report = readiness_report();

        assert_eq!(report.economics.native_fee_token, NBLA_SYMBOL);
        assert_eq!(report.economics.bridged_fee_token, NXMR_SYMBOL);
        assert_eq!(report.economics.native_base_unit, NEBULAI_UNIT);
        assert_eq!(report.economics.nebulai_per_nbla, 1_000_000);
        assert_eq!(report.economics.target_nxmr_per_nbla_numerator, 1);
        assert_eq!(report.economics.target_nxmr_per_nbla_denominator, 1_000);
        assert_eq!(report.economics.nxmr_reserve_backing_bps, 9_000);
        assert_eq!(report.economics.nxmr_validator_reward_bps, 1_000);
    }
}

#[cfg(test)]
mod economics {
    use super::*;

    #[test]
    fn nbla_fee_goes_directly_to_validator_rewards() {
        let quote = quote_hybrid_fee(FeeAsset::Nbla, 25, 4_000, None).unwrap();

        assert_eq!(quote.payment_asset_symbol, NBLA_SYMBOL);
        assert_eq!(quote.required_fee_nebulai, 100_000);
        assert_eq!(quote.paid_amount_units, 100_000);
        assert_eq!(quote.reserve_backing_nebulai, 0);
        assert_eq!(quote.validator_reward_nebulai, 100_000);
        assert_eq!(quote.validator_points, 100_000);
    }

    #[test]
    fn nxmr_fee_converts_to_nbla_and_splits_ninety_ten() {
        let quote = quote_hybrid_fee_at_target_rate(FeeAsset::NXmr, 100, 10_000).unwrap();

        assert_eq!(quote.payment_asset_symbol, NXMR_SYMBOL);
        assert_eq!(quote.required_fee_nebulai, 1_000_000);
        assert_eq!(quote.paid_amount_units, 1_000_000);
        assert_eq!(quote.converted_nbla_nebulai, 1_000_000);
        assert_eq!(quote.reserve_backing_nebulai, 900_000);
        assert_eq!(quote.validator_reward_nebulai, 100_000);
        assert_eq!(quote.validator_points, 100_000);
    }

    #[test]
    fn nbla_target_rate_maps_one_nebulai_to_one_nxmr_base_unit() {
        assert_eq!(NEBULAI_PER_NBLA, 1_000_000);
        assert_eq!(TARGET_NXMR_BASE_UNITS_PER_NXMR, 1_000_000_000);
        assert_eq!(TARGET_NXMR_TO_NBLA_RATE_NEBULAI_PER_UNIT, 1);
    }

    #[test]
    fn nxmr_fee_requires_conversion_rate() {
        assert_eq!(
            quote_hybrid_fee(FeeAsset::NXmr, 1, 1, None).unwrap_err(),
            FeeError::MissingNXmrRate
        );
    }
}
