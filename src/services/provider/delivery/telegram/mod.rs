use crate::config::Config as AppConfig;
use crate::services::provider::controller::ProviderController;

use carapax::prelude::*;
use futures::{
    future::{self, Either},
    Future,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct ProviderTelegram {
    pub cnfg: Arc<AppConfig>,
    pub provider_cnr: Arc<ProviderController>,
}

pub fn init(
    cnfg: &Arc<AppConfig>,
    provider_cnr: &Arc<ProviderController>,
    telegram_app: App,
) -> App {
    let _provider = ProviderTelegram {
        cnfg: cnfg.clone(),
        provider_cnr: provider_cnr.clone(),
    };

    telegram_app.add_handler(FnHandler::from(handle_message))
}

pub fn handle_message(context: &mut Context, message: Message) -> HandlerFuture {
    println!("got a message: {:?}\n", message);
    let api = context.get::<Api>().clone();
    HandlerFuture::new(future::lazy(move || match message.get_text() {
        Some(text) => {
            let chat_id = message.get_chat_id();
            let method = SendMessage::new(chat_id, text.data.clone());
            Either::A(api.execute(method).then(|x| {
                println!("sendMessage result: {:?}\n", x);
                Ok(HandlerResult::Continue)
            }))
        }
        None => Either::B(future::ok(HandlerResult::Continue)),
    }))
}
