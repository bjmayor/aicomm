use std::sync::Arc;

use tauri::{AppHandle, Manager};

use crate::{config::AppConfig, utils::app_dir, AppState};

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub async fn get_app_dir() -> String {
    app_dir().display().to_string()
}

#[tauri::command]
pub fn get_config(handle: AppHandle) -> Arc<AppConfig> {
    handle.state::<AppState>().config.load().clone()
}
