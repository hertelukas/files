// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dirs;
use std::fs;
use serde_json;

mod config;
use config::Config;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn load_config() -> Result<Config, String> {
    if let Some(config_dir) = dirs::config_dir() {

        let config_file_path = config_dir.join("files").join("config.json");

        let config = match fs::read_to_string(config_file_path) {
            Ok(content) => content,
            Err(error) => return Err(error.to_string()),
        };

        let config: Config = match serde_json::from_str(&config) {
            Ok(content) => content,
            Err(_error) => return Err("failed".to_string()),
        };

        Ok(config)
    }
    else {
        Err("Config directory not found.".to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, load_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

