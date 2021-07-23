mod client;
mod common;
pub mod deals;
mod endpoint;
mod error;
pub mod game;
mod query;
pub mod search;
pub mod user;
mod utils;
pub mod web;

pub use client::{AsyncClient, Client, RestClient};
pub use error::ApiError;
