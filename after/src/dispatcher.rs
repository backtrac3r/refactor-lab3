use crate::{db::Db, handlers::start_handler};
use std::sync::Arc;
use teloxide::{Bot, dispatching::UpdateFilterExt, dptree, prelude::Dispatcher, types::Update};

pub async fn run(bot: Bot, db: Arc<Db>) {
    let handler = Update::filter_message().endpoint(start_handler);

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .dependencies(dptree::deps![db])
        .build()
        .dispatch()
        .await;
}
