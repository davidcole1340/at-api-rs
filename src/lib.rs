//! # at-api-rs
//!
//! Tools for interacting with the [Auckland Transport API](https://dev-portal.at.govt.nz/).
//! You must register to receive an API key to use this library.

pub mod error;
mod realtime;
pub mod types;

// Auckland Transport base API URL.
pub(crate) const BASE_API_URL: &str = "https://api.at.govt.nz/v2";

pub use realtime::Realtime;
