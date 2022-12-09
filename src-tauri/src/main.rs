#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Duration;

use entities::msa_device_authenticate::MSADeviceAuthenticateReponse;
use entities::msa_device_flow_error::{MSADeviceFlowErrorResponse, MSADeviceFlowError};
use entities::msa_device_verifing::MSADeviceVerifingResponse;
use reqwest::StatusCode;
use tokio::task::JoinHandle;
use tauri::{Manager, AppHandle, ClipboardManager};
use tracing_subscriber::Layer;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

mod database;
mod entities;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn login_msa(handler: AppHandle) {
    let client = reqwest::Client::new();
    let url = "https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode"
            .parse::<url::Url>()
            .unwrap();
    let response = client.post(url)
        .form(&[
            ("client_id", "3ea1cbe9-4e3a-4a2f-85e7-cca409b6a8ca"),
            ("scope", "XboxLive.signin offline_access")
        ])
        .send().await.unwrap();

    let response = response.json::<MSADeviceVerifingResponse>().await.unwrap();

    let ms_view = tauri::WindowBuilder::new(&handler, "mslogin", tauri::WindowUrl::External(
        response.verification_uri().parse::<url::Url>().unwrap()))
        .enable_clipboard_access()
        .build()
        .unwrap();

    ms_view.set_title(&format!("Paste your verification code. (effective time {}s)", response.expires_in())).unwrap();

    handler.clipboard_manager()
        .write_text(response.user_code())
        .expect("cannot write `user_code`");

    let polling: JoinHandle<Result<_, MSADeviceFlowPollingError>> = tokio::task::spawn(async move {
        let mut retry = 0;
        loop {
            tokio::time::sleep(Duration::from_secs(response.interval() as u64)).await;
            let req = client.post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
            .form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
                ("client_id", "3ea1cbe9-4e3a-4a2f-85e7-cca409b6a8ca"),
                ("device_code", response.device_code())
            ])
            .send().await
            .map_err(MSADeviceFlowPollingError::Unknown)?;

            if req.status() != StatusCode::OK {
                match req.json::<MSADeviceFlowErrorResponse>().await.map_err(MSADeviceFlowPollingError::Unknown)?.into() {
                    MSADeviceFlowError::AuthorizationPending => {
                        retry += 1;
                        let remain = response.expires_in() - (response.interval() * retry);
                        tracing::debug!("(remain {}s) The user has not yet approved the authorization", remain);
                        ms_view.set_title(&format!("Paste your verification code. (effective time {}s)", remain)).unwrap();
                        continue;
                    },
                    MSADeviceFlowError::AuthorizeDeclined => break Err(MSADeviceFlowPollingError::AuthorizeDeclined),
                    MSADeviceFlowError::BadVerificationCode => break Err(MSADeviceFlowPollingError::BadVerificationCode),
                    MSADeviceFlowError::ExpiredToken => break Err(MSADeviceFlowPollingError::ExpiredToken(response.expires_in())),
                    _ => unimplemented!()
                }
            }

            let authed = req.json::<MSADeviceAuthenticateReponse>().await
                .map_err(MSADeviceFlowPollingError::Unknown)?;

            break Ok(authed);
        }
    });

    match polling.await.expect("bugged") {
        Ok(_state) => {
            tracing::info!("(√) MSA verified.")
        },
        Err(reason) => {
            tracing::error!("{}", reason);
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MSADeviceFlowPollingError {
    #[error("User denied authorization.")]
    AuthorizeDeclined,
    #[error("`device_code` is wrong. This may be a bug.")]
    BadVerificationCode,
    #[error("`device_code` disabled due to exceeding `expires_in` seconds (It was set to {0} seconds.). Redo.")]
    ExpiredToken(i32),
    #[error("Unknown Error occured {0:#?}")]
    Unknown(#[from] reqwest::Error)
}

#[tokio::main]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    dotenvy::dotenv().ok();
    dotenvy::from_path("../.env").ok();
    dotenvy::from_path("../local.env").ok();
    let appender = tracing_appender::rolling::daily(std::path::Path::new("../logs/"), "debug.log");
    let (non_blocking_appender, _guard) = tracing_appender::non_blocking(appender);
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer()
            .with_filter(tracing_subscriber::filter::LevelFilter::TRACE))
        .with(tracing_subscriber::fmt::Layer::default()
            .with_writer(non_blocking_appender)
            .with_ansi(false)
            .with_filter(tracing_subscriber::filter::LevelFilter::TRACE))
        .init();
    
    tracing::info!("setup local database");
    database::sqlite::migration().await
        .expect("cannot migration database");
    let _pool = database::sqlite::connect().await
        .expect("cannot construct connection pool");

    tracing::info!("run tauri");

    let quit = tauri::CustomMenuItem::new(SystemTrayMenuSignal::Quit, "Quit Fractal Launcher");
    let menu = tauri::SystemTrayMenu::new().add_item(quit);
    let tray = tauri::SystemTray::new()
        .with_menu(menu);

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(move |app, event| match event {
            tauri::SystemTrayEvent::LeftClick { .. } => {
                let viw = app.get_window("main").unwrap();
                viw.show().map_err(|e| tracing::error!("{:#?}", e)).unwrap();
            },
            tauri::SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.into() {
                    SystemTrayMenuSignal::Quit => {
                        tracing::info!("app shutdown signal received.");
                        app.exit(0);
                    }
                }
            },
            _ => ()
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            login_msa
        ])
        .run(tauri::generate_context!())
        .expect("error tauri handler.");
}

#[derive(Debug, Clone)]
pub enum SystemTrayMenuSignal {
    Quit
}

impl From<SystemTrayMenuSignal> for String {
    fn from(sig: SystemTrayMenuSignal) -> Self {
        #[allow(unreachable_patterns)]
        match sig {
            SystemTrayMenuSignal::Quit => "quit".to_owned(),
            _ => unimplemented!()
        }
    }
}

impl From<String> for SystemTrayMenuSignal {
    fn from(signal: String) -> Self {
        match signal.as_str() {
            "quit" => SystemTrayMenuSignal::Quit,
            _ => unimplemented!()
        }
    }
}