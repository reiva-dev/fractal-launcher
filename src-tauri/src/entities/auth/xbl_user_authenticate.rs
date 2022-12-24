use std::collections::HashMap;

use crate::api::http::Request;

use super::{FlowSteppable, FlowDependent};

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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct XboxLiveUserAuthenticateResponse {
    issue_instant: String,
    not_after: String,
    token: String,
    display_claims: HashMap<String, Vec<HashMap<String, String>>>,
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


impl XboxLiveUserAuthenticateResponse {
    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn user_hash(&self) -> &str {
        &self.display_claims["xui"][0]["uhs"]
    }
}

impl<T> FlowSteppable<T> for XboxLiveUserAuthenticateRequest
  where T: FlowDependent<Flowed = String> {
    fn step(mut self, dep: T) -> Self {
        self.properties.rps_ticket = format!("d={}", dep.flow());
        self
    }
}

impl FlowDependent for XboxLiveUserAuthenticateResponse {
    type Flowed = String;
    fn flow(&self) -> Self::Flowed {
        self.token.clone()
    }
}

impl Default for XboxLiveUserAuthenticateRequest {
    fn default() -> Self {
        XboxLiveUserAuthenticateRequest { 
            properties: XboxLiveUserAuthenticateProperty::default(), 
            relying_party: String::from("http://auth.xboxlive.com"), 
            token_type: String::from("JWT")
        }
    }
}

impl Default for XboxLiveUserAuthenticateProperty {
    fn default() -> Self {
        XboxLiveUserAuthenticateProperty {
            auth_method: String::from("RPS"),
            site_name: String::from("user.auth.xboxlive.com"),
            rps_ticket: String::from("d=TICKET")
        }
    }
}