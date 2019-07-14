use crate::config::Config as AppConfig;
use crate::helpers::api::Telegram;
use crate::services::provider::controller::ProviderController;

use carapax::prelude::{Api, Context, HandlerFuture, HandlerResult, Message, SendMessage};
use futures::{future, Future};
use std::sync::Arc;

#[derive(Clone)]
pub struct ProviderTelegram {
    pub cnfg: Arc<AppConfig>,
    pub provider_cnr: Arc<ProviderController>,
}

pub fn init(
    cnfg: &Arc<AppConfig>,
    provider_cnr: &Arc<ProviderController>,
    mut telegram: Telegram,
) -> Telegram {
    // Todo: Add provider to context
    let _provider = ProviderTelegram {
        cnfg: cnfg.clone(),
        provider_cnr: provider_cnr.clone(),
    };
    telegram.handler = telegram.handler.add_handler("/hello", handle_hello);
    telegram
}

fn handle_hello(ctx: &mut Context, msg: Message, _: Vec<String>) -> HandlerFuture {
    // Todo: Execute self.prvider_cnr here
    let api = ctx.get::<Api>().clone();
    HandlerFuture::new(future::lazy(move || {
        let chat_id = msg.get_chat_id();
        let method = SendMessage::new(chat_id, "Hello there").reply_to_message_id(msg.id);
        api.execute(method).then(|_| Ok(HandlerResult::Continue))
    }))
}
