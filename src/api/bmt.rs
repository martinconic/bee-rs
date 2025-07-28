// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Bee-js `bmt` module implementation.
//! https://github.com/ethersphere/bee-js/blob/main/src/chunk/bmt.ts

use crate::error::Error;
use sha3::{Digest, Keccak256};

const MAX_CHUNK_PAYLOAD_SIZE: usize = 4096;
const SEGMENT_SIZE: usize = 32;

/// Calculate a Binary Merkle Tree hash for a chunk.
///
/// The BMT chunk address is the hash of the 8-byte span and the root
/// hash of a binary Merkle tree (BMT) built on the 32-byte segments
/// of the underlying data.
///
/// If the chunk content is less than 4k, the hash is calculated as
/// if the chunk was padded with all zeros up to 4096 bytes.
///
/// # Arguments
///
/// * `chunk_content` - Chunk data including span and payload.
///
/// # Returns
///
/// The keccak256 hash in a byte array.
pub fn calculate_chunk_address(chunk_content: &[u8]) -> Result<[u8; 32], Error> {
    let span = &chunk_content[0..8];
    let payload = &chunk_content[8..];
    let root_hash = calculate_bmt_root_hash(payload)?;

    let mut hasher = Keccak256::new();
    hasher.update(span);
    hasher.update(root_hash);
    Ok(hasher.finalize().into())
}

fn calculate_bmt_root_hash(payload: &[u8]) -> Result<[u8; 32], Error> {
    if payload.len() > MAX_CHUNK_PAYLOAD_SIZE {
        return Err(Error::Custom(format!(
            "payload size {} exceeds maximum chunk payload size {}",
            payload.len(),
            MAX_CHUNK_PAYLOAD_SIZE
        )));
    }

    let mut input = vec![0u8; MAX_CHUNK_PAYLOAD_SIZE];
    input[..payload.len()].copy_from_slice(payload);

    let mut segments: Vec<[u8; 32]> = input
        .chunks_exact(SEGMENT_SIZE)
        .map(|s| s.try_into().unwrap())
        .collect();

    while segments.len() > 1 {
        let mut next_level = Vec::new();
        for i in (0..segments.len()).step_by(2) {
            let mut hasher = Keccak256::new();
            hasher.update(segments[i]);
            hasher.update(segments[i + 1]);
            next_level.push(hasher.finalize().into());
        }
        segments = next_level;
    }

    Ok(segments[0])
}
