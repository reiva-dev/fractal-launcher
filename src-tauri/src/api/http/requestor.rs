//! Copyright 2022 - Fractal Launcher - ReiRokusanami

//! This API is a somewhat abstracted version of HttpRequest.
//! Essentially, HttpRequests such as `GET`/`POST` have a process in the logic, 
//! which is often twice as long when writing other similar processes.
//! Therefore, it is designed to implement the request processing 
//! in a structure and change it to a calling format.
//! 
//! ---
//! ### Note.
//! 
//! HttpClient is very inefficient 
//! if it is generated every time a function is executed.
//! 
//! For example, the HttpClient of [`reqwest`] has the following description.
//! 
//! The Client holds a connection pool internally, 
//! so it is advised that you create one and **reuse** it.
//!
//! So here I assume that HttpClient is set up to provide a static reference.
//! 
//! ### Usage.
//! ```rust,no_run
//! #[derive(Debug, Default)]
//! pub struct AuthenticationDataResponse {
//!     token: String,
//!     expired_in: i32
//! }
//! 
//! #[async_trait::async_trait]
//! impl Request for AuthenticationDataResponse {
//!     type Client: reqwest::Client;
//!     type Response: reqwest::Response;
//!     type Rejection: reqwest::Error;
//!     async fn request(self, client: &Self::Client) -> Result<Self::Response, Self::Rejection> {
//!         client.get("http://example.com/auth")
//!             .send()
//!             .await?
//!     }
//! }
//! 
//! let response = Requestor::new(AuthenticationDataResponse::default())
//!     .execute()
//!     .await
//!     .unwrap()
//!     .map(|res| res.json::<AuthenticationDataResponse>())
//!     .await
//!     .unwrap();
//! ```

use super::{Response, http_client, Refresh};


/// Trait required when using [`Requestor`].
/// 
/// Implement an instruction that requests a structure or 
/// other object to be implemented in the [`Request::request()`] method.
/// 
/// If you want to import values from outside, see `RequestSynthetic`.
/// 
/// Also, HttpRequest is often asynchronous, so we require [`async_trait`].
#[async_trait::async_trait]
pub trait Request: Sized + serde::ser::Serialize {
    type Client;
    type Response;
    type Rejection: std::fmt::Debug;

    async fn request(self, client: &Self::Client) -> Result<Self::Response, Self::Rejection>;
}

#[async_trait::async_trait]
pub trait RequestSyntheticMix1: Sized + serde::ser::Serialize {
    type Client;
    type Response;
    type Rejection: std::fmt::Debug;

    type Mix1;
    
    async fn request(self, client: &Self::Client, outer_1: Self::Mix1) -> Result<Self::Response, Self::Rejection>;
}

/// This structure is used to execute the asynchronous function 
/// [`Request::request()`]
/// 
/// It is designed to use `HttpClient`, which is static internally.
#[derive(Debug)]
pub struct Requestor<T>(pub(super) T);

/// This implementation uses [`reqwest::Client`].
/// `&'static reqwest::Client` is used internally.
impl<T: Request<Client = reqwest::Client>> Requestor<T> {
    /// The argument required for the initialization process is the return value of this order.
    /// 
    /// An entity is required, but a [`Default`] or empty structure is also acceptable.
    pub fn new(order: T) -> Self {
        Self(order)
    }

    pub async fn execute(self) -> Result<Response<T::Response>, T::Rejection> {
        match self.0.request(http_client()).await {
            Ok(res) => Ok(Response::new(res)),
            Err(error) => {
                tracing::error!("request was not processed correctly. {:?}", error);
                Err(error)
            },
        }
    }
}

impl<T: RequestSyntheticMix1<Client = reqwest::Client>> Requestor<T> {
    /// Initialization process used when requesting external values.
    /// 
    /// The basic behavior is the same as [`Requestor::new()`], but [`RequestSyntheticT1`] is required to be implemented.
    pub fn synthetic(order: T) -> Self {
        Self(order)
    }

    pub async fn execute_sythetic(self, outer: T::Mix1) -> Result<Response<T::Response>, T::Rejection> {
        match self.0.request(http_client(), outer).await {
            Ok(res) => Ok(Response::new(res)),
            Err(error) => {
                tracing::error!("request was not processed correctly. {:?}", error);
                Err(error)
            },
        }
    }
}