# Nebula Security Model

**Status: experimental testnet. Not audited. No mainnet. No real economic value.**
The Monero bridge runs with live value **disabled** by design
(`bridge_policy().live_value_enabled == false` in `crates/nebula-testnet/src/runtime.rs`),
and enabling it is itself a launch-readiness blocking gap
(`bridge-live-value-enabled` in `blocking_gaps`).

This document describes Nebula's **actual, code-derived** security posture — what is
cryptographically verified, what is merely trusted, the known limitations, and the scope
an external security audit must cover. It is written to be honest about weaknesses rather
than to market strengths. See [THREAT_MODEL.md](THREAT_MODEL.md) for the asset/adversary
analysis.

## Trust model at a glance

| Component | Trust level | Why |
|---|---|---|
| **Sequencer** | **Fully trusted** | A single node produces every block (`try_produce_block`), executes transactions, and signs the resulting `state_root` (`sign_block_hash`). It has unilateral authority over state, ordering, censorship, and liveness. |
| **Followers** | Verify signatures + hash-chain, **plus re-execute the replayable columns** | `validate_snapshot` verifies block hash-chaining, `tx_root`, `block_hash`, `snapshot.root`, and per-block signatures (`verify_block_signature`). It additionally **re-executes** the nXMR and validator-point columns (`reexecute_replayable_state`) so a compromised sequencer key cannot silently reassign nXMR or mint validator points. **NBLA balances, account nonces, and the shielded-note set are still sequencer-attested, not independently re-derived** (see Limitations). |
| **Monero bridge** | **Partially trusted M-of-N multisig** | Deposits credit nXMR after an M-of-N observer-identity quorum (`observe_bridge_deposit`); withdrawals require an M-of-N launch-attested operator quorum (`finalize_withdrawal`). The quorum can, by collusion, mint unbacked nXMR or approve a fraudulent withdrawal. This is a disclosed, by-design trust assumption. |
| **Live Monero node verification** | Opt-in, single node | When configured (`--monero-wallet-rpc-url`/`--monero-daemon-rpc-url`/`--monero-custody-address`), each observer verifies deposits/withdrawals against a Monero node via view-key proofs (`verify_deposit_via`, `check_tx_key`) over pinned TLS. This is **off by default** and, when on, trusts one operator-chosen node/wallet. |

## What is cryptographically verified

- **Block, account, and withdrawal signatures** — Ed25519 or hybrid Ed25519+ML-DSA-65
  (`verify_block_signature`, `verify_account_signature`, `scheme_verify_root`; hybrid
  requires **both** halves to verify).
- **Confidential amounts** — Pedersen commitments + Bulletproofs range proofs over
  `[0, 2^64)` and a homomorphic balance check (`nebula_privacy::verify_amount`,
  `amounts_balance`) so shielded transfers conserve value and cannot inflate, without
  revealing amounts.
- **Monero address validity** — base58 + Keccak-256 checksum + network/kind prefix
  (`nebula_monero::parse_address`).
- **Bridge deposit/withdrawal facts, when the live verifier is configured** — view-key
  `check_tx_key` amount proof, confirmation depth, custody address, and an experimental
  `tx_extra` account binding (`nebula_monero::verify::verify_deposit`), plus TLS transport
  with optional SHA-256 leaf-certificate pinning (`HttpMoneroRpc` / `PinnedServerCertVerifier`).
- **Replay resistance** — Monero tx ids and finalization-proof roots are unique across
  deposits and finalized withdrawals; accounts carry nonces.
- **Re-executed consensus columns** — followers recompute per-account nXMR and total
  validator points from the block journal and reject a snapshot whose signed state
  disagrees (`reexecute_replayable_state`).
- **Post-quantum attestation roots (opt-in)** — the off-chain launch-evidence layer
  verifies via the same scheme API and accepts hybrid signatures.

## What is trusted but NOT verified

- **The sequencer-asserted NBLA state, nonces, and shielded-note set.** There is no fraud
  proof, validity proof, or full independent re-execution of these columns (shielding hides
  the debited amount inside a Pedersen commitment, and faucet/shield/unshield advance state
  with no in-block journal). A compromised sequencer key can forge these.
- **Bridge quorum honesty.** M colluding observers can mint unbacked nXMR; M colluding
  operators can approve a fraudulent withdrawal.
- **Custody solvency.** `verify_on_chain_custody` compares outstanding nXMR against **one**
  operator-chosen custody wallet's balance; it is not a decentralized proof of reserves.
- **Shielded transaction graph.** Amounts are hidden; sender/recipient links, the input
  commitments a transfer spends, and shield/unshield endpoints are **public**. Notes are
  **bearer** instruments (knowledge of the opening authorizes spend; `shielded_transfer`
  and `unshield` carry no owner signature).
- **The default signature scheme is classical.** Ed25519 is the default; hybrid PQC is
  opt-in and covers block/account/attestation signing only — not Pedersen/Bulletproofs.

## Known limitations

- **Single sequencer** — a single point of failure for liveness, ordering, and censorship;
  no BFT, leader election, or forced-inclusion.
- **Public shielded graph & bearer notes** — see above; only amounts are private.
- **Opt-in, classical-default PQC** — `ml-dsa` 0.1.1 is a pre-1.0, unaudited crate, and the
  hybrid concat-encoding in `scheme.rs` is bespoke (not an IETF/NIST composite standard).
- **Partially-trusted bridge** — a fully trustless Monero bridge is not possible without
  on-chain Monero light-client verification (infeasible: RandomX) or Monero-level covenants;
  the honest target is atomic swaps or a well-bonded, diverse multisig.
- **No mainnet, no economic value** — testnet rewards are non-transferable validator points.

## External audit scope

An external cryptographic/security review must cover, at minimum:

1. **`ml-dsa` 0.1.1** — correctness, side-channel resistance, encoding stability, and the
   bespoke hybrid concat construction in `crates/nebula-crypto/src/scheme.rs`
   (`scheme_verify_root` both-halves-required, tag parsing, downgrade/mismatch resistance).
2. **Bulletproofs integration** (`crates/nebula-privacy`) — the transcript domain-separation
   label, the single 64-bit range, blinding-factor RNG (`getrandom`), and `amounts_balance`
   soundness including the zero-fee path.
3. **Bridge custody** — deposit view-key + `tx_extra` binding (`verify.rs`), the withdrawal
   payout proof (`finalize_withdrawal` live path), custody reconciliation, replay/nonce
   logic, and the `u64`-vs-`u128` amount parsing boundary in `client.rs`
   (`check_tx_key` / `custody_unlocked_balance` read atomic amounts as `u64`).
4. **Consensus/state** — snapshot signature + hash-chain verification, the scope and
   soundness of `reexecute_replayable_state` (and precisely which columns remain
   sequencer-attested), key rotation (`sequencer_public_key_for_height`), the
   equivocation/accountability fail-closed path (`ensure_accountability_clean`), block
   co-signing (`verify_block_cosigner_quorum`), and root canonicalization
   (`stable_runtime_root` / `serde_json` key-ordering assumptions).
5. **Bridge trust-minimization primitives** — operator bonding/slashing and the withdrawal
   challenge window (`post_bridge_bond`, `slash_bridge_participant`, the `finalizing` →
   `settled`/`reverted` withdrawal lifecycle) and their custody-accounting invariants.

## Reporting

This is experimental testnet software with no bug-bounty program yet. Security issues can be
raised via the project's GitHub repository. Do not deploy Nebula with real value.
