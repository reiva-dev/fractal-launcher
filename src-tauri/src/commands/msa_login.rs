use std::time::Duration;

use reqwest::StatusCode;
use sqlx::SqlitePool;
use tokio::task::JoinHandle;
use tauri::{AppHandle, State, ClipboardManager, Manager};

use crate::{entities::auth::{
    MSADeviceVerifingResponse,
    MSADeviceAuthenticateReponse,
    MSADeviceFlowError, 
    MSADeviceFlowErrorResponse,
    XboxLiveUserAuthenticateRequest,
    XboxLiveUserAuthenticateResponse,
    XBoxLiveSTSAuthorizeRequest,
    XBoxLiveSTSAuthorizeResponse,
    FlowSteppable
}, api::http::Requestor};


#[tauri::command]
pub async fn login(handler: AppHandle) {
    let _pool: State<'_, SqlitePool> = handler.state();
    let mut clipboard = handler.clipboard_manager();

    let res = Requestor::new(MSADeviceVerifingResponse::default())
        .execute().await.unwrap()
        .map_future(|res| res.json::<MSADeviceVerifingResponse>())
        .await
        .unwrap();


    let ms_view = tauri::WindowBuilder::new(&handler, "mslogin", tauri::WindowUrl::External(
            res.verification_uri().parse::<url::Url>().unwrap()))
        .enable_clipboard_access()
        .disable_file_drop_handler()
        .resizable(false)
        .always_on_top(true)
        .build()
        .unwrap();

    clipboard.write_text(res.user_code()).unwrap();

    ms_view.set_title(&format!("Paste your verification code. (effective time {}s)", res.expires_in())).unwrap();

    let interval = res.interval();
    let expired_in = res.expires_in();

    let polling: JoinHandle<Result<_, MSADeviceFlowPollingError>> = tokio::task::spawn(async move {
        let mut retry = 0;
        loop {
            tokio::time::sleep(Duration::from_secs(interval as u64)).await;

            let req = Requestor::synthetic(MSADeviceAuthenticateReponse::default())
                .execute_sythetic(res.device_code().to_string())
                .await
                .map_err(|_| MSADeviceFlowPollingError::Unknown)?;

            if req.peeking(|inner| inner.status()) != StatusCode::OK {
                match req.map_future(|res| res.json::<MSADeviceFlowErrorResponse>())
                    .await.map_err(MSADeviceFlowPollingError::Request)?.into() {
                    MSADeviceFlowError::AuthorizationPending => {
                        retry += 1;
                        let remain = expired_in - (interval * retry);
                        tracing::debug!("(remain {}s) The user has not yet approved the authorization", remain);
                        ms_view.set_title(&format!("Paste your verification code. (effective time {}s)", remain)).unwrap();
                        continue;
                    },
                    MSADeviceFlowError::AuthorizeDeclined => break Err(MSADeviceFlowPollingError::AuthorizeDeclined),
                    MSADeviceFlowError::BadVerificationCode => break Err(MSADeviceFlowPollingError::BadVerificationCode),
                    MSADeviceFlowError::ExpiredToken => break Err(MSADeviceFlowPollingError::ExpiredToken(expired_in)),
                    _ => unimplemented!()
                }
            }

            let authed = req.map_future(|res| res.json::<MSADeviceAuthenticateReponse>())
                .await
                .map_err(MSADeviceFlowPollingError::Request)?;

            break Ok(authed);
        }
    });

    let msa_cert = match polling.await.expect("bugged") {
        Ok(authed) => {
            tracing::info!("(√) MSA verified.");
            clipboard.write_text("").expect("cannot clearing clipboard");
            authed
        },
        Err(reason) => {
            tracing::error!("(X) {}", reason);
            return;
        }
    };

    let req = XboxLiveUserAuthenticateRequest::default().step(msa_cert);

    let xsts_token = Requestor::new(req)
        .execute()
        .await
        .unwrap();

    let xsts_token = match xsts_token.map_future(|xsts| xsts.json::<XboxLiveUserAuthenticateResponse>()).await {
        Ok(authed) => {
            tracing::info!("(√) Xbox Secure Token Service (XSTS) authorized.");
            authed
        },
        Err(reason) => {
            tracing::error!("(X) Failed authorizing Xbox Secure Token Service (XSTS)!");
            tracing::error!("{:?}", reason);
            return;
        }
    };

    let req = XBoxLiveSTSAuthorizeRequest::default().step(xsts_token);

    let mc_user_token = Requestor::new(req)
        .execute()
        .await
        .unwrap();

    let _mc_user_token = match mc_user_token.map_future(|token| token.json::<XBoxLiveSTSAuthorizeResponse>()).await {
        Ok(authed) => {
            tracing::info!("(√) Minecraft user authorized with XSTS.");
            authed
        },
        Err(reason) => {
            tracing::error!("(X) Failed authorizing Minecraft user with XSTS");
            tracing::error!("{:?}", reason);
            return;
        }
    };
}

#[derive(Debug, thiserror::Error)]
pub enum MSADeviceFlowPollingError {
    #[error("User denied authorization.")]
    AuthorizeDeclined,
    #[error("`device_code` is wrong. This may be a bug.")]
    BadVerificationCode,
    #[error("`device_code` disabled due to exceeding `expires_in` seconds (It was set to {0} seconds.). Redo.")]
    ExpiredToken(i32),
    #[error("Request error occured {0:#?}")]
    Request(#[from] reqwest::Error),
    #[error("Unknown Error occured.")]
    Unknown
}