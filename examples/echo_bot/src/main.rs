//! This example shows how to create a simple echo bot, which will repeat all messages, which it receives.
//!
//! You can run this example by setting `BOT_TOKEN` and optional `RUST_LOG` environment variable and running:
//! ```bash
//! cd examples
//! RUST_LOG={log_level} BOT_TOKEN={your_bot_token} cargo run --bin echo_bot
//! ```

use telers::{
    enums::UpdateType,
    event::{telegram::HandlerResult, EventReturn, ToServiceProvider as _},
    methods::CopyMessage,
    types::Message,
    Bot, Dispatcher, Router,
};

async fn echo_handler(bot: Bot, message: Message) -> HandlerResult {
    bot.send(
        &CopyMessage::new(message.chat.id, message.chat.id, message.message_id),
        None,
    )
    .await?;

    Ok(EventReturn::Finish)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    pretty_env_logger::init();

    let Ok(bot_token) = std::env::var("BOT_TOKEN") else {
        panic!("BOT_TOKEN env variable is not set!");
    };

    let bot = Bot::new(bot_token);

    let mut router = Router::new("main");
    router.message.register(echo_handler);

    let dispatcher = Dispatcher::builder()
        .main_router(router)
        .bot(bot)
        .allowed_update(UpdateType::Message)
        .build();

    match dispatcher
        .to_service_provider_default()
        .unwrap()
        .run_polling()
        .await
    {
        Ok(_) => log::info!("Bot stopped"),
        Err(err) => log::error!("Bot stopped with error: {err}"),
    }
}
