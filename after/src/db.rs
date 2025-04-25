use crate::AppErr;
use sqlx::{Sqlite, SqlitePool, Transaction, query};
use std::env;

pub struct Db {
    pool: SqlitePool,
}

impl Db {
    pub async fn new() -> Result<Self, AppErr> {
        let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
        Ok(Self { pool })
    }

    pub async fn tx(&self) -> Result<Transaction<'static, Sqlite>, AppErr> {
        Ok(self.pool.begin().await?)
    }

    pub async fn get_random_response(
        tx: &mut Transaction<'static, Sqlite>,
    ) -> Result<String, AppErr> {
        let r = query!("select msg from responses order by random() limit 1")
            .fetch_one(&mut **tx)
            .await?;

        Ok(r.msg)
    }
}
