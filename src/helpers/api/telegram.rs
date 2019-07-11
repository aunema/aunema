use std::error::Error;
use carapax::prelude::{Api, App, Config};

pub struct Telegram {
    pub api: Api,
    pub app: App,
}

pub fn init_telegram(token: String) -> Result<Telegram, Box<dyn Error>> {
    let config = Config::new(token);
    let api = Api::new(config)?;
    let app = App::new();
    Ok(Telegram {api, app})
}
