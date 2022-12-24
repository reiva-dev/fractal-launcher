mod msa_device_verifing;
mod msa_device_authenticate;
mod msa_device_flow_error;

mod xbl_xsts_authorize;
mod xbl_user_authenticate;

pub trait FlowDependent {
    type Flowed;
    fn flow(&self) -> Self::Flowed;
}

pub trait FlowSteppable<T: FlowDependent> {
    fn step(self, dep: T) -> Self;
}

pub use self::{
    msa_device_verifing::{
        MSADeviceVerifingRequest,
        MSADeviceVerifingResponse
    },
    msa_device_authenticate::{
        MSADeviceAuthenticateRequest,
        MSADeviceAuthenticateReponse
    },
    msa_device_flow_error::{
        MSADeviceFlowError,
        MSADeviceFlowErrorResponse
    },
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