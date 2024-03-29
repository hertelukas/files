use dirs;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Category {
    name: String,
    values: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    folder: String,
    save_date: bool,
    categories: Vec<Category>,
    tags: Vec<String>,
}

impl Config {
    pub fn store(&self) -> std::io::Result<()> {
        let mut file = BufWriter::new(File::create(Config::config_path())?);

        file.write_all(serde_json::to_vec_pretty(self).unwrap().as_slice())?;
        file.flush()?;
        Ok(())
    }

    pub fn load() -> Result<Config, String> {
        let config = match fs::read_to_string(Config::config_path()) {
            Ok(content) => content,
            Err(error) => return Err(error.to_string()),
        };

        let config: Config = match serde_json::from_str(&config) {
            Ok(content) => content,
            Err(_error) => return Err("failed".to_string()),
        };

        Ok(config)
    }

    fn config_path() -> PathBuf {
        if let Some(config_dir) = dirs::config_dir() {
            config_dir.join("files").join("config.json")
        } else {
            panic!("Config directory not found.")
        }
    }
}
