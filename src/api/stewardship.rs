// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Bee-js `stewardship` module implementation.
//! https://github.com/ethersphere/bee-js/blob/main/src/modules/stewardship.ts
//!
//! ## Endpoints
//!
//! - `GET /stewardship/{reference}`: Re-uploads a chunk to the network.
//! - `PUT /stewardship/{reference}`: Checks if a chunk is retrievable.

use crate::{bee::Bee, error::Error};

const ENDPOINT: &str = "stewardship";

/// The `stewardship` endpoint.
pub struct Stewardship<'a> {
    bee: &'a Bee,
}

impl<'a> Stewardship<'a> {
    /// Create a new `Stewardship` endpoint.
    pub fn new(bee: &'a Bee) -> Self {
        Self { bee }
    }

    /// Re-uploads a chunk to the network.
    ///
    /// ## Arguments
    ///
    /// * `reference` - The reference of the chunk to re-upload.
    pub async fn reupload(&self, reference: &str) -> Result<(), Error> {
        let url = format!("{}/{}/{}", self.bee.url(), ENDPOINT, reference);
        let status = self
            .bee
            .client()
            .post(url)
            .send()
            .await?
            .status();

        if status.is_success() {
            Ok(())
        } else {
            Err(Error::StatusCode(status))
        }
    }

    /// Checks if a chunk is retrievable.
    ///
    /// ## Arguments
    ///
    /// * `reference` - The reference of the chunk to check.
    pub async fn is_retrievable(&self, reference: &str) -> Result<bool, Error> {
        let url = format!("{}/{}/{}", self.bee.url(), ENDPOINT, reference);
        let status = self
            .bee
            .client()
            .put(url)
            .send()
            .await?
            .status();

        if status.is_success() {
            Ok(true)
        } else if status.as_u16() == 404 {
            Ok(false)
        } else {
            Err(Error::StatusCode(status))
        }
    }
}