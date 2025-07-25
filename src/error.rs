
// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Error handling in bee-rs.

use thiserror::Error;

/// The error type for the bee-rs library.
#[derive(Debug, Error)]
pub enum Error {
    /// Reqwest error
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// Status code error
    #[error("Status code error: {0}")]
    StatusCode(reqwest::StatusCode),
    /// Custom error
    #[error("{0}")]
    Custom(String),
}
