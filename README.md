# Acuity Index Kreivo

A blockchain event indexer for the [Kreivo](https://kreivo.io) parachain, built on
top of [acuity-index-substrate](https://crates.io/crates/acuity-index-substrate).
It connects to a Kreivo node via WebSocket, indexes on-chain events into a local
[sled](https://github.com/spacejam/sled) database, and exposes a WebSocket API for
querying indexed events.

Full documentation is available at [https://index.acuity.network/](https://index.acuity.network/)

---

## Features

- Indexes events from both **Kreivo Mainnet** and **Kreivo Testnet**
- Supports the following pallets:
  - System
  - Balances
  - TransactionPayment
  - Session
  - Treasury
  - Vesting
  - Proxy
  - Multisig
  - Preimage
- Chain-specific indexing keys:
  - `AuctionIndex`
  - `CandidateHash`
  - `ParaId`
- Configurable database mode, cache capacity, and concurrency
- Optional indexing of event variants
- Optional in-database event storage for immediate retrieval
- WebSocket API for querying indexed events

---

## Requirements

- Rust (nightly) — see [`rust-toolchain.toml`](./rust-toolchain.toml)
- Access to a Kreivo node (local or remote)

---

## Building

```bash
cargo build --release
```

---

## Usage

```bash
acuity-index-kreivo [OPTIONS]
```

### Options

| Flag | Description | Default |
|------|-------------|---------|
| `-c, --chain <CHAIN>` | Chain to index (`mainnet` or `testnet`) | `mainnet` |
| `-d, --db-path <PATH>` | Path to the sled database directory | Auto |
| `--db-mode <MODE>` | Database mode (`low-space` or `high-throughput`) | `low-space` |
| `--db-cache-capacity <SIZE>` | Maximum system page cache size | `1024.00 MiB` |
| `-u, --url <URL>` | WebSocket URL of the Substrate node | Chain default |
| `--queue-depth <N>` | Maximum number of concurrent chain requests | `1` |
| `-f, --finalized` | Only index finalized blocks | `false` |
| `-i, --index-variant` | Index event variants | `false` |
| `-s, --store-events` | Store events in the index for immediate retrieval | `false` |
| `-p, --port <PORT>` | Port for the WebSocket query server | `8172` |
| `-v, --verbose` | Increase logging verbosity | Info |

### Default Node URLs

| Chain | URL |
|-------|-----|
| Mainnet | `wss://kreivo.io:443` |
| Testnet | `wss://testnet.kreivo.io:443` |

### Examples

Index mainnet using the default node:

```bash
acuity-index-kreivo --chain mainnet
```

Index testnet with a custom node URL and higher concurrency:

```bash
acuity-index-kreivo \
  --chain testnet \
  --url wss://testnet.kreivo.io:443 \
  --queue-depth 4
```

Index mainnet with a custom database path and high-throughput mode:

```bash
acuity-index-kreivo \
  --chain mainnet \
  --db-path /var/lib/kreivo-index \
  --db-mode high-throughput \
  --db-cache-capacity "4 GiB"
```

---

## Project Structure
