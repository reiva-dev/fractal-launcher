//! Copyright 2022 - Fractal Launcher - ReiRokusanami

mod request;

mod device_flow;
mod xboxlive;

pub use self::{
    device_flow::{
        verifing::MSADeviceVerifingResponse,
        authenticate::MSADeviceAuthenticateReponse,
        error::{MSADeviceFlowError, MSADeviceFlowErrorResponse},
    },
    xboxlive::{
        user_authenticate::XboxLiveUserAuthenticateResponse,
        xsts_authorize::XBoxLiveSTSAuthorizeResponse,
    },

    request::{
        msa_device_verifing::{
            MSADeviceVerifingRequest,
            VerifingRequestRejection
        },
        msa_device_authenticate::{
            MSADeviceAuthenticateRequest,
            MSADeviceAuthenticateRefresh,
            DeviceAuthenticateRejection
        },
        xbl_user_authenticate::{
            XboxLiveUserAuthenticateRequest,
            XboxLiveUserAuthenticateRejection
        },
        xbl_xsts_authorize::{
            XBoxLiveSTSAuthorizeRequest,
            XBoxLiveSTSAuthorizeRejection
        }
    }
};