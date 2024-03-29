use dirs;
use rusqlite::{params, Connection, Result};
use std::path::PathBuf;

struct File {
    path: String,
}

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn open() -> Database {
        let con = Connection::open(Database::db_path()).expect("Failed to open database");
        let db = Database { connection: con };
        db.create_tables().expect("Failed to generate tables");
        db
    }

    fn db_path() -> PathBuf {
        if let Some(config_dir) = dirs::config_dir() {
            config_dir.join("files").join("files.sqlite")
        } else {
            panic!("Config directory not found")
        }
    }

    fn create_tables(&self) -> Result<()> {
        let qry = "
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS files (
  id INTEGER PRIMARY KEY,
  path TEXT NOT NULL
);

CREATE TABLE tags (
  id INTEGER PRIMARY KEY,
  tag TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS fileTags (
  file_id INTEGER,
  tag_id INTEGER,
  CONSTRAINT fk_file FOREIGN KEY (file_id) REFERENCES files(id) ON UPDATE CASCADE ON DELETE CASCADE,
  CONSTRAINT fk_tag FOREIGN KEY (tag_id) REFERENCES tags(id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS categories (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS categoryValue (
  category_id INTEGER,
  value TEXT NOT NULL,
  id TEXT GENERATED ALWAYS AS (concat(value, category_id)) VIRTUAL UNIQUE,
  CONSTRAINT fk_category FOREIGN KEY (category_id) REFERENCES categories(id) ON UPDATE CASCADE ON DELETE CASCADE,
  CONSTRAINT cv PRIMARY KEY (category_id, value)
);


CREATE TABLE IF NOT EXISTS fileValues (
  file_id INTEGER,
  value_id TEXT,
  CONSTRAINT fk_file FOREIGN KEY (file_id) REFERENCES files(id) ON UPDATE CASCADE ON DELETE CASCADE,
  CONSTRAINT fk_value FOREIGN KEY (value_id) REFERENCES categoryValue(id) ON UPDATE CASCADE ON DELETE CASCADE
);";
        self.connection.execute_batch(qry)
    }
}
