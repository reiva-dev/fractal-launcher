use super::{Request, Requestor, Response, http_client};

/// The implementation is the same as [`requestor::Request`].
/// 
/// This Trait should be implemented in an `Entity` that can use `RefreshToken` to regenerate valid `Entities`.
#[async_trait::async_trait]
pub trait Refresh: Sized + serde::ser::Serialize {
    type Client;
    type Response;
    type Rejection: std::fmt::Debug;

    async fn refresh(self, client: &Self::Client) -> Result<Self::Response, Self::Rejection>;
}

impl<T: Refresh<Client = reqwest::Client>> Requestor<T> {
    pub async fn refresh(self) -> Result<Response<T::Response>, T::Rejection> {
        match self.0.refresh(http_client()).await {
            Ok(res) => Ok(Response::new(res)),
            Err(error) => {
                tracing::error!("request was not processed correctly. {:?}", error);
                Err(error)
            },
        }
    }
}