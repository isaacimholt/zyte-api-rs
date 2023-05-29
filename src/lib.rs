//! # zyte-api-rs
//!
//! This is an unofficial rust package for the Zyte API.
pub mod request;
pub mod request_builder;
pub mod response;
pub mod zyte_api;

pub use request::{HttpRequestBodyType, Request};
pub use request_builder::RequestBuilder;
pub use response::Response;
pub use zyte_api::ZyteApi;
