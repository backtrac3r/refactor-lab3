use crate::{HandlerResult, REPLY_CHANCE, db::Db};
use rand::Rng;
use std::sync::Arc;
use teloxide::{prelude::*, sugar::request::RequestReplyExt};

pub async fn start_handler(msg: Message, bot: Bot, db: Arc<Db>) -> HandlerResult {
    if !(msg.chat.is_group() || msg.chat.is_supergroup()) {
        return Ok(());
    }

    let random_number: f64 = rand::thread_rng().r#gen();

    if random_number < REPLY_CHANCE {
        let mut tx = db.tx().await?;
        let response = Db::get_random_response(&mut tx).await?;
        tx.commit().await?;

        bot.send_message(msg.chat.id, response)
            .reply_to(msg.id)
            .await?;
    }

    Ok(())
}
