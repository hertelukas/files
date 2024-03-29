// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod db;
use config::Config;
use db::Database;
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
fn load_config(state: State<Mutex<Config>>) -> Result<Config, String> {
    let config = match state.lock() {
        Ok(content) => content,
        Err(_) => return Err("Locking the config mutex failed.".to_string()),
    };
    Ok(config.clone())
}

#[tauri::command]
fn store_config(config: Config, state: State<Mutex<Config>>) -> Result<(), String> {
    // TODO check that the new config is in a legal state
    let mut old_onfig = match state.lock() {
        Ok(content) => content,
        Err(_) => return Err("Locking the config mutex failed.".to_string()),
    };
    *old_onfig = config;
    match old_onfig.store() {
        Ok(_) => Ok(()),
        Err(_err) => Err("Storing failed.".to_string()),
    }
}

#[tauri::command]
fn import(
    path: String,
    tags: Vec<String>,
    categories: HashMap<String, String>,
) -> Result<(), String> {
    println!("Path: {:?}", path);
    println!("Tags: {:?}", tags);
    println!("Categories: {:?}", categories);
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(Config::load().expect("Could not load config.")))
        .manage(Mutex::new(Database::open()))
        .invoke_handler(tauri::generate_handler![load_config, store_config, import])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
