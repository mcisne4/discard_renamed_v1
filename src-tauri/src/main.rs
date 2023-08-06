// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    match rs_db::DB::Settings.connect() {
        Ok(_db) => println!("Connected to Settings db"),
        Err(errs) => eprintln!("Error connecting to settings db:\n{:#?}", errs),
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
