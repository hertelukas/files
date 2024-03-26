use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Category {
    name: String,
    values: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    folder: String,
    save_date: bool,
    categories: Vec<Category>,
    tags: Vec<String>,
}
