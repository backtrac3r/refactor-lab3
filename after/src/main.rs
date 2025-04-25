mod db;
mod dispatcher;
mod handlers;

use db::Db;
use std::sync::Arc;
use teloxide::prelude::*;

pub type AppErr = Box<dyn std::error::Error + Send + Sync>;
pub type HandlerResult = Result<(), AppErr>;

static REPLY_CHANCE: f64 = 0.25;

#[tokio::main]
async fn main() -> Result<(), AppErr> {
    dotenvy::dotenv().unwrap();

    let db = Arc::new(Db::new().await?);
    let bot = Bot::from_env();

    dispatcher::run(bot, db).await;

    Ok(())
}
