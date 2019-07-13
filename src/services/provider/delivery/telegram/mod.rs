use crate::config::Config as AppConfig;
use crate::services::provider::controller::ProviderController;

use carapax::prelude::{
    Api, App, Context, Handler, HandlerFuture, HandlerResult, Message, SendMessage, Method, MessageData,
};
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
    let provider = ProviderTelegram {
        cnfg: cnfg.clone(),
        provider_cnr: provider_cnr.clone(),
    };

    telegram_app.add_handler(provider)
}

impl Handler for ProviderTelegram {
    type Input = Message;
    type Output = HandlerFuture;

    fn handle(&self, ctx: &mut Context, msg: Self::Input) -> Self::Output {
        let api = ctx.get::<Api>().clone();
        HandlerFuture::new(future::lazy(move || {
            match msg.data.clone() {
                MessageData::Text(text) => {
                    let command = text.data.clone();
                    let method = self.handle_text_message(&api, msg.clone(), command);
                    api.execute(method)
                },
                _ => {
                    let method = self.handle_unsupported_message(msg.clone());
                    api.execute(method)
                }
            }.then(|_| Ok(HandlerResult::Continue))
        }))
    }
}

impl ProviderTelegram {
    pub fn handle_text_message(&self, api: &Api, msg: Message, command: String) -> impl Method<Response=Message> {
        match &*command {
            "/hello" => {
                let chat_id = msg.get_chat_id();
                SendMessage::new(chat_id, "Hello there".to_string())
            }
            _ => {
                let chat_id = msg.get_chat_id();
                SendMessage::new(chat_id, command.clone())
            }
        }
    }

    pub fn handle_unsupported_message(&self, msg: Message) -> impl Method<Response=Message> {
        let chat_id = msg.get_chat_id();
        SendMessage::new(chat_id, "Unsupported message".to_string())
    }
}
