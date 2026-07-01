use ml_dsa::{
    EncodedVerifyingKey, Keypair, MlDsa65, Signature, Signer, SigningKey, Verifier, VerifyingKey,
    B32,
};

use crate::{
    public_key_hex_for_secret, sign_ed25519_root, verify_ed25519_signature, verifying_key_from_hex,
};

pub const ED25519_PUBLIC_LEN: usize = 32;
pub const ED25519_SIGNATURE_LEN: usize = 64;
pub const ED25519_SEED_LEN: usize = 32;

pub const MLDSA65_PUBLIC_LEN: usize = 1952;
pub const MLDSA65_SIGNATURE_LEN: usize = 3309;
pub const MLDSA65_SEED_LEN: usize = 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SchemeId {
    Ed25519,
    MlDsa65,
    HybridEd25519MlDsa65,
}

impl SchemeId {
    pub const fn tag(self) -> &'static str {
        match self {
            SchemeId::Ed25519 => "ed25519",
            SchemeId::MlDsa65 => "mldsa65",
            SchemeId::HybridEd25519MlDsa65 => "hybrid-ed25519-mldsa65",
        }
    }

    pub fn from_tag(tag: &str) -> Option<SchemeId> {
        match tag {
            "ed25519" => Some(SchemeId::Ed25519),
            "mldsa65" => Some(SchemeId::MlDsa65),
            "hybrid-ed25519-mldsa65" => Some(SchemeId::HybridEd25519MlDsa65),
            _ => None,
        }
    }

    pub const fn public_len(self) -> usize {
        match self {
            SchemeId::Ed25519 => ED25519_PUBLIC_LEN,
            SchemeId::MlDsa65 => MLDSA65_PUBLIC_LEN,
            SchemeId::HybridEd25519MlDsa65 => ED25519_PUBLIC_LEN + MLDSA65_PUBLIC_LEN,
        }
    }

    pub const fn signature_len(self) -> usize {
        match self {
            SchemeId::Ed25519 => ED25519_SIGNATURE_LEN,
            SchemeId::MlDsa65 => MLDSA65_SIGNATURE_LEN,
            SchemeId::HybridEd25519MlDsa65 => ED25519_SIGNATURE_LEN + MLDSA65_SIGNATURE_LEN,
        }
    }

    pub const fn seed_len(self) -> usize {
        match self {
            SchemeId::Ed25519 => ED25519_SEED_LEN,
            SchemeId::MlDsa65 => MLDSA65_SEED_LEN,
            SchemeId::HybridEd25519MlDsa65 => ED25519_SEED_LEN + MLDSA65_SEED_LEN,
        }
    }
}

fn require_root(root: &str) -> Result<(), String> {
    if root.len() != 64 || !root.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("signing_root must be a 64-character hex value".to_string());
    }
    Ok(())
}

fn parse_tagged(value: &str, kind: &str) -> Result<(SchemeId, Vec<u8>), String> {
    let (scheme, hex_part) = match value.split_once(':') {
        Some((tag, rest)) => {
            let scheme = SchemeId::from_tag(tag)
                .ok_or_else(|| format!("{kind} has an unknown scheme tag '{tag}'"))?;
            (scheme, rest)
        }
        None => (SchemeId::Ed25519, value),
    };
    let bytes =
        hex::decode(hex_part).map_err(|error| format!("{kind} is not valid hex: {error}"))?;
    Ok((scheme, bytes))
}

fn encode_value(scheme: SchemeId, hex_value: String) -> String {
    match scheme {
        SchemeId::Ed25519 => hex_value,
        _ => format!("{}:{}", scheme.tag(), hex_value),
    }
}

pub fn validate_scheme_public(value: &str, name: &str) -> Result<SchemeId, String> {
    let (scheme, bytes) = parse_tagged(value, name)?;
    if bytes.len() != scheme.public_len() {
        return Err(format!(
            "{name} must be a {}-byte {} public key, got {} bytes",
            scheme.public_len(),
            scheme.tag(),
            bytes.len()
        ));
    }
    match scheme {
        SchemeId::Ed25519 => {
            verifying_key_from_hex(&hex::encode(&bytes), name)?;
        }
        SchemeId::MlDsa65 => {
            mldsa_verifying_key_from_bytes(&bytes)?;
        }
        SchemeId::HybridEd25519MlDsa65 => {
            let (ed_public, ml_public) = bytes.split_at(ED25519_PUBLIC_LEN);
            verifying_key_from_hex(&hex::encode(ed_public), name)?;
            mldsa_verifying_key_from_bytes(ml_public)?;
        }
    }
    Ok(scheme)
}

pub fn scheme_tag_for_public(value: &str, name: &str) -> Result<&'static str, String> {
    Ok(validate_scheme_public(value, name)?.tag())
}

pub fn scheme_normalize_public(value: &str, name: &str) -> Result<String, String> {
    validate_scheme_public(value, name)?;
    Ok(value.to_ascii_lowercase())
}

pub fn scheme_secret_from_seed(scheme: SchemeId, seed_hex: &str) -> Result<String, String> {
    let bytes = hex::decode(seed_hex).map_err(|error| format!("seed is not valid hex: {error}"))?;
    if bytes.len() != scheme.seed_len() {
        return Err(format!(
            "{} secret seed must be {} bytes, got {}",
            scheme.tag(),
            scheme.seed_len(),
            bytes.len()
        ));
    }
    Ok(encode_value(scheme, hex::encode(&bytes)))
}

pub fn scheme_derive_public(secret: &str) -> Result<String, String> {
    let (scheme, seed) = parse_tagged(secret, "secret key")?;
    if seed.len() != scheme.seed_len() {
        return Err(format!(
            "{} secret seed must be {} bytes, got {}",
            scheme.tag(),
            scheme.seed_len(),
            seed.len()
        ));
    }
    match scheme {
        SchemeId::Ed25519 => {
            let public = public_key_hex_for_secret(&hex::encode(&seed), "secret key")?;
            Ok(encode_value(scheme, public))
        }
        SchemeId::MlDsa65 => {
            let public = mldsa_public_from_seed(&seed)?;
            Ok(encode_value(scheme, hex::encode(public)))
        }
        SchemeId::HybridEd25519MlDsa65 => {
            let (ed_seed, ml_seed) = seed.split_at(ED25519_SEED_LEN);
            let ed_public = public_key_hex_for_secret(&hex::encode(ed_seed), "secret key")?;
            let ml_public = mldsa_public_from_seed(ml_seed)?;
            Ok(encode_value(
                scheme,
                format!("{ed_public}{}", hex::encode(ml_public)),
            ))
        }
    }
}

pub fn scheme_sign_root(secret: &str, root: &str) -> Result<String, String> {
    require_root(root)?;
    let (scheme, seed) = parse_tagged(secret, "secret key")?;
    if seed.len() != scheme.seed_len() {
        return Err(format!(
            "{} secret seed must be {} bytes, got {}",
            scheme.tag(),
            scheme.seed_len(),
            seed.len()
        ));
    }
    match scheme {
        SchemeId::Ed25519 => {
            let signature = sign_ed25519_root(&hex::encode(&seed), "secret key", root)?;
            Ok(encode_value(scheme, signature))
        }
        SchemeId::MlDsa65 => {
            let signature = mldsa_sign(&seed, root.as_bytes())?;
            Ok(encode_value(scheme, hex::encode(signature)))
        }
        SchemeId::HybridEd25519MlDsa65 => {
            let (ed_seed, ml_seed) = seed.split_at(ED25519_SEED_LEN);
            let ed_sig = sign_ed25519_root(&hex::encode(ed_seed), "secret key", root)?;
            let ml_sig = mldsa_sign(ml_seed, root.as_bytes())?;
            Ok(encode_value(
                scheme,
                format!("{ed_sig}{}", hex::encode(ml_sig)),
            ))
        }
    }
}

pub fn scheme_verify_root(public: &str, root: &str, signature: &str) -> Result<(), String> {
    require_root(root)?;
    let (public_scheme, public_bytes) = parse_tagged(public, "public key")?;
    let (signature_scheme, signature_bytes) = parse_tagged(signature, "signature")?;
    if public_scheme != signature_scheme {
        return Err(format!(
            "scheme mismatch: public key is {} but signature is {}",
            public_scheme.tag(),
            signature_scheme.tag()
        ));
    }
    if public_bytes.len() != public_scheme.public_len() {
        return Err(format!(
            "{} public key must be {} bytes",
            public_scheme.tag(),
            public_scheme.public_len()
        ));
    }
    if signature_bytes.len() != signature_scheme.signature_len() {
        return Err(format!(
            "{} signature must be {} bytes",
            signature_scheme.tag(),
            signature_scheme.signature_len()
        ));
    }
    match public_scheme {
        SchemeId::Ed25519 => verify_ed25519_signature(
            &hex::encode(&public_bytes),
            "public key",
            root,
            &hex::encode(&signature_bytes),
            "signature",
        ),
        SchemeId::MlDsa65 => mldsa_verify(&public_bytes, root.as_bytes(), &signature_bytes),
        SchemeId::HybridEd25519MlDsa65 => {
            let (ed_public, ml_public) = public_bytes.split_at(ED25519_PUBLIC_LEN);
            let (ed_sig, ml_sig) = signature_bytes.split_at(ED25519_SIGNATURE_LEN);
            verify_ed25519_signature(
                &hex::encode(ed_public),
                "hybrid ed25519 public key",
                root,
                &hex::encode(ed_sig),
                "hybrid ed25519 signature",
            )?;
            mldsa_verify(ml_public, root.as_bytes(), ml_sig)
        }
    }
}

fn mldsa_signing_key_from_seed(seed: &[u8]) -> Result<SigningKey<MlDsa65>, String> {
    let seed = B32::try_from(seed)
        .map_err(|_| format!("ML-DSA-65 seed must be {MLDSA65_SEED_LEN} bytes"))?;
    Ok(SigningKey::<MlDsa65>::from_seed(&seed))
}

fn mldsa_public_from_seed(seed: &[u8]) -> Result<Vec<u8>, String> {
    let signing_key = mldsa_signing_key_from_seed(seed)?;
    Ok(signing_key.verifying_key().encode().to_vec())
}

fn mldsa_sign(seed: &[u8], msg: &[u8]) -> Result<Vec<u8>, String> {
    let signing_key = mldsa_signing_key_from_seed(seed)?;
    let signature = signing_key
        .try_sign(msg)
        .map_err(|error| format!("ML-DSA-65 signing failed: {error}"))?;
    Ok(signature.encode().to_vec())
}

fn mldsa_verifying_key_from_bytes(bytes: &[u8]) -> Result<VerifyingKey<MlDsa65>, String> {
    let encoded = EncodedVerifyingKey::<MlDsa65>::try_from(bytes)
        .map_err(|_| format!("ML-DSA-65 public key must be {MLDSA65_PUBLIC_LEN} bytes"))?;
    Ok(VerifyingKey::<MlDsa65>::decode(&encoded))
}

fn mldsa_verify(public_bytes: &[u8], msg: &[u8], signature_bytes: &[u8]) -> Result<(), String> {
    let verifying_key = mldsa_verifying_key_from_bytes(public_bytes)?;
    let signature = Signature::<MlDsa65>::try_from(signature_bytes)
        .map_err(|_| "ML-DSA-65 signature is malformed or the wrong length".to_string())?;
    verifying_key
        .verify(msg, &signature)
        .map_err(|_| "ML-DSA-65 signature verification failed".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const ED_SEED: &str = "0707070707070707070707070707070707070707070707070707070707070707";
    const ML_SEED: &str = "0909090909090909090909090909090909090909090909090909090909090909";
    const ROOT: &str = "1111111111111111111111111111111111111111111111111111111111111111";

    fn hybrid_seed_hex() -> String {
        format!("{ED_SEED}{ML_SEED}")
    }

    fn secret_for(scheme: SchemeId) -> String {
        let seed = match scheme {
            SchemeId::Ed25519 => ED_SEED.to_string(),
            SchemeId::MlDsa65 => ML_SEED.to_string(),
            SchemeId::HybridEd25519MlDsa65 => hybrid_seed_hex(),
        };
        scheme_secret_from_seed(scheme, &seed).unwrap()
    }

    fn value_hex(value: &str) -> &str {
        value.split_once(':').map(|(_, hex)| hex).unwrap_or(value)
    }

    const ALL: [SchemeId; 3] = [
        SchemeId::Ed25519,
        SchemeId::MlDsa65,
        SchemeId::HybridEd25519MlDsa65,
    ];

    #[test]
    fn sign_verify_roundtrips_for_every_scheme() {
        for scheme in ALL {
            let secret = secret_for(scheme);
            let public = scheme_derive_public(&secret).unwrap();
            match scheme {
                SchemeId::Ed25519 => assert!(!public.contains(':')),
                _ => assert!(public.starts_with(scheme.tag())),
            }
            let signature = scheme_sign_root(&secret, ROOT).unwrap();
            scheme_verify_root(&public, ROOT, &signature)
                .unwrap_or_else(|e| panic!("{} verify failed: {e}", scheme.tag()));
        }
    }

    #[test]
    fn signatures_are_deterministic() {
        for scheme in ALL {
            let secret = secret_for(scheme);
            let a = scheme_sign_root(&secret, ROOT).unwrap();
            let b = scheme_sign_root(&secret, ROOT).unwrap();
            assert_eq!(a, b, "{} signing is not deterministic", scheme.tag());
        }
    }

    #[test]
    fn encoded_lengths_match_scheme_sizes() {
        for scheme in ALL {
            let secret = secret_for(scheme);
            let public = scheme_derive_public(&secret).unwrap();
            let signature = scheme_sign_root(&secret, ROOT).unwrap();
            assert_eq!(value_hex(&public).len(), scheme.public_len() * 2);
            assert_eq!(value_hex(&signature).len(), scheme.signature_len() * 2);
        }
    }

    #[test]
    fn tampered_signature_is_rejected_for_every_scheme() {
        for scheme in ALL {
            let secret = secret_for(scheme);
            let public = scheme_derive_public(&secret).unwrap();
            let mut signature = scheme_sign_root(&secret, ROOT).unwrap();
            let idx = signature.find(':').map(|colon| colon + 1).unwrap_or(0);
            let first = signature.as_bytes()[idx];
            let replacement = if first == b'0' { '1' } else { '0' };
            signature.replace_range(idx..idx + 1, &replacement.to_string());
            assert!(
                scheme_verify_root(&public, ROOT, &signature).is_err(),
                "{} accepted a tampered signature",
                scheme.tag()
            );
        }
    }

    #[test]
    fn wrong_root_is_rejected_for_every_scheme() {
        let other_root = "2222222222222222222222222222222222222222222222222222222222222222";
        for scheme in ALL {
            let secret = secret_for(scheme);
            let public = scheme_derive_public(&secret).unwrap();
            let signature = scheme_sign_root(&secret, ROOT).unwrap();
            assert!(
                scheme_verify_root(&public, other_root, &signature).is_err(),
                "{} accepted a signature over the wrong root",
                scheme.tag()
            );
        }
    }

    #[test]
    fn hybrid_requires_both_halves() {
        let secret = secret_for(SchemeId::HybridEd25519MlDsa65);
        let public = scheme_derive_public(&secret).unwrap();
        let signature = scheme_sign_root(&secret, ROOT).unwrap();
        let hex_part = signature.split_once(':').unwrap().1.to_string();
        let tag = SchemeId::HybridEd25519MlDsa65.tag();

        let mut ed_corrupt = hex_part.clone();
        flip_hex(&mut ed_corrupt, 0);
        let ed_sig = format!("{tag}:{ed_corrupt}");
        assert!(
            scheme_verify_root(&public, ROOT, &ed_sig).is_err(),
            "hybrid accepted a signature with a broken Ed25519 half"
        );

        let mut ml_corrupt = hex_part;
        flip_hex(&mut ml_corrupt, ED25519_SIGNATURE_LEN * 2);
        let ml_sig = format!("{tag}:{ml_corrupt}");
        assert!(
            scheme_verify_root(&public, ROOT, &ml_sig).is_err(),
            "hybrid accepted a signature with a broken ML-DSA half"
        );
    }

    #[test]
    fn scheme_mismatch_between_key_and_signature_is_rejected() {
        let ed_secret = secret_for(SchemeId::Ed25519);
        let ed_public = scheme_derive_public(&ed_secret).unwrap();
        let ml_secret = secret_for(SchemeId::MlDsa65);
        let ml_signature = scheme_sign_root(&ml_secret, ROOT).unwrap();
        assert!(scheme_verify_root(&ed_public, ROOT, &ml_signature).is_err());
    }

    #[test]
    fn legacy_untagged_ed25519_still_verifies() {
        let public_hex = public_key_hex_for_secret(ED_SEED, "secret key").unwrap();
        let signature_hex = sign_ed25519_root(ED_SEED, "secret key", ROOT).unwrap();
        scheme_verify_root(&public_hex, ROOT, &signature_hex).unwrap();
    }

    fn flip_hex(hex_string: &mut String, index: usize) {
        let current = hex_string.as_bytes()[index];
        let replacement = if current == b'0' { '1' } else { '0' };
        hex_string.replace_range(index..index + 1, &replacement.to_string());
    }

    fn lcg(state: &mut u64) -> u64 {
        *state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        *state >> 33
    }

    #[test]
    fn parsers_never_panic_on_arbitrary_input() {
        const POOL: &[u8] = b"0123456789abcdefABCDEF:xyzZ-_. \x00\xff";
        let mut state: u64 = 0x5eed_5eed_5eed_5eed;
        let real_public = scheme_derive_public(&secret_for(SchemeId::Ed25519)).unwrap();
        for _ in 0..20_000 {
            let len = (lcg(&mut state) % 200) as usize;
            let candidate: String = (0..len)
                .map(|_| char::from(POOL[(lcg(&mut state) as usize) % POOL.len()]))
                .collect();
            let _ = validate_scheme_public(&candidate, "fuzz");
            let _ = scheme_tag_for_public(&candidate, "fuzz");
            let _ = scheme_normalize_public(&candidate, "fuzz");
            let _ = scheme_derive_public(&candidate);
            let _ = scheme_sign_root(&candidate, ROOT);
            let _ = scheme_verify_root(&candidate, ROOT, &candidate);
            let _ = scheme_verify_root(&real_public, ROOT, &candidate);
            let _ = scheme_verify_root(&candidate, &candidate, &candidate);
        }
    }

    #[test]
    fn tagged_inputs_with_arbitrary_payload_lengths_never_panic() {
        let mut state: u64 = 0xdead_beef_cafe_f00d;
        let tags = [
            "ed25519",
            "mldsa65",
            "hybrid-ed25519-mldsa65",
            "unknown-scheme",
            "",
        ];
        let interesting_lens = [
            0usize,
            1,
            31,
            32,
            33,
            63,
            64,
            65,
            ED25519_PUBLIC_LEN,
            ED25519_SIGNATURE_LEN,
            MLDSA65_PUBLIC_LEN - 1,
            MLDSA65_PUBLIC_LEN,
            MLDSA65_PUBLIC_LEN + 1,
            MLDSA65_SIGNATURE_LEN,
            ED25519_PUBLIC_LEN + MLDSA65_PUBLIC_LEN,
            ED25519_SIGNATURE_LEN + MLDSA65_SIGNATURE_LEN,
        ];
        for _ in 0..600 {
            let tag = tags[(lcg(&mut state) as usize) % tags.len()];
            let byte_len = interesting_lens[(lcg(&mut state) as usize) % interesting_lens.len()];
            let payload: String = (0..byte_len * 2)
                .map(|_| char::from(b"0123456789abcdef"[(lcg(&mut state) as usize) % 16]))
                .collect();
            let candidate = if tag.is_empty() {
                payload
            } else {
                format!("{tag}:{payload}")
            };
            let _ = validate_scheme_public(&candidate, "fuzz");
            let _ = scheme_derive_public(&candidate);
            let _ = scheme_sign_root(&candidate, ROOT);
            let _ = scheme_verify_root(&candidate, ROOT, &candidate);
        }
    }

    #[test]
    fn random_seeds_round_trip_and_reject_cross_key_verification() {
        let mut state: u64 = 0x0123_4567_89ab_cdef;
        let mut random_hex = |bytes: usize| -> String {
            (0..bytes * 2)
                .map(|_| char::from(b"0123456789abcdef"[(lcg(&mut state) as usize) % 16]))
                .collect()
        };
        for _ in 0..6 {
            let root = random_hex(32);
            for scheme in ALL {
                let seed_a = random_hex(scheme.seed_len());
                let seed_b = random_hex(scheme.seed_len());
                let secret_a = scheme_secret_from_seed(scheme, &seed_a).unwrap();
                let secret_b = scheme_secret_from_seed(scheme, &seed_b).unwrap();
                let public_a = scheme_derive_public(&secret_a).unwrap();
                let public_b = scheme_derive_public(&secret_b).unwrap();
                let signature = scheme_sign_root(&secret_a, &root).unwrap();
                scheme_verify_root(&public_a, &root, &signature).unwrap_or_else(|e| {
                    panic!("{} random-seed round trip failed: {e}", scheme.tag())
                });
                assert!(
                    scheme_verify_root(&public_b, &root, &signature).is_err(),
                    "{} signature verified under a different random key",
                    scheme.tag()
                );
            }
        }
    }
}
