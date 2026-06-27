use serde_json::{json, Value};
use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_nebula-testnet")
}

fn temp_dir(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("nebula-{name}-{nonce}"));
    fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn run_nebula(args: Vec<String>) -> String {
    let output = Command::new(binary())
        .args(args)
        .output()
        .expect("run nebula-testnet");

    assert!(
        output.status.success(),
        "command failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).expect("stdout should be utf8")
}

fn secret(byte: u8) -> String {
    format!("{byte:02x}").repeat(32)
}

#[test]
fn bridge_evidence_commands_emit_rpc_ready_json() {
    let dir = temp_dir("bridge-cli");
    let deposit_path = dir.join("deposit.json");
    let obs_a_path = dir.join("obs-a.json");
    let obs_b_path = dir.join("obs-b.json");
    let withdrawal_path = dir.join("withdrawal.json");
    let op_a_path = dir.join("op-a.json");
    let op_b_path = dir.join("op-b.json");

    let deposit = json!({
        "monero_tx_id": "a".repeat(64),
        "account": "alice.testnet",
        "amount_nxmr_units": 42_000_000_000_u64,
        "confirmations": 20_u64,
        "observer_id": "observer-a",
        "observer_ids": ["observer-a", "observer-b"],
        "proof_root": "b".repeat(64),
        "custody_proof_root": "c".repeat(64),
        "relayer_set_root": "d".repeat(64),
        "observer_signature_roots": [],
        "observer_evidence": [],
        "observed_at_unix_ms": 1_735_689_600_000_u64
    });
    fs::write(
        &deposit_path,
        serde_json::to_string_pretty(&deposit).expect("serialize deposit"),
    )
    .expect("write deposit");

    let obs_a = run_nebula(vec![
        "--sign-bridge-observer-evidence".into(),
        "--bridge-deposit".into(),
        deposit_path.display().to_string(),
        "--observer-id".into(),
        "observer-a".into(),
        "--observer-secret-key".into(),
        secret(0x11),
    ]);
    fs::write(&obs_a_path, obs_a).expect("write observer a evidence");

    let obs_b = run_nebula(vec![
        "--sign-bridge-observer-evidence".into(),
        "--bridge-deposit".into(),
        deposit_path.display().to_string(),
        "--observer-id".into(),
        "observer-b".into(),
        "--observer-secret-key".into(),
        secret(0x22),
    ]);
    fs::write(&obs_b_path, obs_b).expect("write observer b evidence");

    let assembled_deposit = run_nebula(vec![
        "--assemble-bridge-deposit".into(),
        "--bridge-deposit".into(),
        deposit_path.display().to_string(),
        "--observer-evidence".into(),
        obs_b_path.display().to_string(),
        "--observer-evidence".into(),
        obs_a_path.display().to_string(),
    ]);
    let assembled_deposit: Value =
        serde_json::from_str(&assembled_deposit).expect("assembled deposit json");
    assert_eq!(
        assembled_deposit["observer_signature_roots"]
            .as_array()
            .expect("signature roots")
            .len(),
        2
    );
    assert_eq!(
        assembled_deposit["observer_evidence"]
            .as_array()
            .expect("observer evidence")
            .len(),
        2
    );
    assert_eq!(
        assembled_deposit["observer_evidence"][0]["observer_id"],
        "observer-a"
    );
    assert_eq!(
        assembled_deposit["observer_evidence"][1]["observer_id"],
        "observer-b"
    );

    let withdrawal = json!({
        "withdrawal_id": "withdrawal-0001",
        "account": "alice.testnet",
        "monero_address": "44AFFq5kSiGBoZ...test-only-address",
        "amount_nxmr_units": 21_000_000_000_u64,
        "nonce": 7_u64,
        "signature": "1".repeat(128),
        "requested_at_unix_ms": 1_735_689_700_000_u64,
        "status": "operator_pending",
        "bridge_policy_root": "2".repeat(64),
        "operator_approval_ids": [],
        "operator_approval_roots": [],
        "operator_approvals": [],
        "finalized_monero_tx_id": null,
        "finalization_proof_root": null,
        "finalized_at_unix_ms": null,
        "root": ""
    });
    fs::write(
        &withdrawal_path,
        serde_json::to_string_pretty(&withdrawal).expect("serialize withdrawal"),
    )
    .expect("write withdrawal");

    let finalized_tx = "e".repeat(64);
    let finalization_root = "f".repeat(64);
    let op_a = run_nebula(vec![
        "--sign-withdrawal-operator-approval".into(),
        "--withdrawal".into(),
        withdrawal_path.display().to_string(),
        "--finalized-monero-tx-id".into(),
        finalized_tx.clone(),
        "--finalization-proof-root".into(),
        finalization_root.clone(),
        "--operator-id".into(),
        "operator-a".into(),
        "--operator-secret-key".into(),
        secret(0x33),
    ]);
    fs::write(&op_a_path, op_a).expect("write operator a approval");

    let op_b = run_nebula(vec![
        "--sign-withdrawal-operator-approval".into(),
        "--withdrawal".into(),
        withdrawal_path.display().to_string(),
        "--finalized-monero-tx-id".into(),
        finalized_tx.clone(),
        "--finalization-proof-root".into(),
        finalization_root.clone(),
        "--operator-id".into(),
        "operator-b".into(),
        "--operator-secret-key".into(),
        secret(0x44),
    ]);
    fs::write(&op_b_path, op_b).expect("write operator b approval");

    let finalized = run_nebula(vec![
        "--assemble-finalize-withdrawal".into(),
        "--withdrawal".into(),
        withdrawal_path.display().to_string(),
        "--finalized-monero-tx-id".into(),
        finalized_tx,
        "--finalization-proof-root".into(),
        finalization_root,
        "--operator-approval".into(),
        op_a_path.display().to_string(),
        "--operator-approval".into(),
        op_b_path.display().to_string(),
        "--admin-token".into(),
        "launch-admin".into(),
    ]);
    let finalized: Value = serde_json::from_str(&finalized).expect("finalized withdrawal json");
    assert_eq!(finalized["admin_token"], "launch-admin");
    assert_eq!(
        finalized["operator_approvals"]
            .as_array()
            .expect("operator approvals")
            .len(),
        2
    );
    assert_eq!(
        finalized["operator_approval_roots"]
            .as_array()
            .expect("operator approval roots")
            .len(),
        2
    );
}
