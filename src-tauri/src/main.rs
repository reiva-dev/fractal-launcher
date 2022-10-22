#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use tracing_subscriber::Layer;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();
    let appender = tracing_appender::rolling::daily(std::path::Path::new("./logs/"), "debug.log");
    let (non_blocking_appender, _guard) = tracing_appender::non_blocking(appender);
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer()
            .with_filter(tracing_subscriber::filter::LevelFilter::TRACE))
        .with(tracing_subscriber::fmt::Layer::default()
            .with_writer(non_blocking_appender)
            .with_ansi(false)
            .with_filter(tracing_subscriber::filter::LevelFilter::TRACE))
        .init();

    tracing::info!("run tauri");

    tauri::async_runtime::set(tokio::runtime::Handle::current());

    let quit = tauri::CustomMenuItem::new(SystemTrayMenuSignal::Quit, "Quit Nekomata");
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
            greet
        ])
        .run(tauri::generate_context!())
        .expect("tauri handler excp");
    Ok(())
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