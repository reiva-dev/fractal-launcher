use std::collections::HashMap;

use crate::api::http::Request;

use super::{FlowSteppable, FlowDependent};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct XBoxLiveSTSAuthorizeRequest {
    pub properties: XBoxLiveSTSAuthorizeProperty,
    pub relying_party: String,
    pub token_type: String
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct XBoxLiveSTSAuthorizeProperty {
    pub sandbox_id: String,
    pub user_tokens: Vec<String>
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct XBoxLiveSTSAuthorizeResponse {
    issue_instant: String,
    not_after: String,
    token: String,
    display_claims: HashMap<String, Vec<HashMap<String, String>>>,
}

impl XBoxLiveSTSAuthorizeResponse {
    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn user_hash(&self) -> &str {
        &self.display_claims["xui"][0]["uhs"]
    }
}

#[async_trait::async_trait]
impl Request for XBoxLiveSTSAuthorizeRequest {
    type Client = reqwest::Client;
    type Response = reqwest::Response;
    type Rejection = XBoxLiveSTSAuthorizeRejection;

    async fn request(self, client: &Self::Client) -> Result<Self::Response, Self::Rejection> {
        let url = "https://xsts.auth.xboxlive.com/xsts/authorize"
            .parse::<url::Url>()
            .map_err(XBoxLiveSTSAuthorizeRejection::UrlParse)?;

        let res = client.post(url)
            .json(&self)
            .send()
            .await
            .map_err(XBoxLiveSTSAuthorizeRejection::Reqwest)?;
        
        Ok(res)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum XBoxLiveSTSAuthorizeRejection {
    #[error("failed url Parse. {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("failed request send. {0}")]
    Reqwest(#[from] reqwest::Error)
}

impl<T> FlowSteppable<T> for XBoxLiveSTSAuthorizeRequest
  where T: FlowDependent<Flowed = String> {
    fn step(mut self, dep: T) -> Self {
        self.properties.user_tokens = Vec::from([dep.flow()]);
        self
    }
}

impl FlowDependent for XBoxLiveSTSAuthorizeResponse {
    type Flowed = String;

    fn flow(&self) -> Self::Flowed {
        self.token.clone()
    }
}

impl Default for XBoxLiveSTSAuthorizeRequest {
    fn default() -> Self {
        XBoxLiveSTSAuthorizeRequest { 
            properties: XBoxLiveSTSAuthorizeProperty::default(), 
            relying_party: String::from("rp://api.minecraftservices.com/"), 
            token_type: String::from("JWT")
        }
    }
}

impl Default for XBoxLiveSTSAuthorizeProperty {
    fn default() -> Self {
        XBoxLiveSTSAuthorizeProperty {
            sandbox_id: String::from("RETAIL"),
            user_tokens: Vec::from([String::from("UserTokens")])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{XBoxLiveSTSAuthorizeRequest, XBoxLiveSTSAuthorizeProperty};

    #[test]
    fn resolve_test() {
        let _ = XBoxLiveSTSAuthorizeRequest {
            properties: XBoxLiveSTSAuthorizeProperty { 
                user_tokens: Vec::from([String::from("USER_TOKEN")]), 
                ..Default::default()
            },
            ..Default::default()
        };
    }
}