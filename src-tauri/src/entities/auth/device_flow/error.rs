//! Copyright 2022 - Fractal Launcher - ReiRokusanami

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSADeviceFlowErrorResponse {
    error: String,
    error_description: String,
    error_codes: Vec<i32>,
    timestamp: String,
    trace_id: String,
    correlation_id: String,
    error_uri: String
}

#[derive(Debug)]
pub enum MSADeviceFlowError {
    AuthorizationPending,
    AuthorizeDeclined,
    BadVerificationCode,
    ExpiredToken,
    Unknown
}

impl From<MSADeviceFlowErrorResponse> for MSADeviceFlowError {
    fn from(res: MSADeviceFlowErrorResponse) -> Self {
        match res.error.as_str() {
            "authorization_pending" => Self::AuthorizationPending,
            "authorization_declined" => Self::AuthorizeDeclined,
            "bad_verification_code" => Self::BadVerificationCode,
            "expired_token" => Self::ExpiredToken,
            _ => Self::Unknown
        }
    }
}