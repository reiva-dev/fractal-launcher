use crate::api::http::Request;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct XboxLiveUserAuthenticateRequest {
    pub properties: XboxLiveUserAuthenticateProperty,
    pub relying_party: String,
    pub token_type: String
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct XboxLiveUserAuthenticateProperty {
    pub auth_method: String,
    pub site_name: String,
    pub rps_ticket: String
}

impl XboxLiveUserAuthenticateRequest {
    pub fn new(rps_ticket: impl Into<String>) -> Self {
        XboxLiveUserAuthenticateRequest { 
            relying_party: String::from("http://auth.xboxlive.com"), 
            token_type: String::from("JWT"),
            properties: XboxLiveUserAuthenticateProperty {
                auth_method: String::from("RPS"),
                site_name: String::from("user.auth.xboxlive.com"),
                rps_ticket: format!("d={}", rps_ticket.into())
            }
        }
    }
}

#[async_trait::async_trait]
impl Request for XboxLiveUserAuthenticateRequest {
    type Client = reqwest::Client;
    type Response = reqwest::Response;
    type Rejection = XboxLiveUserAuthenticateRejection;

    async fn request(self, client: &Self::Client) -> Result<Self::Response, Self::Rejection> {
        let url = "https://user.auth.xboxlive.com/user/authenticate"
            .parse::<url::Url>()
            .map_err(XboxLiveUserAuthenticateRejection::UrlParse)?;

        let res = client.post(url)
            .json(&self)
            .send()
            .await
            .map_err(XboxLiveUserAuthenticateRejection::Reqwest)?;
        
        Ok(res)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum XboxLiveUserAuthenticateRejection {
    #[error("failed parse url. {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("failed request send. {0}")]
    Reqwest(#[from] reqwest::Error),
}
