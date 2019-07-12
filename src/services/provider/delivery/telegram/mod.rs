use crate::config::Config as AppConfig;
use crate::services::provider::controller::ProviderController;

use carapax::prelude::{
    Api, App, Context, Handler, HandlerFuture, HandlerResult, Message, SendMessage,
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
        println!("Got a message");
        let api = ctx.get::<Api>().clone();
        HandlerFuture::new(future::lazy(move || match msg.get_text() {
            Some(text) => {
                // Todo: Handle provider api
                let chat_id = msg.get_chat_id();
                let method = SendMessage::new(chat_id, text.data.clone());
                Either::A(api.execute(method).then(|x| {
                    println!("Send message result: {:?}\n", x);
                    Ok(HandlerResult::Continue)
                }))
            }
            None => Either::B(future::ok(HandlerResult::Continue)),
        }))
    }
}
