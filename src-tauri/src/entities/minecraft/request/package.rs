use crate::api::http::Request;

#[derive(Debug, serde::Serialize)]
pub struct PackageRequest<'a>(&'a str);

impl<'a> PackageRequest<'a> {
    pub fn new(url: &'a str) -> Self {
        Self(url)
    }
}

#[async_trait::async_trait]
impl<'a> Request for PackageRequest<'a> {
    type Client = reqwest::Client;
    type Response = reqwest::Response;
    type Rejection = PackageRejection;

    async fn request(self, client: &Self::Client) -> Result<Self::Response, Self::Rejection> {
        let url = self.0
            .parse::<url::Url>()
            .map_err(|e|  PackageRejection::UrlParse(e.to_string()))?;

        let res = client.get(url)
            .send()
            .await
            .map_err(|e|  PackageRejection::Reqwest(e.to_string()))?;

        Ok(res)
    }
}

#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum PackageRejection {
    #[error("failed parse url. {0}")]
    UrlParse(String),
    #[error("failed send request. {0}")]
    Reqwest(String)
}