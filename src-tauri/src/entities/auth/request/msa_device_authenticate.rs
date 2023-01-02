use crate::api::http::{Request, Refresh};

#[derive(Debug, serde::Serialize)]
pub struct MSADeviceAuthenticateRequest<'a>(&'a str);

impl<'a> MSADeviceAuthenticateRequest<'a> {
    pub fn new(device_code: &'a str) -> Self {
        Self(device_code)
    }
}

#[async_trait::async_trait]
impl<'a> Request for MSADeviceAuthenticateRequest<'a> {
    type Client = reqwest::Client;
    type Response = reqwest::Response;
    type Rejection = DeviceAuthenticateRejection;

    async fn request(self, client: &Self::Client) -> Result<Self::Response, Self::Rejection> {
        let url = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token"
            .parse::<url::Url>()
            .map_err(DeviceAuthenticateRejection::UrlParse)?;

        let res = client.post(url)
            .form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
                ("client_id", "3ea1cbe9-4e3a-4a2f-85e7-cca409b6a8ca"),
                ("device_code", self.0)
            ])
            .send()
            .await
            .map_err(DeviceAuthenticateRejection::Reqwest)?;

        Ok(res)
    }
}

#[derive(Debug, serde::Serialize)]
pub struct MSADeviceAuthenticateRefresh<'a>(&'a str);

impl<'a> MSADeviceAuthenticateRefresh<'a> {
    pub fn new(refresh_code: &'a str) -> Self {
        Self(refresh_code)
    }
}

#[async_trait::async_trait]
impl<'a> Refresh for MSADeviceAuthenticateRefresh<'a> {
    type Client = reqwest::Client;
    type Response = reqwest::Response;
    type Rejection = DeviceAuthenticateRejection;

    async fn refresh(self, client: &Self::Client) -> Result<Self::Response, Self::Rejection> {
        let url = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token"
            .parse::<url::Url>()
            .map_err(DeviceAuthenticateRejection::UrlParse)?;

        let refreshed = client.post(url)
            .form(&[
                ("grant_type", "refresh_token"),
                ("client_id", "3ea1cbe9-4e3a-4a2f-85e7-cca409b6a8ca"),
                ("refresh_token", self.0)
            ])
            .send().await
            .map_err(DeviceAuthenticateRejection::Reqwest)?;
        Ok(refreshed)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DeviceAuthenticateRejection {
    #[error("failed url parse. {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("failed request sending. {0}")]
    Reqwest(#[from] reqwest::Error),
}