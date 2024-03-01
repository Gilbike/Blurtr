// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{PhysicalSize, Size};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![export_blurt])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn export_blurt(handle: tauri::AppHandle) {
  let window = tauri::WindowBuilder::new(
    &handle,
    "external", /* the unique window label */
    tauri::WindowUrl::App("/export".parse().unwrap())
  ).build().unwrap();

  window.set_skip_taskbar(true).unwrap();
  window.set_resizable(false).unwrap();
  window.set_size(Size::Physical(PhysicalSize { width: 600, height: 800 })).unwrap();
  window.set_title("Blurt exportálása").unwrap();
}