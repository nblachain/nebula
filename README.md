# <img src="https://raw.githubusercontent.com/pompompur1nn/nebula/master/docs/assets/nebula-logo.svg" alt="" align="left" width="40" height="40"> Nebula

Nebula is a privacy-preserving Monero Layer 2.


## Documentation

**[Read the testnet runtime guide](https://github.com/pompompur1nn/nebula/blob/master/crates/nebula-testnet/README.md)**, review the [public testnet readiness runbook](https://github.com/pompompur1nn/nebula/blob/master/docs/PUBLIC_TESTNET_READINESS_RUNBOOK.md), or inspect the [Nebula CI checks](https://github.com/pompompur1nn/nebula/actions/workflows/nebula.yml).

## What is Nebula?

to do

## Why Nebula?

- **Hybrid gas trial:** `NBLA` is the native gas and validator-accounting token, while bridged Monero appears as `nXMR` for gas paths that exercise buyback and reward accounting.
- adding more

## Quick Start

Run the local readiness contract:

```bash
cargo run --manifest-path crates/nebula-testnet/Cargo.toml --bin nebula-testnet -- --mainnet-readiness --json
```

Prove the local launch artifact chain:

```bash
cargo run --manifest-path crates/nebula-testnet/Cargo.toml --bin nebula-testnet -- --prove-local-public-testnet --json
```

Prove the stronger loopback RPC devnet:

```bash
cargo run --manifest-path crates/nebula-testnet/Cargo.toml --bin nebula-testnet -- --prove-live-rpc-devnet --json
```

Run the test suite:

```bash
cargo test --manifest-path crates/nebula-testnet/Cargo.toml -- --test-threads=1
```

## Public Testnet Status

The expected local readiness shape is:

```json
{
  "local_testnet_ready": true,
  "public_launch_ready": false,
  "public_launch_level": "public-launch-blocked",
  "blocking_gaps": ["public-launch-deployment-attestation"]
}
```

That state is intentional for an operator workspace that has not supplied live public deployment evidence. See the [runtime guide](https://github.com/pompompur1nn/nebula/blob/master/crates/nebula-testnet/README.md) for the full public-testnet architecture and launch sequence.

## License

This project is licensed under the Nebula Source License. See [LICENSE](https://github.com/pompompur1nn/nebula/blob/master/LICENSE) for details.

Copyright (c) 2026 Nebula contributors.

---

[![Nebula CI](https://github.com/pompompur1nn/nebula/actions/workflows/nebula.yml/badge.svg)](https://github.com/pompompur1nn/nebula/actions/workflows/nebula.yml)
