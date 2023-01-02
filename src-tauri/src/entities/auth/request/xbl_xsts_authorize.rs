use crate::api::http::Request;

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

impl XBoxLiveSTSAuthorizeRequest {
    pub fn new(user_tokens: &[String]) -> Self {
        XBoxLiveSTSAuthorizeRequest { 
            relying_party: String::from("rp://api.minecraftservices.com/"), 
            token_type: String::from("JWT"),
            properties: XBoxLiveSTSAuthorizeProperty {
                sandbox_id: String::from("RETAIL"),
                user_tokens: Vec::from(user_tokens)
            },
        }
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