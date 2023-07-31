use chrono::prelude::*;
use sqlx::{query, sqlite::SqliteConnectOptions, Error, SqlitePool};

use crate::models::Horse;

pub async fn fetch_database_pool() -> SqlitePool {
    let database_url = "mazhi.db";
    let options = SqliteConnectOptions::new().filename(database_url);
    let pool = SqlitePool::connect_with(options)
        .await
        .expect("获取数据库连接池失败!");
    pool
}

#[allow(unused)]
pub struct HorseModel {
    pool: SqlitePool,
}

impl HorseModel {
    async fn new() -> Self {
        HorseModel {
            pool: fetch_database_pool().await,
        }
    }

    async fn create(&self, horse: Horse) -> Result<Horse, Error> {
        query(
            "INSERT INTO horse (
            name,
            images,
            breed,
            birth,
            info,
            status_health,
            status_activity,
            features,
            tags,
            special,
            manager_user_id,
            look_number,
            use_number,
            created_at,
            updated_at,
            is_del 
        )
        VALUES
            (
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?,
                ?
            );",
        )
        .bind(horse.name.clone())
        .bind(horse.images.clone())
        .bind(horse.breed.clone())
        .bind(horse.birth as i64)
        .bind(horse.info.clone())
        .bind(horse.status_health)
        .bind(horse.status_activity)
        .bind(horse.features.clone())
        .bind(horse.tags.clone())
        .bind(horse.special.clone())
        .bind(horse.manager_user_id)
        .bind(horse.look_number as i64)
        .bind(horse.use_number as i64)
        .bind(Utc::now().timestamp())
        .bind(Utc::now().timestamp())
        .bind(horse.is_del)
        .execute(&self.pool)
        .await?;
        Ok(horse)
    }

    fn delete(&self, id: i64) {}

    fn update(&self, id: i64) -> Option<Horse> {
        None
    }

    fn query_one(&self, id: i64) -> Option<Horse> {
        None
    }

    fn query_all(&self) -> Option<Vec<Horse>> {
        None
    }
}
