#![warn(future_incompatible, rust_2018_compatibility, rust_2018_idioms, unused)]
#![warn(clippy::all)]
#![allow(broken_intra_doc_links)]
// TODO: Documentation
//#![warn(missing_docs)]

pub mod api;
pub(crate) mod auth;
mod client;
mod error;

pub use client::{ItadApiBuilder, ItadApiClient, ItadApiClientAsync};
pub use error::{ItadApiError, ItadApiResult};
