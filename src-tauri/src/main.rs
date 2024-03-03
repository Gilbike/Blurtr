// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, PhysicalSize, Size};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![export_blurt, close_export])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn export_blurt(handle: tauri::AppHandle, params: String) {
  let url = format!("/export?data={params}");
  let window = tauri::WindowBuilder::new(
    &handle,
    "export", /* the unique window label */
    tauri::WindowUrl::App(url.parse().unwrap())
  ).build().unwrap();

  window.set_skip_taskbar(true).unwrap();
  window.set_resizable(false).unwrap();
  window.set_size(Size::Physical(PhysicalSize { width: 600, height: 800 })).unwrap();
  window.set_title("Blurt exportálása").unwrap();
}

#[tauri::command]
async fn close_export(handle: tauri::AppHandle) {
  let window = handle.get_window("export").unwrap();
  window.close().unwrap();
}