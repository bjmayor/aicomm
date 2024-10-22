// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
mod commands;
mod config;
mod utils;

use std::sync::Arc;

use arc_swap::ArcSwap;
use commands::{get_app_dir, get_config, greet};
use config::AppConfig;
use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem, SubmenuBuilder},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    webview, App, AppHandle, Manager, Runtime, Webview, WebviewUrl, WebviewWindowBuilder, Window,
    WindowEvent,
};
use tauri_plugin_log::{Target, TargetKind};
use tracing::{debug, info};
use utils::log_dir;

const APP_NAME: &str = "chatapp";
pub struct AppState {
    config: Arc<ArcSwap<AppConfig>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(logger().build())
        .invoke_handler(tauri::generate_handler![greet, get_app_dir, get_config])
        .setup(setup)
        .on_page_load(page_load_handler)
        .on_window_event(window_event_handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up the app");

    let state = AppState {
        config: Arc::new(ArcSwap::from_pointee(AppConfig::try_new()?)),
    };
    app.manage(state);
    let handle = app.handle();
    #[cfg(desktop)]
    {
        handle.plugin(tauri_plugin_window_state::Builder::default().build())?;
    }

    setup_menu(app)?;

    let mut builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default());
    #[cfg(desktop)]
    {
        builder = builder
            .title("Chat App")
            .user_agent(&format!("Chat app - {}", std::env::consts::OS))
            .inner_size(1200., 800.)
            .min_inner_size(800., 600.)
            .resizable(true)
            .content_protected(true);
    }
    #[allow(unused_variables)]
    let webview = builder.build()?;

    #[cfg(debug_assertions)]
    {
        webview.open_devtools();
    }
    Ok(())
}

fn page_load_handler(_window: &Webview, payload: &webview::PageLoadPayload<'_>) {
    info!("Page loaded: {:?}", payload.url());
}

fn window_event_handler(window: &Window, event: &tauri::WindowEvent) {
    debug!("Window event: {:?}", event);
    if let WindowEvent::CloseRequested { api, .. } = event {
        if window.label() == "main" {
            api.prevent_close();
            window.hide().unwrap()
        }
    }
}
fn logger() -> tauri_plugin_log::Builder {
    tauri_plugin_log::Builder::default()
        .targets([
            Target::new(TargetKind::Webview),
            Target::new(TargetKind::Folder {
                path: log_dir(),
                file_name: None,
            }),
            Target::new(TargetKind::Stdout),
        ])
        .level(tracing::log::LevelFilter::Info)
}

fn setup_menu<R: Runtime>(app: &App<R>) -> Result<(), tauri::Error> {
    let icon = app.default_window_icon().unwrap().clone();
    // add submenus
    let file_menu = SubmenuBuilder::with_id(app, "file", "File")
        .item(&MenuItem::with_id(
            app,
            "open",
            "Open",
            true,
            Some("CmdOrCtrl+O"),
        )?)
        .item(&MenuItem::with_id(
            app,
            "save",
            "Save",
            true,
            Some("CmdOrCtrl+S"),
        )?)
        .separator()
        .quit()
        .build()?;
    let edit_menu = SubmenuBuilder::with_id(app, "edit", "Edit")
        .item(&MenuItem::with_id(
            app,
            "process",
            "Process",
            true,
            Some("CmdOrCtrl+P"),
        )?)
        .separator()
        .undo()
        .redo()
        .separator()
        .cut()
        .copy()
        .paste()
        .separator()
        .select_all()
        .item(&CheckMenuItem::with_id(
            app,
            "checkme",
            "Check Me",
            true,
            true,
            None::<&str>,
        )?)
        .build()?;
    let tray_menu = SubmenuBuilder::with_id(app, "tray", "Tray")
        .item(&MenuItem::with_id(app, "open", "Open", true, None::<&str>)?)
        .item(&MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?)
        .separator()
        .quit()
        .build()?;

    TrayIconBuilder::with_id(format!("{}-tray", APP_NAME))
        .tooltip("Chat App")
        .icon(icon)
        .menu(&tray_menu)
        .menu_on_left_click(true)
        .on_tray_icon_event(|tray, event| {
            // info!("Tray icon event: {:?}", event);
            if let TrayIconEvent::Click {
                button: MouseButton::Right,
                ..
            } = event
            {
                open_main(tray.app_handle()).unwrap();
            }
        })
        .build(app)?;

    let menu = Menu::with_items(app, &[&file_menu, &edit_menu])?;
    app.set_menu(menu)?;
    app.on_menu_event(|app, event| {
        info!("Menu event: {:?}", event);
        match event.id.as_ref() {
            "open" => open_main(app).unwrap(),
            "save" => {}
            "saveas" => {}
            "process" => {}
            "checkme" => {
                // toggle checkme status and update config and runtime state
                // for runtime state - Arc<Mutex<State>> / ArcSwap
            }
            _ => {}
        }
    });
    Ok(())
}

fn open_main<R: Runtime>(handle: &AppHandle<R>) -> Result<(), tauri::Error> {
    handle
        .get_webview_window("main")
        .ok_or_else(|| tauri::Error::WindowNotFound)?
        .show()?;
    Ok(())
}
