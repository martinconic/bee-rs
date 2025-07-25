
// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The Bee client.

use reqwest::Client;

use crate::api::stewardship::Stewardship;

/// The Bee client.
#[derive(Debug)]
pub struct Bee {
    url: String,
    client: Client,
}

impl Bee {
    /// Create a new Bee client.
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            client: Client::new(),
        }
    }

    /// Get the client.
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get the URL.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Get the stewardship endpoint.
    pub fn stewardship(&self) -> Stewardship {
        Stewardship::new(self)
    }
}
