use crate::api::http::Request;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MSADeviceVerifingRequest;

#[async_trait::async_trait]
impl Request for MSADeviceVerifingRequest {
    type Client = reqwest::Client;
    type Response = reqwest::Response;
    type Rejection = VerifingRequestRejection;

    async fn request(self, client: &Self::Client) -> Result<Self::Response, Self::Rejection> {
        let url = "https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode"
            .parse::<url::Url>()
            .map_err(VerifingRequestRejection::UrlParse)?;
        let res = client.post(url)
            .form(&[
                ("client_id", "3ea1cbe9-4e3a-4a2f-85e7-cca409b6a8ca"),
                ("scope", "XboxLive.signin offline_access")
            ])
            .send()
            .await
            .map_err(VerifingRequestRejection::FormSending)?;
        Ok(res)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum VerifingRequestRejection {
    #[error("failed url parse. {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("failed request sending. {0}")]
    FormSending(#[from] reqwest::Error)
}