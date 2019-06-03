use crate::config::Config;

use sendgrid::errors::SendgridError;
use sendgrid::v3::{Email, Message, Personalization, Sender};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Mailer {
    pub cnfg: Arc<Config>,
    pub sender: Arc<Sender>,
}

pub fn init_mailer(cnfg: &Arc<Config>) -> Arc<Mailer> {
    let csg = cnfg.sendgrid.clone().expect("Sendgrid is not initialized");
    let sender = Sender::new(csg.api_key.clone());
    let client = Mailer {
        cnfg: cnfg.clone(),
        sender: Arc::new(sender),
    };
    Arc::new(client)
}

impl Mailer {
    pub fn send(&self, to: String, template_id: String) -> Result<(), SendgridError> {
        let pln = Personalization::new().add_to(Email::new().set_email(&to));
        let csg = self.cnfg.sendgrid.clone().expect("Sendgrid is not initialized");
        let msg = Message::new()
            .set_from(
                Email::new()
                    .set_email(&csg.sender_email)
                    .set_name(&csg.sender_name),
            )
            .set_template_id(&template_id)
            .add_personalization(pln);

        self.sender.send(&msg)?;
        Ok(())
    }
}
