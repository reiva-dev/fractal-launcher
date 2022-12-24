
use super::FlowDependent;

#[derive(Debug, serde::Serialize)]
pub struct MSADeviceAuthenticateRequest<'a>(&'a str);

impl<'a> MSADeviceAuthenticateRequest<'a> {
    pub fn new(device_code: &'a str) -> Self {
        Self(device_code)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct MSADeviceAuthenticateReponse {
    token_type: String,
    scope: String,
    expires_in: i32,
    ext_expires_in: i32,
    access_token: String,
    refresh_token: String
}

impl MSADeviceAuthenticateReponse {
    pub fn access_token(&self) -> &str {
        &self.access_token
    }
}

impl FlowDependent for MSADeviceAuthenticateReponse {
    type Flowed = String;

    fn flow(&self) -> Self::Flowed {
        self.access_token.clone()
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

#[async_trait::async_trait]
impl Refresh for MSADeviceAuthenticateReponse {
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
                ("refresh_token", &self.refresh_token)
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