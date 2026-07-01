use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::traits::Identity;
use merlin::Transcript;
use sha3::{Digest, Sha3_256};

pub const RANGE_BITS: usize = 64;
const TRANSCRIPT_LABEL: &[u8] = b"nebula-confidential-amount-v1";
pub const NULLIFIER_DOMAIN: &[u8] = b"nebula-nullifier-v1";

fn pedersen_gens() -> PedersenGens {
    PedersenGens::default()
}

fn bulletproof_gens() -> BulletproofGens {
    BulletproofGens::new(RANGE_BITS, 1)
}

#[derive(Clone)]
pub struct Blinding(Scalar);

impl Blinding {
    pub fn random() -> Self {
        let mut wide = [0u8; 64];
        getrandom::getrandom(&mut wide).expect("OS randomness for blinding factor");
        Blinding(Scalar::from_bytes_mod_order_wide(&wide))
    }

    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Blinding(Scalar::from_bytes_mod_order(bytes))
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }

    pub fn add(&self, other: &Blinding) -> Blinding {
        Blinding(self.0 + other.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Commitment(CompressedRistretto);

impl Commitment {
    pub fn to_hex(&self) -> String {
        hex::encode(self.0.as_bytes())
    }

    pub fn from_hex(value: &str) -> Result<Self, String> {
        let bytes = hex::decode(value).map_err(|error| format!("commitment hex: {error}"))?;
        let compressed = CompressedRistretto::from_slice(&bytes)
            .map_err(|_| "commitment must be 32 bytes".to_string())?;
        Ok(Commitment(compressed))
    }

    pub fn as_bytes(&self) -> [u8; 32] {
        *self.0.as_bytes()
    }

    fn point(&self) -> Option<RistrettoPoint> {
        self.0.decompress()
    }
}

pub fn nullifier(blinding: &Blinding, commitment: &Commitment) -> [u8; 32] {
    let mut hasher = Sha3_256::new();
    hasher.update(NULLIFIER_DOMAIN);
    hasher.update(blinding.to_bytes());
    hasher.update(commitment.as_bytes());
    hasher.finalize().into()
}

pub fn nullifier_hex(blinding_hex: &str, commitment_hex: &str) -> Result<String, String> {
    let blinding_bytes =
        hex::decode(blinding_hex).map_err(|error| format!("blinding hex: {error}"))?;
    let blinding_array: [u8; 32] = blinding_bytes
        .as_slice()
        .try_into()
        .map_err(|_| "blinding must be 32 bytes".to_string())?;
    let blinding = Blinding::from_bytes(blinding_array);
    let commitment = Commitment::from_hex(commitment_hex)?;
    Ok(hex::encode(nullifier(&blinding, &commitment)))
}

pub fn commit(value: u64, blinding: &Blinding) -> Commitment {
    Commitment(
        pedersen_gens()
            .commit(Scalar::from(value), blinding.0)
            .compress(),
    )
}

pub fn prove_amount(value: u64, blinding: &Blinding) -> (Commitment, Vec<u8>) {
    let mut transcript = Transcript::new(TRANSCRIPT_LABEL);
    let (proof, committed) = RangeProof::prove_single(
        &bulletproof_gens(),
        &pedersen_gens(),
        &mut transcript,
        value,
        &blinding.0,
        RANGE_BITS,
    )
    .expect("range proof generation cannot fail for a u64 value");
    (Commitment(committed), proof.to_bytes())
}

pub fn verify_amount(commitment: &Commitment, proof_bytes: &[u8]) -> bool {
    let Ok(proof) = RangeProof::from_bytes(proof_bytes) else {
        return false;
    };
    let mut transcript = Transcript::new(TRANSCRIPT_LABEL);
    proof
        .verify_single(
            &bulletproof_gens(),
            &pedersen_gens(),
            &mut transcript,
            &commitment.0,
            RANGE_BITS,
        )
        .is_ok()
}

pub fn amounts_balance(inputs: &[Commitment], outputs: &[Commitment], fee: &Commitment) -> bool {
    let sum = |commitments: &[Commitment]| -> Option<RistrettoPoint> {
        commitments
            .iter()
            .try_fold(RistrettoPoint::identity(), |acc, c| Some(acc + c.point()?))
    };
    match (sum(inputs), sum(outputs), fee.point()) {
        (Some(input_sum), Some(output_sum), Some(fee_point)) => input_sum == output_sum + fee_point,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commitment_matches_range_proof_commitment() {
        let blinding = Blinding::from_bytes([7u8; 32]);
        let (committed, _proof) = prove_amount(1234, &blinding);
        assert_eq!(commit(1234, &blinding), committed);
    }

    #[test]
    fn range_proof_round_trips() {
        let blinding = Blinding::from_bytes([7u8; 32]);
        let (commitment, proof) = prove_amount(1_000_000, &blinding);
        assert!(verify_amount(&commitment, &proof));
    }

    #[test]
    fn range_proof_rejects_tampering_and_wrong_commitment() {
        let blinding = Blinding::from_bytes([7u8; 32]);
        let (commitment, proof) = prove_amount(1234, &blinding);

        let mut tampered = proof.clone();
        tampered[0] ^= 0xff;
        assert!(!verify_amount(&commitment, &tampered));

        let (other_commitment, _) = prove_amount(5678, &Blinding::from_bytes([8u8; 32]));
        assert!(!verify_amount(&other_commitment, &proof));

        assert!(!verify_amount(&commitment, b"not a proof"));
    }

    #[test]
    fn balanced_commitments_verify_and_inflation_is_caught() {
        let r_out = Blinding::from_bytes([2u8; 32]);
        let r_fee = Blinding::from_bytes([3u8; 32]);
        let r_in = r_out.add(&r_fee);

        let input = commit(100, &r_in);
        let output = commit(70, &r_out);
        let fee = commit(30, &r_fee);
        assert!(amounts_balance(&[input], &[output], &fee));

        let inflated = commit(80, &r_out);
        assert!(!amounts_balance(&[input], &[inflated], &fee));
    }

    #[test]
    fn multi_input_output_balance() {
        let r_in1 = Blinding::from_bytes([11u8; 32]);
        let r_in2 = Blinding::from_bytes([12u8; 32]);
        let r_out = Blinding::from_bytes([13u8; 32]);
        let r_fee = Blinding(r_in1.0 + r_in2.0 - r_out.0);
        let inputs = [commit(60, &r_in1), commit(40, &r_in2)];
        let outputs = [commit(90, &r_out)];
        let fee = commit(10, &r_fee);
        assert!(amounts_balance(&inputs, &outputs, &fee));
    }

    #[test]
    fn commitment_hex_round_trips() {
        let commitment = commit(42, &Blinding::from_bytes([9u8; 32]));
        let hex = commitment.to_hex();
        assert_eq!(hex.len(), 64);
        assert_eq!(Commitment::from_hex(&hex).unwrap(), commitment);
        assert!(Commitment::from_hex("zz").is_err());
    }

    #[test]
    fn nullifier_is_deterministic() {
        let blinding = Blinding::from_bytes([5u8; 32]);
        let commitment = commit(100, &blinding);
        assert_eq!(
            nullifier(&blinding, &commitment),
            nullifier(&blinding, &commitment)
        );
    }

    #[test]
    fn nullifiers_differ_across_notes() {
        let b1 = Blinding::from_bytes([5u8; 32]);
        let b2 = Blinding::from_bytes([6u8; 32]);
        let c1 = commit(100, &b1);
        let c2 = commit(100, &b2);
        assert_ne!(nullifier(&b1, &c1), nullifier(&b2, &c2));
        assert_ne!(nullifier(&b1, &c1), nullifier(&b1, &c2));
    }

    #[test]
    fn nullifier_hex_len_and_bad_input() {
        let blinding = Blinding::from_bytes([5u8; 32]);
        let commitment = commit(100, &blinding);
        let nf = nullifier_hex(&hex::encode(blinding.to_bytes()), &commitment.to_hex()).unwrap();
        assert_eq!(nf.len(), 64);
        assert_eq!(nf, hex::encode(nullifier(&blinding, &commitment)));
        assert!(nullifier_hex("zz", &commitment.to_hex()).is_err());
        assert!(nullifier_hex(&hex::encode(blinding.to_bytes()), "zz").is_err());
    }

    #[test]
    fn balance_holds_for_valid_splits_and_rejects_inflation_across_many_cases() {
        let mut state: u64 = 0x00c0_ffee_1234_5678;
        let mut next = || {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            state >> 33
        };
        let seed = |value: u64| -> [u8; 32] {
            let mut bytes = [0u8; 32];
            bytes[..8].copy_from_slice(&value.to_le_bytes());
            bytes[8] = 1;
            bytes
        };
        for case in 0..300u64 {
            let out1 = next() % 1_000_000;
            let out2 = next() % 1_000_000;
            let fee = next() % 10_000;
            let total = out1 + out2 + fee;

            let r_out1 = Blinding::from_bytes(seed(next()));
            let r_out2 = Blinding::from_bytes(seed(next()));
            let r_fee = Blinding::from_bytes(seed(next()));
            let r_in = r_out1.add(&r_out2).add(&r_fee);

            let input = commit(total, &r_in);
            let o1 = commit(out1, &r_out1);
            let o2 = commit(out2, &r_out2);
            let fee_commitment = commit(fee, &r_fee);

            assert!(
                amounts_balance(&[input], &[o1, o2], &fee_commitment),
                "case {case} with a correct split must balance"
            );

            let inflated = commit(out1 + 1, &r_out1);
            assert!(
                !amounts_balance(&[input], &[inflated, o2], &fee_commitment),
                "case {case} inflating an output by 1 must be caught"
            );
        }
    }
}
