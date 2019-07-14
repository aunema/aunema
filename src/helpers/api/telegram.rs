use carapax::prelude::{
    Api, App, CommandsHandler, Config, Context, HandlerFuture, HandlerResult, Message, SendMessage,
};
use futures::Future;
use std::error::Error;

pub struct Telegram {
    pub api: Api,
    pub app: App,
    pub handler: CommandsHandler,
}

pub fn init_telegram(token: String) -> Result<Telegram, Box<dyn Error>> {
    let config = Config::new(token);
    let api = Api::new(config)?;
    let app = App::new();
    let handler = CommandsHandler::default().not_found_handler(handle_not_found);
    Ok(Telegram { api, app, handler })
}

fn handle_not_found(context: &mut Context, msg: Message, _: Vec<String>) -> HandlerFuture {
    let chat_id = msg.get_chat_id();
    let method = SendMessage::new(chat_id, "Unsupported message").reply_to_message_id(msg.id);
    let api = context.get::<Api>();
    HandlerFuture::new(api.execute(method).then(|_| Ok(HandlerResult::Continue)))
}
