// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
use std::collections::HashMap;

use config::Config;

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

#[tauri::command]
fn import(path: String, tags: Vec<String>, categories: HashMap<String, String>) -> Result<(), String> {
    println!("Path: {:?}", path);
    println!("Tags: {:?}", tags);
    println!("Categories: {:?}", categories);
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_config, store_config, import])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
