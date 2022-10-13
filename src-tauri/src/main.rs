#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Manager, Runtime, WindowBuilder, WindowUrl};

#[tauri::command]
fn new_window<R: Runtime>(app_handle: AppHandle<R>,) -> tauri::Result<()> {
    let label = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to construct unix timestamp")
        .as_millis()
        .to_string();

    WindowBuilder::new(&app_handle, label, WindowUrl::App("index.html".into()))
        .min_inner_size(500.0, 500.0)
        .build()?;

    Ok(())
}

#[tauri::command]
fn emit_all<R: Runtime>(
    app_handle: AppHandle<R>,
    event: String,
    payload: Option<String>,
) -> tauri::Result<()> {
    app_handle.emit_all(&event, payload).map_err(Into::into)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![new_window, emit_all])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
