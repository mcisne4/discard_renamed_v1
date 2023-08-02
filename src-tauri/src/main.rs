// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use rs_errors::{create_error, RenamedResult};

use serde::Serialize;
#[derive(Serialize)]
pub enum TestErr {
    Err01 { error: String, info: String },
}
#[derive(Serialize)]
pub struct Ex {
    pub error: String,
    pub info: String,
}

#[tauri::command]
fn err_test() -> RenamedResult<()> {
    Err(create_error(
        "Error 25634",
        "Description related to error: Error 25634",
    ))
}

fn main() {
    let data_dir = tauri::api::path::data_dir();
    rs_db::connection::connect(data_dir);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, err_test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
