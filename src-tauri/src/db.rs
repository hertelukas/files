use crate::config::Category;

use super::Config;
use log::{debug, info, warn};
use rusqlite::{params, Connection, Result};
use std::path::PathBuf;

#[derive(Debug)]
struct CategoryEntry {
    id: u32,
    name: String,
}

impl PartialEq for CategoryEntry {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}

pub struct Database {
    connection: Option<Connection>,
}

impl Database {
    pub fn new() -> Self {
        Database { connection: None }
    }

    pub fn open(&mut self, path: PathBuf) -> Result<()> {
        info!("Opening database at {0}", path.display());
        self.connection = match Connection::open(path) {
            Ok(con) => Some(con),
            Err(err) => {
                return Err(err);
            }
        };
        if self.is_initialized() {
            self.create_tables().expect("Failed to generate tables");
        }
        Ok(())
    }

    pub fn is_initialized(&self) -> bool {
        self.connection.is_some()
    }

    fn get_tags(&self) -> Result<Vec<String>> {
        if let Some(ref con) = self.connection {
            con.prepare("SELECT tag FROM tags")
                .expect("Tag select failed")
                .query_map([], |row| row.get(0))?
                .collect::<Result<Vec<String>>>()
        } else {
            // TODO not very clean (has to be chanaged eveywhere)
            Err(rusqlite::Error::InvalidQuery)
        }
    }

    fn delete_tag(&self, tag: &String) -> Result<usize> {
        if let Some(ref con) = self.connection {
            con.execute("DELETE FROM tags WHERE tag = ?1", params![tag])
        } else {
            Err(rusqlite::Error::InvalidQuery)
        }
    }

    fn insert_tag(&self, tag: &String) -> Result<usize> {
        if let Some(ref con) = self.connection {
            con.execute("INSERT INTO tags(tag) VALUES (?1)", params![tag])
        } else {
            Err(rusqlite::Error::InvalidQuery)
        }
    }

    fn tag_consistency(&self, config_tags: &Vec<String>) -> Result<()> {
        let tags = self.get_tags()?;

        // Delete unused tags
        for tag in tags.iter() {
            if !config_tags.contains(tag) {
                debug!("Deleting tag {}", tag);
                self.delete_tag(tag)?;
            }
        }

        // Insert new tags
        for tag in config_tags.iter() {
            if !tags.contains(tag) {
                debug!("Inserting tag {}", tag);
                self.insert_tag(tag)?;
            }
        }
        Ok(())
    }

    fn get_categories(&self) -> Result<Vec<CategoryEntry>> {
        if let Some(ref con) = self.connection {
            con.prepare("SELECT id, name FROM categories")?
                .query_map([], |row| {
                    Ok(CategoryEntry {
                        id: row.get(0)?,
                        name: row.get(1)?,
                    })
                })?
                .collect::<Result<Vec<CategoryEntry>>>()
        } else {
            Err(rusqlite::Error::InvalidQuery)
        }
    }

    fn get_category_id(&self, name: &String) -> Result<u32> {
        if let Some(ref con) = self.connection {
            con.query_row(
                "SELECT id FROM categories WHERE name = ?1",
                params![name],
                |r| r.get(0),
            )
        } else {
            Err(rusqlite::Error::InvalidQuery)
        }
    }

    fn delete_category(&self, id: u32) -> Result<usize> {
        if let Some(ref con) = self.connection {
            con.execute("DELETE FROM categories WHERE id = ?1", params![id])
        } else {
            Err(rusqlite::Error::InvalidQuery)
        }
    }

    fn insert_category(&self, name: &String) -> Result<usize> {
        if let Some(ref con) = self.connection {
            con.execute("INSERT INTO categories(name) VALUES (?1)", params![name])
        } else {
            Err(rusqlite::Error::InvalidQuery)
        }
    }

    fn delete_value(&self, category_id: u32, value: &String) -> Result<usize> {
        if let Some(ref con) = self.connection {
            con.execute(
                "DELETE FROM categoryValue WHERE value = ?1 and category_id = ?2",
                params![value, category_id],
            )
        } else {
            Err(rusqlite::Error::InvalidQuery)
        }
    }

    fn insert_value(&self, category_id: u32, value: &String) -> Result<usize> {
        if let Some(ref con) = self.connection {
            con.execute(
                "INSERT INTO categoryValue(category_id, value) VALUES (?1, ?2)",
                params![category_id, value],
            )
        } else {
            Err(rusqlite::Error::InvalidQuery)
        }
    }

    fn category_consistency(&self, config_cats: &Vec<Category>) -> Result<()> {
        let categories = self.get_categories()?;
        for category in categories.iter() {
            // Delete unused categories
            if !config_cats.iter().any(|c| c.name.eq(&category.name)) {
                debug!("Removing category {}", category.name);
                self.delete_category(category.id)?;
            }
        }

        // Insert new categories
        for config_category in config_cats {
            debug!("Checking category {}", config_category.name);
            if !categories.iter().any(|c| c.name.eq(&config_category.name)) {
                debug!("Inserting category {}", config_category.name);
                self.insert_category(&config_category.name)?;

                let id: u32 = self.get_category_id(&config_category.name)?;

                for value in config_category.values.iter() {
                    debug!("Inserting value {}", value);
                    self.insert_value(id, value)?;
                }
            } else {
                // Here we can be sure that the category in the config also exists
                // in the database. We need to check if the values match
                if let Some(ref con) = self.connection {
                    let mut stmt = con.prepare("SELECT categoryValue.value FROM categoryValue JOIN categories ON categoryValue.category_id = categories.id WHERE categories.name = ?1")?;

                    // These are the values of the current category
                    let values: Vec<String> = stmt
                        .query_map(params![config_category.name], |row| Ok(row.get(0)?))?
                        .collect::<Result<Vec<String>>>()?;

                    let id: u32 = self.get_category_id(&config_category.name)?;

                    // Check if value needs to be deleted
                    for val in values.iter() {
                        if !config_category.values.iter().any(|c| c.eq(val)) {
                            debug!("Removing value {}", val);
                            self.delete_value(id, val)?;
                        }
                    }
                    for config_val in config_category.values.iter() {
                        if !values.iter().any(|c| c.eq(config_val)) {
                            debug!("Inserting value {}", config_val);
                            self.insert_value(id, config_val)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn check_config_consistency(&self, config: &Config) -> Result<(), String> {
        info!("Performing consistency check");
        let _ = self.tag_consistency(&config.tags).map_err(|err| {
            warn!("Updating tags failed: {err}");
            format!("Failed to update tags: {err}").to_string()
        });

        let _ = self
            .category_consistency(&config.categories)
            .map_err(|err| {
                warn!("Updating cateogires failed: {err}");
                format!("Failed to update categories: {err}").to_string()
            });

        Ok(())
    }

    pub fn store_file(&self, relative_path: &String, name: &String) -> Result<(), String> {
        if let Some(ref con) = self.connection {
            match con.execute(
                "INSERT INTO files(path, name) VALUES (?1, ?2)",
                params![relative_path, name],
            ) {
                Ok(updated) => info!("{} file(s) inserted", updated),
                Err(err) => return Err(format!("Failed to insert file: {err}").to_string()),
            }
        } else {
            return Ok(());
        };

        Ok(())
    }

    fn create_tables(&self) -> Result<()> {
        let qry = "
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS files (
  id INTEGER PRIMARY KEY,
  path TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL
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
        if let Some(ref con) = self.connection {
            con.execute_batch(qry)
        } else {
            Err(rusqlite::Error::InvalidQuery)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_db() -> Database {
        let con = Connection::open_in_memory().expect("Database in memory failed");
        let db = Database {
            connection: Some(con),
        };
        db.create_tables().expect("Database creation failed");
        db
    }

    #[test]
    fn tag_insert() {
        let db = create_db();
        assert_eq!(Vec::<String>::new(), db.get_tags().unwrap());
        db.insert_tag(&"test".to_string())
            .expect("Failed to insert tag");
        assert_eq!(vec!["test".to_string()], db.get_tags().unwrap());
    }

    #[test]
    #[should_panic(expected = "UNIQUE constraint failed")]
    fn tag_duplicate_insert() {
        let db = create_db();
        db.insert_tag(&"test".to_string())
            .expect("Failed to insert tag");
        db.insert_tag(&"test".to_string())
            .expect("Failed to insert tag");
    }

    #[test]
    fn tag_delete() {
        let db = create_db();
        db.insert_tag(&"test".to_string())
            .expect("Failed to insert tag");
        db.delete_tag(&"test".to_string())
            .expect("Failed to delete tag");
        assert_eq!(Vec::<String>::new(), db.get_tags().unwrap());
    }

    #[test]
    fn category_insert() {
        let db = create_db();
        assert_eq!(Vec::<CategoryEntry>::new(), db.get_categories().unwrap());
        db.insert_category(&"test".to_string())
            .expect("Failed to insert category");
        assert!(
            db.get_categories()
                .unwrap()
                .iter()
                .any(|c| c.name == "test".to_string()),
            "Does not contain the correct category"
        );
        assert_eq!(
            db.get_categories().unwrap().len(),
            1,
            "Does not contain exactly one category"
        );
    }

    #[test]
    fn category_delete() {
        let db = create_db();
        db.insert_category(&"test".to_string())
            .expect("Failed to insert category");
        let id = db
            .get_category_id(&"test".to_string())
            .expect("Failed to get category id");
        db.delete_category(id).expect("Failed to delete category");
        assert_eq!(Vec::<CategoryEntry>::new(), db.get_categories().unwrap());
    }

    #[test]
    #[should_panic(expected = "UNIQUE constraint failed")]
    fn category_duplicate_insert() {
        let db = create_db();
        db.insert_category(&"test".to_string())
            .expect("Failed to insert category");
        db.insert_category(&"test".to_string())
            .expect("Failed to insert category");
    }
}
