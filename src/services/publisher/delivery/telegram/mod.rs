use crate::config::Config as AppConfig;
use crate::helpers::api::Telegram;
use crate::models::{MediaType, SocialNetwork};
use crate::services::publisher::controller::PublisherController;

use carapax::prelude::{
    Api, Context, Handler, HandlerFuture, HandlerResult, Message, MessageData, SendMessage,
};
use futures::{future, Future};
use std::sync::Arc;

#[derive(Clone)]
pub struct PublisherTelegram {
    pub cnfg: Arc<AppConfig>,
    pub publisher_cnr: Arc<PublisherController>,
}

pub fn init(
    cnfg: &Arc<AppConfig>,
    publisher_cnr: &Arc<PublisherController>,
    mut telegram: Telegram,
) -> Telegram {
    let publisher = PublisherTelegram {
        cnfg: cnfg.clone(),
        publisher_cnr: publisher_cnr.clone(),
    };
    telegram.app = telegram.app.add_handler(publisher);
    telegram
}

impl Handler for PublisherTelegram {
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

impl PublisherTelegram {
    pub fn handle_commands(
        &self,
        _api: &Api,
        msg: Message,
        command: String,
    ) -> Option<SendMessage> {
        match command.as_ref() {
            "/publish" => {
                let chat_id = msg.get_chat_id();
                let result = "Failed: I can't";
                Some(SendMessage::new(chat_id, result).reply_to_message_id(msg.id))
            }
            "/setup_channel" => {
                let chat_id = msg.get_chat_id();
                let res = self.publisher_cnr.add_publisher(
                    chat_id,
                    SocialNetwork::Telegram,
                    vec![MediaType::Image],
                    Some("Example capthion".to_string()),
                    vec!["* * * * *".to_string()],
                );
                let result = match res {
                    Ok(_) => "Success: Channel saved",
                    Err(_err) => "Error",
                };
                Some(SendMessage::new(chat_id, result).reply_to_message_id(msg.id))
            }
            _ => None,
        }
    }
}
