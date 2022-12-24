use std::time::Duration;

use once_cell::sync::Lazy;
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

pub fn http_client() -> &'static reqwest::Client {
    static HTTP_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
        reqwest::Client::new()
    });
    &HTTP_CLIENT
}

#[tauri::command]
pub async fn login(handler: AppHandle) {
    let _pool: State<'_, SqlitePool> = handler.state();
    let mut clipboard = handler.clipboard_manager();

    // old
    let client = http_client();

    // new
    let res = Requestor::new(MSADeviceVerifingResponse::default())
        .execute().await.unwrap()
        .map_future(|res| res.json::<MSADeviceVerifingResponse>())
        .await
        .unwrap();

    let ms_view = tauri::WindowBuilder::new(&handler, "mslogin", tauri::WindowUrl::External(
        res.verification_uri().parse::<url::Url>().unwrap()))
        .enable_clipboard_access()
        .build()
        .unwrap();

    ms_view.set_title(&format!("Paste your verification code. (effective time {}s)", res.expires_in())).unwrap();

    clipboard.write_text(res.user_code())
        .expect("cannot write `user_code`");

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

    clipboard.write_text("")
        .expect("cannot clearing clipboard");

    let msa_cert = match polling.await.expect("bugged") {
        Ok(authed) => {
            tracing::info!("(√) MSA verified.");
            authed
        },
        Err(reason) => {
            tracing::error!("(X) {}", reason);
            return;
        }
    };

    let req = XboxLiveUserAuthenticateRequest::default().step(msa_cert);

    let xsts_token = client.post("https://user.auth.xboxlive.com/user/authenticate")
        .json(&req)
        .send()
        .await.unwrap();

    let xsts_token = match xsts_token.json::<XboxLiveUserAuthenticateResponse>().await {
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

    let mc_user_token = client.post("https://xsts.auth.xboxlive.com/xsts/authorize")
        .json(&req)
        .send()
        .await.unwrap();

    let _mc_user_token = match mc_user_token.json::<XBoxLiveSTSAuthorizeResponse>().await {
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