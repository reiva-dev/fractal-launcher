use crate::api::http::Request;

#[derive(Debug, serde::Serialize)]
pub struct VersionManifestRequest;

#[async_trait::async_trait]
impl Request for VersionManifestRequest {
    type Client = reqwest::Client;
    type Response = reqwest::Response;
    type Rejection = VersionManifestRejection;

    async fn request(self, client: &Self::Client) -> Result<Self::Response, Self::Rejection> {
        let url = "https://launchermeta.mojang.com/mc/game/version_manifest.json"
            .parse::<url::Url>()
            .map_err(|e| VersionManifestRejection::UrlParse(e.to_string()))?;

        let res = client.get(url)
            .send()
            .await
            .map_err(|e| VersionManifestRejection::Reqwest(e.to_string()))?;

        Ok(res)
    }
}

#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum VersionManifestRejection {
    #[error("failed parse url. {0}")]
    UrlParse(String),
    #[error("failed send request. {0}")]
    Reqwest(String)
}