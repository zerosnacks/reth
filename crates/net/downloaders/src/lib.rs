//! Implements the downloader algorithms.
//!
//! ## Feature Flags
//!
//! - `test-utils`: Export utilities for testing

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/paradigmxyz/reth/main/assets/reth-docs.png",
    html_favicon_url = "https://avatars0.githubusercontent.com/u/97369466?s=256",
    issue_tracker_base_url = "https://github.com/paradigmxyz/reth/issues/"
)]
#![warn(missing_debug_implementations, missing_docs, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

/// The collection of algorithms for downloading block bodies.
pub mod bodies;

/// The collection of algorithms for downloading block headers.
pub mod headers;

/// Common downloader metrics.
pub mod metrics;

#[cfg(any(test, feature = "test-utils"))]
pub mod test_utils;
