//! Copyright 2022 - Fractal Launcher - ReiRokusanami

use std::collections::HashMap;

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