// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
use config::Config;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn load_config() -> Result<Config, String> {
    Config::load()
}

#[tauri::command]
fn store_config(config: Config) -> Result<(), String> {
    match config.store() {
        Ok(_) => Ok(()),
        Err(_err) => Err("Storing failed.".to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, load_config, store_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
