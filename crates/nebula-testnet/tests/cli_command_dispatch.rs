use std::process::Command;

fn run(args: &[&str]) -> (String, bool) {
    let output = Command::new(env!("CARGO_BIN_EXE_nebula-testnet"))
        .args(args)
        .output()
        .expect("spawn nebula-testnet binary");
    let mut combined = String::from_utf8_lossy(&output.stdout).into_owned();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    (combined, output.status.success())
}

const SECRET_KEY: &str = "0707070707070707070707070707070707070707070707070707070707070707";
const BLINDING: &str = "0101010101010101010101010101010101010101010101010101010101010101";

#[test]
fn generate_account_dispatches_with_hybrid_scheme() {
    let (out, ok) = run(&["--generate-account", "--scheme", "hybrid", "--json"]);
    assert!(ok, "generate-account should exit 0; got: {out}");
    assert!(
        out.contains("\"key_scheme\": \"hybrid-ed25519-mldsa65\""),
        "expected hybrid key_scheme in output; got: {out}"
    );
    assert!(
        out.contains("\"public_key\""),
        "expected public_key; got: {out}"
    );
    assert!(
        out.contains("\"secret_key\""),
        "expected secret_key; got: {out}"
    );
}

#[test]
fn build_shield_dispatches_and_signs() {
    let (out, ok) = run(&[
        "--build-shield",
        "--secret-key",
        SECRET_KEY,
        "--amount",
        "5",
        "--nonce",
        "0",
        "--blinding",
        BLINDING,
        "--json",
    ]);
    assert!(ok, "build-shield should exit 0; got: {out}");
    assert!(
        out.contains("\"signature\""),
        "expected a shield signature; got: {out}"
    );
    assert!(
        out.contains("\"commitment\""),
        "expected a Pedersen commitment; got: {out}"
    );
    assert!(
        out.contains("\"account\""),
        "expected the derived account; got: {out}"
    );
}

#[test]
fn build_shielded_transfer_dispatches_and_proves() {
    let (out, ok) = run(&[
        "--build-shielded-transfer",
        "--input",
        "abcdef",
        "--output",
        &format!("5:{BLINDING}"),
    ]);
    assert!(ok, "build-shielded-transfer should exit 0; got: {out}");
    assert!(out.contains("\"inputs\""), "expected inputs; got: {out}");
    assert!(
        out.contains("\"range_proof_hex\""),
        "each output should carry a Bulletproofs range proof; got: {out}"
    );
}

#[test]
fn build_unshield_rejects_an_opening_that_does_not_match_the_commitment() {
    let (out, ok) = run(&[
        "--build-unshield",
        "--commitment",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "--account",
        "nbla1deadbeef",
        "--amount",
        "5",
        "--blinding",
        BLINDING,
    ]);
    assert!(!ok, "a mismatched opening should exit non-zero; got: {out}");
    assert!(
        out.contains("do not open"),
        "expected an opening-mismatch error; got: {out}"
    );
}

#[test]
fn verify_monero_deposit_dispatches_and_reports_transport_failure() {
    let (out, ok) = run(&[
        "--verify-monero-deposit",
        "--wallet-rpc-url",
        "http://127.0.0.1:1",
        "--monero-tx-id",
        "aa",
        "--tx-key",
        "bb",
        "--bridge-address",
        "cc",
        "--expected-atomic",
        "1",
    ]);
    assert!(
        !ok,
        "an unreachable node should make verify exit non-zero; got: {out}"
    );
    assert!(
        out.contains("monero rpc error"),
        "expected a transport error; got: {out}"
    );
}
