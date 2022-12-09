#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MSADeviceAuthenticateReponse {
    token_type: String,
    scope: String,
    expires_in: i32,
    ext_expires_in: i32,
    access_token: String,
    refresh_token: String
}