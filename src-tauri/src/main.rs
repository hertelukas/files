// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod db;
use config::Config;
use db::Database;
use log::info;
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::{Manager, State};

#[tauri::command]
fn load_config(
    config_state: State<Mutex<Option<Config>>>,
    db_state: State<Mutex<Database>>,
) -> Result<Config, String> {
    let config_opt = match config_state.lock() {
        Ok(content) => content,
        Err(_) => return Err("Locking the config mutex failed.".to_string()),
    };
    if let Some(config) = config_opt.clone() {
        let db = match db_state.lock() {
            Ok(content) => content,
            Err(_) => return Err("Locking the db mutex failed.".to_string()),
        };
        assert!(db.is_initialized());
        db.check_config_consistency(&config)?;
        Ok(config)
    } else {
        return Err("No config exists yet.".to_string());
    }
}

#[tauri::command]
fn store_config(
    config: Config,
    config_state: State<Mutex<Option<Config>>>,
    db_state: State<Mutex<Database>>,
) -> Result<(), String> {
    info!("Storing new config: {0:?}", config);
    let mut db = match db_state.lock() {
        Ok(content) => content,
        Err(_) => return Err("Locking the db mutex failed.".to_string()),
    };

    if !db.is_initialized() {
        match db.open(config.db_path()) {
            Ok(_) => (),
            Err(error) => return Err("Opening database failed: ".to_string() + &error.to_string()),
        };
    }

    db.check_config_consistency(&config)?;
    config
        .store()
        .expect("Failed to store config. But the database has been updated!");
    let mut old_config = config_state.lock().unwrap();
    *old_config = Some(config);
    Ok(())
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
    env_logger::init();
    tauri::Builder::default()
        .setup(|app| {
            let mut db = Database::new();
            // If we can load a config, we will load the database too
            match Config::load() {
                Ok(config) => {
                    db.open(config.db_path())
                        .expect("Opening the database failed");
                    app.manage(Mutex::new(Some(config)));
                }
                Err(_error) => {
                    app.manage(Mutex::new(None::<Config>));
                }
            }
            app.manage(Mutex::new(db));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![load_config, store_config, import])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
