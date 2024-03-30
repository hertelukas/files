use crate::config::Category;

use super::Config;
use dirs;
use rusqlite::{params, Connection, Result};
use std::path::PathBuf;

struct File {
    path: String,
}

struct CategoryEntry {
    id: i32,
    name: String,
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

    fn tag_consistency(&self, config_tags: &Vec<String>) -> Result<()> {
        let mut stmt = self
            .connection
            .prepare("SELECT tag FROM tags")
            .expect("Tag select failed");
        let tag_rows = stmt.query_map([], |row| row.get(0))?;

        let mut tags: Vec<String> = Vec::new();
        for tag in tag_rows {
            tags.push(tag.unwrap());
        }

        // Delete unused tags
        for tag in &tags {
            if !config_tags.contains(&tag) {
                println!("Deleting tag {}", tag);
                self.connection
                    .execute("DELETE FROM tags WHERE tag = ?1", params![tag])?;
            }
        }

        // Insert new tags
        for tag in config_tags {
            if !tags.contains(tag) {
                println!("Inserting tag {}", tag);
                self.connection
                    .execute("INSERT INTO tags(tag) VALUES (?1)", params![tag])?;
            }
        }
        Ok(())
    }

    fn category_consistency(&self, config_cats: &Vec<Category>) -> Result<()> {
        let mut stmt = self.connection.prepare("SELECT id, name FROM categories")?;
        let categories: Vec<CategoryEntry> = stmt
            .query_map([], |row| {
                Ok(CategoryEntry {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            })?
            .collect::<Result<Vec<CategoryEntry>>>()?;

        for category in categories.iter() {
            // Delete unused categories
            if !config_cats.into_iter().any(|c| c.name.eq(&category.name)) {
                println!("Removing category {}", category.name);
                self.connection
                    .execute("DELETE FROM categories WHERE id = ?1", params![category.id])?;
            }
            // TODO also delete unused values of a category
            // and insert new values of a category
        }

        // Insert new categories
        for config_category in config_cats {
            if !categories
                .iter()
                .by_ref()
                .any(|c| c.name.eq(&config_category.name))
            {
                println!("Inserting category {}", config_category.name);
                self.connection.execute(
                    "INSERT INTO categories(name) VALUES (?1)",
                    params![config_category.name],
                )?;
                let id: u32 = self.connection.query_row(
                    "SELECT id FROM categories WHERE name = ?1",
                    params![config_category.name],
                    |r| r.get(0),
                )?;

                for value in config_category.values.iter() {
                    println!("Inserting value {}", value);
                    self.connection
                        .execute(
                            "INSERT INTO categoryValue(category_id, value) VALUES (?1, ?2)",
                            params![id, value],
                        )
                        .unwrap();
                }
            }
        }

        Ok(())
    }

    pub fn check_config_consistency(&self, config: &Config) -> Result<(), String> {
        let _ = self
            .tag_consistency(&config.tags)
            .map_err(|err| format!("Failed to update tags: {err}").to_string());

        let _ = self
            .category_consistency(&config.categories)
            .map_err(|err| format!("Failed to update categories: {err}").to_string());

        Ok(())
    }

    fn create_tables(&self) -> Result<()> {
        let qry = "
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS files (
  id INTEGER PRIMARY KEY,
  path TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tags (
  id INTEGER PRIMARY KEY,
  tag TEXT NOT NULL UNIQUE
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
