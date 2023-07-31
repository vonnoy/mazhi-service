use sqlx::query;
use std::error::Error;

use mazhi_service::storage::fetch_database_pool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = fetch_database_pool().await;
    query(
        "CREATE TABLE user (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE
        );",
    )
    .execute(&pool)
    .await?;
    Ok(())
}
