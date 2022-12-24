#![allow(unused)]
#![forbid(unsafe_code)]
//! `Copyright 2022 - Fractal Launcher - ReiRokusanami`
//!
//! This API is a collection of implementations that simplify processing for http.
//! 
//! This module uses the following static [`http_client()`] to perform the processing.
//! 
//! Dependency
//! - [`serde`]
//! - [`async_trait`]
//! - [`reqwest`]
//! - [`once_cell`]

mod refresh;
mod response;
mod requestor;

pub use self::{
    requestor::{Requestor, Request},
    response::Response,
    refresh::Refresh
};

/// This function returns a static reference [`reqwest::Client`].
/// 
/// Internally, use [`once_cell::sync::Lazy`] to handle the creation of the Client statically.
fn http_client() -> &'static reqwest::Client {
    use once_cell::sync::Lazy;
    static HTTP_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
        reqwest::Client::new()
    });
    &HTTP_CLIENT
}