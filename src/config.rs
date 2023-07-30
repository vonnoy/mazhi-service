#[derive(Debug)]
pub struct SQLiteConfig {
    pub database_path: String,
}

impl SQLiteConfig {
    pub fn new(path: &str) -> Self {
        SQLiteConfig {
            database_path: path.to_string(),
        }
    }
}
