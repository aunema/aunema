use crate::config::Config as AppConfig;
use crate::helpers::api::Telegram;
use crate::services::provider::controller::ProviderController;

use carapax::prelude::{
    Api, Context, Handler, HandlerFuture, HandlerResult, Message, MessageData, SendMessage,
};
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
    let provider = ProviderTelegram {
        cnfg: cnfg.clone(),
        provider_cnr: provider_cnr.clone(),
    };
    telegram.app = telegram.app.add_handler(provider);
    telegram
}

impl Handler for ProviderTelegram {
    type Input = Message;
    type Output = HandlerFuture;

    fn handle(&self, ctx: &mut Context, msg: Self::Input) -> Self::Output {
        let this = self.clone();
        let api = ctx.get::<Api>().clone();
        HandlerFuture::new(future::lazy(move || {
            let result = match msg.data.clone() {
                MessageData::Text(text) => {
                    let command = text.data.clone();
                    this.handle_commands(&api, msg.clone(), command)
                }
                _ => None,
            };
            match result {
                Some(method) => {
                    future::Either::A(api.execute(method).then(|_| Ok(HandlerResult::Stop)))
                }
                None => future::Either::B(future::ok(HandlerResult::Continue)),
            }
        }))
    }
}

impl ProviderTelegram {
    pub fn handle_commands(
        &self,
        _api: &Api,
        msg: Message,
        command: String,
    ) -> Option<SendMessage> {
        match command.as_ref() {
            "/bounce" => {
                let chat_id = msg.get_chat_id();
                let bounce_result = self.provider_cnr.bounce();
                let result = format!("Success: {}", bounce_result);
                Some(SendMessage::new(chat_id, result).reply_to_message_id(msg.id))
            }
            _ => None,
        }
    }
}
