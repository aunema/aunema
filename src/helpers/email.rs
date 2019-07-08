use crate::config::Config;

use sendgrid::v3::{Email, Message, Personalization, Sender};
use std::error::Error;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Mailer {
    pub cnfg: Arc<Config>,
    pub sender: Arc<Sender>,
}

pub fn init_mailer(cnfg: &Arc<Config>) -> Result<Arc<Mailer>, Box<dyn Error>> {
    let csg = match cnfg.sendgrid.clone() {
        Some(csg) => csg,
        None => return Err(Box::from("Sendgrid is not initialized")),
    };
    let sender = Sender::new(csg.api_key.clone());
    let client = Mailer {
        cnfg: cnfg.clone(),
        sender: Arc::new(sender),
    };
    Ok(Arc::new(client))
}

impl Mailer {
    pub fn send(&self, to: String, template_id: String) -> Result<(), Box<dyn Error>> {
        let pln = Personalization::new().add_to(Email::new().set_email(&to));
        let csg = match self.cnfg.sendgrid.clone() {
            Some(csg) => csg,
            None => return Err(Box::from("Sendgrid is not initialized")),
        };
        let msg = Message::new()
            .set_from(
                Email::new()
                    .set_email(&csg.sender_email)
                    .set_name(&csg.sender_name),
            )
            .set_template_id(&template_id)
            .add_personalization(pln);

        match self.sender.send(&msg) {
            Ok(_) => (),
            Err(_) => return Err(Box::from("Failed to send mail")),
        };
        Ok(())
    }
}
