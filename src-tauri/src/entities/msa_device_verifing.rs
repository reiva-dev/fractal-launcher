//! Copyright 2022 - Fractal Launcher - ReiRokusanami

/// This structure corresponds to the Microsoft Azure ActiveDirectory device authorization response.
/// 
/// **Note**: that it is of type `String`, not `Url`, 
/// to eliminate the dependency of the `verification_uri` on another Crate.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSADeviceVerifingResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: i32,
    interval: i32,
    message: String
}

impl MSADeviceVerifingResponse {
    pub fn device_code(&self) -> &str {
        &self.device_code
    }

    pub fn user_code(&self) -> &str {
        &self.user_code
    }

    pub fn expires_in(&self) -> i32 {
        self.expires_in
    }

    pub fn interval(&self) -> i32 {
        self.interval
    }

    pub fn verification_uri(&self) -> &str {
        &self.verification_uri
    }
}