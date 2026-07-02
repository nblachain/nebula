use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};

pub mod scheme;
pub use scheme::{
    scheme_derive_public, scheme_normalize_public, scheme_secret_from_seed, scheme_sign_root,
    scheme_tag_for_public, scheme_verify_root, validate_scheme_public, SchemeId,
};

fn require_hex_len(value: &str, name: &str, hex_len: usize) -> Result<(), String> {
    if value.len() != hex_len || !value.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(format!("{name} must be a {hex_len}-character hex value"));
    }
    Ok(())
}

fn decode_hex_bytes(value: &str, name: &str, bytes_len: usize) -> Result<Vec<u8>, String> {
    require_hex_len(value, name, bytes_len * 2)?;
    hex::decode(value).map_err(|error| format!("{name} is not valid hex: {error}"))
}

pub fn verifying_key_from_hex(public_key_hex: &str, name: &str) -> Result<VerifyingKey, String> {
    let bytes = decode_hex_bytes(public_key_hex, name, 32)?;
    let bytes: [u8; 32] = bytes
        .as_slice()
        .try_into()
        .map_err(|_| format!("{name} must decode to 32 bytes"))?;
    let verifying_key = VerifyingKey::from_bytes(&bytes)
        .map_err(|error| format!("{name} is not an Ed25519 key: {error}"))?;
    if verifying_key.is_weak() {
        return Err(format!(
            "{name} is a small-order (weak) Ed25519 key and is rejected"
        ));
    }
    Ok(verifying_key)
}

pub fn signing_key_from_hex(secret_key_hex: &str, name: &str) -> Result<SigningKey, String> {
    let bytes = decode_hex_bytes(secret_key_hex, name, 32)?;
    let bytes: [u8; 32] = bytes
        .as_slice()
        .try_into()
        .map_err(|_| format!("{name} must decode to 32 bytes"))?;
    Ok(SigningKey::from_bytes(&bytes))
}

fn signature_from_hex(signature_hex: &str, name: &str) -> Result<Signature, String> {
    let bytes = decode_hex_bytes(signature_hex, name, 64)?;
    let bytes: [u8; 64] = bytes
        .as_slice()
        .try_into()
        .map_err(|_| format!("{name} must decode to 64 bytes"))?;
    Ok(Signature::from_bytes(&bytes))
}

pub fn public_key_hex_for_secret(secret_key_hex: &str, name: &str) -> Result<String, String> {
    let signing_key = signing_key_from_hex(secret_key_hex, name)?;
    Ok(hex::encode(signing_key.verifying_key().to_bytes()))
}

pub fn sign_ed25519_root(secret_key_hex: &str, name: &str, root: &str) -> Result<String, String> {
    require_hex_len(root, "signing_root", 64)?;
    let signing_key = signing_key_from_hex(secret_key_hex, name)?;
    let signature: Signature = signing_key.sign(root.as_bytes());
    Ok(hex::encode(signature.to_bytes()))
}

pub fn verify_ed25519_signature(
    public_key_hex: &str,
    public_key_name: &str,
    signing_root: &str,
    signature_hex: &str,
    signature_name: &str,
) -> Result<(), String> {
    let verifying_key = verifying_key_from_hex(public_key_hex, public_key_name)?;
    require_hex_len(signing_root, "signing_root", 64)?;
    let signature = signature_from_hex(signature_hex, signature_name)?;
    verifying_key
        .verify_strict(signing_root.as_bytes(), &signature)
        .map_err(|error| format!("{signature_name} Ed25519 verification failed: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECRET_HEX: &str = "0707070707070707070707070707070707070707070707070707070707070707";
    const ROOT_HEX: &str = "1111111111111111111111111111111111111111111111111111111111111111";

    fn public_hex() -> String {
        public_key_hex_for_secret(SECRET_HEX, "secret_key_hex").unwrap()
    }

    #[test]
    fn sign_then_verify_roundtrips() {
        let public = public_hex();
        let signature = sign_ed25519_root(SECRET_HEX, "secret_key_hex", ROOT_HEX).unwrap();
        assert_eq!(signature.len(), 128);
        verify_ed25519_signature(&public, "public_key", ROOT_HEX, &signature, "signature").unwrap();
    }

    #[test]
    fn tampered_signature_is_rejected() {
        let public = public_hex();
        let mut signature = sign_ed25519_root(SECRET_HEX, "secret_key_hex", ROOT_HEX).unwrap();
        let first = if signature.starts_with('0') { '1' } else { '0' };
        signature.replace_range(0..1, &first.to_string());
        assert!(
            verify_ed25519_signature(&public, "public_key", ROOT_HEX, &signature, "signature")
                .is_err()
        );
    }

    #[test]
    fn wrong_root_is_rejected() {
        let public = public_hex();
        let signature = sign_ed25519_root(SECRET_HEX, "secret_key_hex", ROOT_HEX).unwrap();
        let other_root = "2222222222222222222222222222222222222222222222222222222222222222";
        assert!(verify_ed25519_signature(
            &public,
            "public_key",
            other_root,
            &signature,
            "signature"
        )
        .is_err());
    }

    #[test]
    fn wrong_key_is_rejected() {
        let other_secret = "0808080808080808080808080808080808080808080808080808080808080808";
        let other_public = public_key_hex_for_secret(other_secret, "secret_key_hex").unwrap();
        let signature = sign_ed25519_root(SECRET_HEX, "secret_key_hex", ROOT_HEX).unwrap();
        assert!(verify_ed25519_signature(
            &other_public,
            "public_key",
            ROOT_HEX,
            &signature,
            "signature"
        )
        .is_err());
    }

    #[test]
    fn malformed_inputs_are_rejected() {
        let public = public_hex();
        let signature = sign_ed25519_root(SECRET_HEX, "secret_key_hex", ROOT_HEX).unwrap();
        assert!(verifying_key_from_hex("zz", "public_key").is_err());
        assert!(
            verify_ed25519_signature(&public, "public_key", "1111", &signature, "signature")
                .is_err()
        );
        assert!(
            verify_ed25519_signature(&public, "public_key", ROOT_HEX, "abcd", "signature").is_err()
        );
        assert!(sign_ed25519_root(SECRET_HEX, "secret_key_hex", "abcd").is_err());
    }
}
