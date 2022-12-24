mod msa_device_verifing;
mod msa_device_authenticate;
mod msa_device_flow_error;

mod xbl_xsts_authorize;
mod xbl_user_authenticate;

#[deprecated]
#[async_trait::async_trait]
trait Refreshable: Sized {
    async fn refresh(self) -> Result<Self, RefreshError>;
}

#[deprecated]
#[derive(Debug, thiserror::Error)]
enum RefreshError {
    #[error("failed reqwest task. {0:?}")]
    Reqwest(#[from] reqwest::Error)
}

pub trait FlowDependent {
    type Flowed;
    fn flow(&self) -> Self::Flowed;
}

pub trait FlowSteppable<T: FlowDependent> {
    fn step(self, dep: T) -> Self;
}

pub use self::{
    msa_device_verifing::MSADeviceVerifingResponse,
    msa_device_authenticate::MSADeviceAuthenticateReponse,
    msa_device_flow_error::{MSADeviceFlowError, MSADeviceFlowErrorResponse},
    xbl_user_authenticate::{
        XboxLiveUserAuthenticateRequest,
        XboxLiveUserAuthenticateProperty,
        XboxLiveUserAuthenticateResponse
    },
    xbl_xsts_authorize::{
        XBoxLiveSTSAuthorizeRequest,
        XBoxLiveSTSAuthorizeProperty,
        XBoxLiveSTSAuthorizeResponse
    }
};