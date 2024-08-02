mod api;
mod clients;
mod settings;

use lazy_static::lazy_static;
use settings::{load_settings, write_settings};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};
use tokio::runtime::Runtime;

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "full");
    tracing_subscriber::fmt::init();

    RUNTIME.spawn(async {
        api::init().await;
        tracing::debug!("Server stopped");
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .setup(|app| {
            let settings = load_settings();
            let autohide = settings.autohide.unwrap_or(false);
            let window_size = settings.window_size.unwrap_or((320, 600));

            let icon = app.default_window_icon().unwrap();

            let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let show = MenuItemBuilder::with_id("show", "Show").build(app)?;
            let hide = MenuItemBuilder::with_id("hide", "Hide").build(app)?;

            let menu = MenuBuilder::new(app)
                .items(&[&show, &hide])
                .separator()
                .items(&[&quit])
                .build()?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .icon(icon.clone())
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "show" => {
                        let window = app.get_webview_window("main").unwrap();
                        window.show().unwrap();
                    }
                    "hide" => {
                        let window = app.get_webview_window("main").unwrap();
                        window.hide().unwrap();
                    }
                    _ => (),
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(webview_window) = app.get_webview_window("main") {
                            let _ = webview_window.show();
                            let _ = webview_window.set_focus();
                        }
                    }
                })
                .build(app)?;

            let window = tauri::WebviewWindowBuilder::new(
                app,
                "main".to_string(),
                tauri::WebviewUrl::App("index.html".into()),
            )
            .resizable(true)
            .title("Home Remote")
            .inner_size(window_size.0.into(), window_size.1.into())
            .maximizable(false)
            .icon(icon.clone())?
            .visible(autohide)
            .build()?;
            window.clone().on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    let _ = api.prevent_close();
                    let _ = window.hide();

                    let size = window.inner_size().unwrap_or(tauri::PhysicalSize {
                        width: 320,
                        height: 600,
                    });

                    let mut settings = load_settings();
                    settings.window_size = Some((size.width, size.height));

                    write_settings(&settings);
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            api::mpv::handle_pause_cmd,
            api::mpv::handle_volume_down_cmd,
            api::mpv::handle_volume_up_cmd,
            settings::save_settings,
            settings::load_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
