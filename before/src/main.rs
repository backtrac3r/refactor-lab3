use rand::Rng;
use sqlx::{Error as SqlxError, SqlitePool};
use std::sync::Arc;
use teloxide::sugar::request::RequestReplyExt;
use teloxide::{prelude::*, types::Message};

pub type AppErr = Box<dyn std::error::Error + Send + Sync>;
pub type HandlerResult = Result<(), AppErr>;

static REPLY_CHANCE: f64 = 0.25;

async fn init_db(pool: &SqlitePool) -> Result<(), SqlxError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS responses (
            id INTEGER PRIMARY KEY,
            text TEXT NOT NULL
        )",
    )
    .execute(pool)
    .await?;
    Ok(())
}

async fn get_random_response(pool: &SqlitePool) -> Option<String> {
    sqlx::query_scalar::<_, String>("SELECT text FROM responses ORDER BY RANDOM() LIMIT 1")
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
}

#[tokio::main]
async fn main() {
    let pool = Arc::new(SqlitePool::connect("sqlite:responses.db").await.unwrap());
    init_db(&pool).await.unwrap();
    let bot = Bot::new("7578470928:AAGQMhuEupWhiDup2gJW_YrbZwziCJ6g53M");

    let handler = Update::filter_message().endpoint(start_handler);

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .dependencies(dptree::deps![pool.clone()])
        .build()
        .dispatch()
        .await;
}

async fn start_handler(msg: Message, bot: Bot, pool: Arc<SqlitePool>) -> HandlerResult {
    if !(msg.chat.is_group() || msg.chat.is_supergroup()) {
        return Ok(());
    }

    let random_number: f64 = rand::thread_rng().r#gen();

    if random_number < REPLY_CHANCE {
        if let Some(response) = get_random_response(&pool).await {
            bot.send_message(msg.chat.id, response)
                .reply_to(msg.id)
                .await?;
        }
    }
    Ok(())
}
