use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::traits::Identity;
use merlin::Transcript;

pub const RANGE_BITS: usize = 64;
const TRANSCRIPT_LABEL: &[u8] = b"nebula-confidential-amount-v1";

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

    fn point(&self) -> Option<RistrettoPoint> {
        self.0.decompress()
    }
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
}
