# Bee-rs

[![](https://img.shields.io/badge/made%20by-Swarm-blue.svg?style=flat-square)](https://swarm.ethereum.org/)
[![crates.io](https://img.shields.io/crates/v/bee-rs.svg)](https://crates.io/crates/bee-rs)
[![docs.rs](https://docs.rs/bee-rs/badge.svg)](https://docs.rs/bee-rs)
[![standard-readme compliant](https://img.shields.io/badge/standard--readme-OK-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)

> Rust SDK for the Swarm decentralised storage.

> Intended to be used with Bee version 2.5.0.

## Quick start

Start a Swarm project using Rust:

```sh
cargo new my-dapp
cd my-dapp
cargo add bee-rs
```

## Install

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
bee-rs = "0.1.0" # Replace with the latest version
```

## Import

```rust
use bee_rs::Bee;
```

## Overview

### Type interfaces

`NumberString` is a branded type for marking strings that represent numbers. It interops with `string` and `bigint`
types. Where `NumberString` is present, `number` is disallowed in order to avoid pitfalls with unsafe large values.

### Byte primitives

All the classes below extend `Bytes`, therefor the following methods are available on all of them: `to_uint8_array`,
`to_hex`, `to_base64`, `to_base32`, `to_utf8`, `to_json`, `static keccak256`, `static from_utf8`.

The `to_string` method uses `to_hex`.

`Bytes` and its subclasses may be constructed with `new` from `Uint8Array` or hex `string`.

#### Elliptic

| Name       | Description               | Methods                                                |
| ---------- | ------------------------- | ------------------------------------------------------ |
| PrivateKey | 32 bytes private key      | `public_key`, `sign`                                    |
| PublicKey  | 64 bytes public key       | `address`, `to_compressed_uint8_array`, `to_compressed_hex` |
| EthAddress | 20 bytes Ethereum address | `to_checksum`                                           |
| Signature  | 65 bytes signature        | `recover_public_key`                                     |

#### Swarm

| Name          | Description                         | Methods                         |
| ------------- | ----------------------------------- | ------------------------------- |
| Reference     | 32/64 bytes reference (chunk, feed) | `to_cid`                         |
| Identifier    | 32 bytes identifier (SOC, Feed)     | -                               |
| TransactionId | 32 bytes transaction ID             | -                               |
| FeedIndex     | 8 bytes feed index (BE)             | `static from_bigint`, `to_bigint` |
| Topic         | 32 bytes topic                      | `static from_string`             |
| PeerAddress   | 32 bytes peer address               | -                               |
| BatchId       | 32 bytes batch ID                   | -                               |
| Span          | 8 bytes span (LE)                   | `static from_bigint`, `to_bigint` |

### Tokens

| Name | Description                 | Methods                                                                                          |
| ---- | --------------------------- | ------------------------------------------------------------------------------------------------ |
| DAI  | ERC20 DAI token (18 digits) | `static from_decimal_string`, `static from_wei`, `to_wei_string`, `to_wei_bigint`, `to_decimal_string`    |
| BZZ  | ERC20 BZZ token (16 digits) | `static from_decimal_string`, `static from_plur`, `to_plur_string`, `to_plur_bigint`, `to_decimal_string` |

### Swarm chunks

| Name             | Description                                                                                     | Creation                    |
| ---------------- | ----------------------------------------------------------------------------------------------- | --------------------------- |
| Chunk            | Span, max. 4096 bytes payload; address dervied from content                                     | `make_content_addressed_chunk` |
| SingleOwnerChunk | Identifier, signature, span, max. 4096 bytes payload; address derived from identifier and owner | `make_single_owner_chunk`      |

### Swarm primitives

| Name         | Description                                          | Methods                                                                                                                                         |
| ------------ | ---------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| MantarayNode | Compact trie with reference values and JSON metadata | `add_fork`, `remove_fork`, `calculate_self_address`, `find`, `find_closest`, `collect`, `marshal`, `unmarshal`, `save_recursively`, `load_recursively` |
| MerkleTree   | Streaming BMT of chunks                              | `append`, `finalize`, `static root`                                                                                                             |

### Swarm objects

| Name       | Description             | Creation             |
| ---------- | ----------------------- | -------------------- |
| SOCWriter  | SingleOwnerChunk writer | `bee.make_soc_writer`  |
| SOCReader  | SingleOwnerChunk reader | `bee.make_soc_reader`  |
| FeedWriter | Feed writer             | `bee.make_feed_writer` |
| FeedReader | Feed reader             | `bee.make_feed_reader` |

### Bee API

> **Note:** This section is a work in progress and may not be accurate for the Rust implementation.

- ❌❌✅ - Full node only
- ❌✅✅ - Light node and full node
- ✅✅✅ - Ultra-light node, light node and full node

| JS Call                              | Bee Endpoint                                                                                                                                             | Bee Mode |
| ------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------- | -------- |
| `upload_file`                         | `POST /bzz` [🔗](https://docs.ethswarm.org/api/#tag/BZZ/paths/~1bzz/post)                                                                                | ❌✅✅   |
| `upload_files_from_directory` _Node.js_ | `POST /bzz` [🔗](https://docs.ethswarm.org/api/#tag/BZZ/paths/~1bzz/post)                                                                                | ❌✅✅   |
| `upload_files`                        | `POST /bzz` [🔗](https://docs.ethswarm.org/api/#tag/BZZ/paths/~1bzz/post)                                                                                | ❌✅✅   |
| `upload_collection`                   | `POST /bzz` [🔗](https://docs.ethswarm.org/api/#tag/BZZ/paths/~1bzz/post)                                                                                | ❌✅✅   |
| `upload_data`                         | `POST /bytes` [🔗](https://docs.ethswarm.org/api/#tag/Bytes/paths/~1bytes/post)                                                                          | ❌✅✅   |
| `upload_chunk`                        | `POST /chunks` [🔗](https://docs.ethswarm.org/api/#tag/Chunk/paths/~1chunks/post)                                                                        | ❌✅✅   |
| `stream_directory` _Node.js_          | `POST /chunks` [🔗](https://docs.ethswarm.org/api/#tag/Chunk/paths/~1chunks/post)                                                                        | ❌✅✅   |
| `stream_files` _Browser_              | `POST /chunks` [🔗](https://docs.ethswarm.org/api/#tag/Chunk/paths/~1chunks/post)                                                                        | ❌✅✅   |
| `soc_writer.upload`                   | `POST /soc/:owner/:identifier` [🔗](https://docs.ethswarm.org/api/#tag/Single-owner-chunk/paths/~1soc~1%7Bowner%7D~1%7Bid%7D/post)                       | ❌✅✅   |
| `feed_reader.download`                | `GET /feeds/:owner/:topic` [🔗](https://docs.ethswarm.org/api/#tag/Feed/paths/~1feeds~1%7Bowner%7D~1%7Btopic%7D/get)                                     | ✅✅✅   |
| `feed_writer.update_feed`              | `POST /soc/:owner/:identifier` [🔗](https://docs.ethswarm.org/api/#tag/Single-owner-chunk/paths/~1soc~1%7Bowner%7D~1%7Bid%7D/post)                       | ❌✅✅   |
| `download_file`                       | `GET /bzz/:reference` [🔗](https://docs.ethswarm.org/api/#tag/BZZ/paths/~1bzz~1%7Breference%7D/get)                                                      | ✅✅✅   |
| `download_file`                       | `GET /bzz/:reference/:path` [🔗](https://docs.ethswarm.org/api/#tag/BZZ/paths/~1bzz~1%7Breference%7D~1%7Bpath%7D/get)                                    | ✅✅✅   |
| `download_readable_file`               | `GET /bzz/:reference` [🔗](https://docs.ethswarm.org/api/#tag/BZZ/paths/~1bzz~1%7Breference%7D/get)                                                      | ✅✅✅   |
| `download_data`                       | `GET /bytes/:reference` [🔗](https://docs.ethswarm.org/api/#tag/Bytes/paths/~1bytes~1%7Breference%7D/get)                                                | ✅✅✅   |
| `download_readable_data`               | `GET /bytes/:reference` [🔗](https://docs.ethswarm.org/api/#tag/Bytes/paths/~1bytes~1%7Breference%7D/get)                                                | ✅✅✅   |
| `download_chunk`                      | `GET /chunks/:reference` [🔗](https://docs.ethswarm.org/api/#tag/Chunk/paths/~1chunks~1%7Baddress%7D/get)                                                | ✅✅✅   |
| `create_feed_manifest`                 | `POST /feeds/:owner/:topic` [🔗](https://docs.ethswarm.org/api/#tag/Feed/paths/~1feeds~1%7Bowner%7D~1%7Btopic%7D/post)                                   | ❌✅✅   |
| `is_connected`                        | `GET /`                                                                                                                                                  | ✅✅✅   |
| `get_health`                          | `GET /health` [🔗](https://docs.ethswarm.org/api/#tag/Status/paths/~1health/get)                                                                         | ✅✅✅   |
| `get_readiness`                       | `GET /readiness` [🔗](https://docs.ethswarm.org/api/#tag/Status/paths/~1readiness/get)                                                                   | ✅✅✅   |
| `get_node_info`                        | `GET /node` [🔗](https://docs.ethswarm.org/api/#tag/Status/paths/~1node/get)                                                                             | ✅✅✅   |
| `get_chain_state`                      | `GET /chainstate` [🔗](https://docs.ethswarm.org/api/#tag/Status/paths/~1chainstate/get)                                                                 | ❌✅✅   |
| `get_redistribution_state`             | `GET /redistributionstate` [🔗](https://docs.ethswarm.org/api/#tag/RedistributionState/paths/~1redistributionstate/get)                                  | ❌❌✅   |
| `get_reserve_state`                    | `GET /reservestate` [🔗](https://docs.ethswarm.org/api/#tag/Status/paths/~1reservestate/get)                                                             | ❌❌✅   |
| `get_status`                          | `GET /status` [🔗](https://docs.ethswarm.org/api/#tag/Node-Status/paths/~1status/get)                                                                    | ✅✅✅   |
| `get_wallet`                          | `GET /wallet` [🔗](https://docs.ethswarm.org/api/#tag/Wallet/paths/~1wallet/get)                                                                         | ❌✅✅   |
| `get_topology`                        | `GET /topology` [🔗](https://docs.ethswarm.org/api/#tag/Connectivity/paths/~1topology/get)                                                               | ✅✅✅   |
| `get_addresses`                       | `GET /addresses` [🔗](https://docs.ethswarm.org/api/#tag/Connectivity/paths/~1addresses/get)                                                             | ✅✅✅   |
| `get_peers`                           | `GET /peers` [🔗](https://docs.ethswarm.org/api/#tag/Connectivity/paths/~1peers/get)                                                                     | ✅✅✅   |
| `get_all_balances`                     | `GET /balances` [🔗](https://docs.ethswarm.org/api/#tag/Balance/paths/~1balances/get)                                                                    | ❌✅✅   |
| `get_peer_balance`                     | `GET /balances/:peer` [🔗](https://docs.ethswarm.org/api/#tag/Balance/paths/~1balances~1%7Baddress%7D/get)                                               | ❌✅✅   |
| `get_past_due_consumption_balances`      | `GET /consumed` [🔗](https://docs.ethswarm.org/api/#tag/Balance/paths/~1consumed/get)                                                                    | ❌✅✅   |
| `get_past_due_consumption_peer_balance`   | `GET /consumed/:peer` [🔗](https://docs.ethswarm.org/api/#tag/Balance/paths/~1consumed~1%7Baddress%7D/get)                                               | ❌✅✅   |
| `get_all_settlements`                  | `GET /settlements` [🔗](https://docs.ethswarm.org/api/#tag/Settlements/paths/~1settlements/get)                                                          | ❌✅✅   |
| `get_settlements`                     | `GET /settlements/:peer` [🔗](https://docs.ethswarm.org/api/#tag/Settlements/paths/~1settlements~1%7Baddress%7D/get)                                     | ❌✅✅   |
| `get_chequebook_address`               | `GET /chequebook/address` [🔗](https://docs.ethswarm.org/api/#tag/Chequebook/paths/~1chequebook~1address/get)                                            | ❌✅✅   |
| `get_chequebook_balance`               | `GET /chequebook/balance` [🔗](https://docs.ethswarm.org/api/#tag/Chequebook/paths/~1chequebook~1balance/get)                                            | ❌✅✅   |
| `get_last_cheques`                     | `GET /chequebook/cheque` [🔗](https://docs.ethswarm.org/api/#tag/Chequebook/paths/~1chequebook~1cheque/get)                                              | ❌✅✅   |
| `get_last_cheques_for_peer`              | `GET /chequebook/cheque/:peer` [🔗](https://docs.ethswarm.org/api/#tag/Chequebook/paths/~1chequebook~1cheque~1%7Bpeer-id%7D/get)                         | ❌✅✅   |
| `get_last_cashout_action`               | `GET /chequebook/cashout/:peer` [🔗](https://docs.ethswarm.org/api/#tag/Chequebook/paths/~1chequebook~1cashout~1%7Bpeer-id%7D/get)                       | ❌✅✅   |
| `cashout_last_cheque`                  | `POST /chequebook/cashout/:peer` [🔗](https://docs.ethswarm.org/api/#tag/Chequebook/paths/~1chequebook~1cashout~1%7Bpeer-id%7D/post)                     | ❌✅✅   |
| `deposit_tokens`                      | `POST /chequebook/deposit` [🔗](https://docs.ethswarm.org/api/#tag/Chequebook/paths/~1chequebook~1deposit/post)                                          | ❌✅✅   |
| `withdraw_tokens`                     | `POST /chequebook/withdraw` [🔗](https://docs.ethswarm.org/api/#tag/Chequebook/paths/~1chequebook~1withdraw/post)                                        | ❌✅✅   |
| `get_all_pending_transactions`          | `GET /transactions` [🔗](https://docs.ethswarm.org/api/#tag/Transaction/paths/~1transactions/get)                                                        | ❌✅✅   |
| `get_pending_transaction`              | `GET /transactions/:id` [🔗](https://docs.ethswarm.org/api/#tag/Transaction/paths/~1transactions~1%7BtxHash%7D/get)                                      | ❌✅✅   |
| `rebroadcast_transaction`             | `POST /transactions/:id` [🔗](https://docs.ethswarm.org/api/#tag/Transaction/paths/~1transactions~1%7BtxHash%7D/post)                                    | ❌✅✅   |
| `cancel_transaction`                  | `DELETE /transactions/:id` [🔗](https://docs.ethswarm.org/api/#tag/Transaction/paths/~1transactions~1%7BtxHash%7D/delete)                                | ❌✅✅   |
| `create_tag`                          | `POST /tags` [🔗](https://docs.ethswarm.org/api/#tag/Tag/paths/~1tags/post)                                                                              | ❌✅✅   |
| `retrieve_tag`                        | `GET /tags/:id` [🔗](https://docs.ethswarm.org/api/#tag/Tag/paths/~1tags~1%7Buid%7D/get)                                                                 | ❌✅✅   |
| `get_all_tags`                         | `GET /tags` [🔗](https://docs.ethswarm.org/api/#tag/Tag/paths/~1tags/get)                                                                                | ❌✅✅   |
| `delete_tag`                          | `DELETE /tags/:id` [🔗](https://docs.ethswarm.org/api/#tag/Tag/paths/~1tags~1%7Buid%7D/delete)                                                           | ❌✅✅   |
| `update_tag`                          | `PATCH /tags/:id` [🔗](https://docs.ethswarm.org/api/#tag/Tag/paths/~1tags~1%7Buid%7D/patch)                                                             | ❌✅✅   |
| `pin`                                | `POST /pins/:reference` [🔗](https://docs.ethswarm.org/api/#tag/Pinning/paths/~1pins~1%7Breference%7D/post)                                              | ✅✅✅   |
| `get_all_pins`                         | `GET /pins` [🔗](https://docs.ethswarm.org/api/#tag/Pinning/paths/~1pins/get)                                                                            | ✅✅✅   |
| `get_pin`                             | `GET /pins/:reference` [🔗](https://docs.ethswarm.org/api/#tag/Pinning/paths/~1pins~1%7Breference%7D/get)                                                | ✅✅✅   |
| `is_reference_retrievable`             | `GET /stewardship/:reference` [🔗](https://docs.ethswarm.org/api/#tag/Stewardship/paths/~1stewardship~1%7Breference%7D/get)                              | ✅✅✅   |
| `reupload_pinned_data`                 | `PUT /stewardship/:reference` [🔗](https://docs.ethswarm.org/api/#tag/Stewardship/paths/~1stewardship~1%7Breference%7D/put)                              | ❌✅✅   |
| `unpin`                              | `DELETE /pins/:reference` [🔗](https://docs.ethswarm.org/api/#tag/Pinning/paths/~1pins~1%7Breference%7D/delete)                                          | ✅✅✅   |
| `get_grantees`                        | `GET /grantee/:reference` [🔗](https://docs.ethswarm.org/api/#tag/ACT/paths/~1grantee~1%7Breference%7D/get)                                              | ❌✅✅   |
| `create_grantees`                     | `POST /grantee` [🔗](https://docs.ethswarm.org/api/#tag/ACT/paths/~1grantee/post)                                                                        | ❌✅✅   |
| `patch_grantees`                      | `PATCH /grantee/:reference` [🔗](https://docs.ethswarm.org/api/#tag/ACT/paths/~1grantee~1%7Breference%7D/patch)                                          | ❌✅✅   |
| `pss_send`                            | `POST /pss/send/:topic/:target` [🔗](https://docs.ethswarm.org/api/#tag/Postal-Service-for-Swarm/paths/~1pss~1send~1%7Btopic%7D~1%7Btargets%7D/post)     | ❌✅✅   |
| `pss_subscribe` _Websocket_           | `GET /pss/subscribe/:topic` [🔗](https://docs.ethswarm.org/api/#tag/Postal-Service-for-Swarm/paths/~1pss~1subscribe~1%7Btopic%7D/get)                    | ❌❌✅   |
| `pss_receive`                         | `GET /pss/subscribe/:topic` [🔗](https://docs.ethswarm.org/api/#tag/Postal-Service-for-Swarm/paths/~1pss~1subscribe~1%7Btopic%7D/get)                    | ❌❌✅   |
| `get_all_postage_batch`                 | `GET /stamps` [🔗](https://docs.ethswarm.org/api/#tag/Postage-Stamps/paths/~1stamps/get)                                                                 | ❌✅✅   |
| `get_global_postage_batches`            | `GET /batches` [🔗](https://docs.ethswarm.org/api/#tag/Postage-Stamps/paths/~1batches/get)                                                               | ❌✅✅   |
| `get_postage_batch`                    | `GET /stamps/:batchId` [🔗](https://docs.ethswarm.org/api/#tag/Postage-Stamps/paths/~1stamps~1%7Bbatch_id%7D/get)                                        | ❌✅✅   |
| `get_postage_batch_buckets`             | `GET /stamps/:batchId/buckets` [🔗](https://docs.ethswarm.org/api/#tag/Postage-Stamps/paths/~1stamps~1%7Bbatch_id%7D~1buckets/get)                       | ❌✅✅   |
| `create_postage_batch`                 | `POST /stamps/:amount/:depth` [🔗](https://docs.ethswarm.org/api/#tag/Postage-Stamps/paths/~1stamps~1%7Bamount%7D~1%7Bdepth%7D/post)                     | ❌✅✅   |
| `top_up_batch`                         | `PATCH /stamps/topup/:batchId/:amount` [🔗](https://docs.ethswarm.org/api/#tag/Postage-Stamps/paths/~1stamps~1topup~1%7Bbatch_id%7D~1%7Bamount%7D/patch) | ❌✅✅   |
| `dilute_batch`                        | `PATCH /stamps/dilute/:batchId/:depth` [🔗](https://docs.ethswarm.org/api/#tag/Postage-Stamps/paths/~1stamps~1dilute~1%7Bbatch_id%7D~1%7Bdepth%7D/patch) | ❌✅✅   |
| `create_envelope`                     | `POST /envelope/:reference` [🔗](https://docs.ethswarm.org/api/#tag/Envelope/paths/~1envelope~1%7Baddress%7D/post)                                       | ❌✅✅   |
| `get_stake`                           | `GET /stake` [🔗](https://docs.ethswarm.org/api/#tag/Staking/paths/~1stake/get)                                                                          | ❌❌✅   |
| `deposit_stake`                       | `POST /stake` [🔗](https://docs.ethswarm.org/api/#tag/Staking/paths/~1stake~1%7Bamount%7D/post)                                                          | ❌❌✅   |

### Utils

#### General

- `get_collection_size`
- `get_folder_size`

#### PSS

- `make_max_target`

#### Erasure Coding

- `approximate_overhead_for_redundancy_level`
- `get_redundancy_stat`
- `get_redundancy_stats`

#### Stamps

- `get_amount_for_ttl`
- `get_depth_for_capacity`
- `get_stamp_cost`
- `get_stamp_effective_bytes`
- `get_stamp_maximum_capacity_bytes`
- `get_stamp_ttl_seconds`
- `get_stamp_usage`

## Usage

### Upload via Swarm Gateway

```rust
use bee_rs::{Bee, NULL_STAMP, SWARM_GATEWAY_URL};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bee = Bee::new(SWARM_GATEWAY_URL);
    let result = bee.upload_data(NULL_STAMP, "Hello, World!").await?;
    println!("{}", result.reference);
    Ok(())
}
```

### Create or select an existing postage batch

Swarm incentivizes nodes in the network to store content, therefor all uploads require a paid
[postage batch](https://docs.ethswarm.org/docs/learn/technology/contracts/postage-stamp).

```rust
use bee_rs::Bee;

async fn get_or_create_postage_batch() -> Result<String, Box<dyn std::error::Error>> {
    let bee = Bee::new("http://localhost:1633");
    let batch_id = if let Some(usable) = bee.get_all_postage_batch().await?.into_iter().find(|x| x.usable) {
        usable.batch_id
    } else {
        bee.buy_storage(Size::from_gigabytes(1), Duration::from_days(7)).await?
    };
    Ok(batch_id)
}
```

> The following examples all assume an existing batch_id.

### Upload simple data

```rust
use bee_rs::Bee;

let bee = Bee::new("http://localhost:1633");

let upload_result = bee.upload_data(&batch_id, "Bee is awesome!").await?;
let data = bee.download_data(upload_result.reference).await?;

println!("{}", String::from_utf8_lossy(&data)); // prints 'Bee is awesome!'
```

### Upload arbitrary large file

```rust
use bee_rs::Bee;
use tokio::fs::File;

let bee = Bee::new("http://localhost:1633");
let file = File::open("./path/to/large.bin").await?;
let upload_result = bee.upload_file(&batch_id, file).await?;
```

## Contribute

Stay up to date by joining the [official Discord](https://discord.gg/GU22h2utj6) and by keeping an eye on the
[releases tab](https://github.com/ethersphere/bee-rs/releases).

We are using [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) for our commit messages and pull
requests, following the [Semantic Versioning](https://semver.org/) rules.

There are some ways you can make this module better:

- Consult our [open issues](https://github.com/ethersphere/bee-rs/issues) and take on one of them
- Help our tests reach 100% coverage!
- Join us in our [Discord chat](https://discord.gg/wdghaQsGq5) in the #develop-on-swarm channel if you have questions or
  want to give feedback

### Setup

Install project dependencies:

```sh
cargo build
```

### Test

```sh
cargo test
```

## License

[BSD-3-Clause](./LICENSE)
