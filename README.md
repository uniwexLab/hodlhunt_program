# HodlHunt Solana Program

Minimal source bundle for public verification of the on-chain program.

## Program
- Program ID: `B1osUCap5eJ2iJnbRqfCQB87orhJM5EqZqPcGMbjJvXz`
- Repository: `https://github.com/uniwexLab/hodlhunt_program`

## Toolchain
- Anchor: **0.32.1** (must use this exact version)
- Solana program crate: 2.3.0
- Rust edition: 2021

## Build Instructions

**IMPORTANT:** To build the contract for verification, you must use Docker with the `solana-verify` tool and the exact Anchor version.

### Using Docker (Recommended)

Build the contract using Docker container with Anchor 0.32.1:

```bash
solana-verify build \
  --base-image solanafoundation/anchor:v0.32.1 \
  "$@"
```

This will create a reproducible build in a Docker container, ensuring consistent binary output regardless of your local environment.

### Alternative: Local Build

If you prefer to build locally, ensure you have Anchor 0.32.1 installed:

```bash
# Verify that Anchor 0.32.1 is being used
anchor --version  # should output 0.32.1

# Build with verification flag
anchor build --verifiable
```

The `--verifiable` flag creates a binary file ready for on-chain verification.

## Verification Instructions

To verify the on-chain program matches the source code, use `solana-verify verify-from-repo` with Docker:

```bash
solana-verify verify-from-repo \
  --remote \
  --base-image solanafoundation/anchor:v0.32.1 \
  --url "https://mainnet.helius-rpc.com/?api-key=YOUR_API_KEY" \
  --program-id B1osUCap5eJ2iJnbRqfCQB87orhJM5EqZqPcGMbjJvXz \
  --library-name hodlhunt \
  https://github.com/uniwexLab/hodlhunt_program
```

**Note:** Replace `YOUR_API_KEY` with your own Helius API key. You can get a free API key at [helius.dev](https://helius.dev).

### Verification Parameters

- `--remote`: Use remote Docker execution
- `--base-image solanafoundation/anchor:v0.32.1`: Use exact Anchor version for reproducible builds
- `--url`: RPC endpoint (use your own API key)
- `--program-id`: On-chain program ID
- `--library-name`: Library name from Cargo.toml (`hodlhunt`)
- Repository URL: GitHub repository with source code

## Project Layout

```
Anchor.toml
Cargo.lock
programs/hodlhunt/Cargo.toml
programs/hodlhunt/src/**   (all source files)
```
