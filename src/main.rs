#[macro_use]
extern crate json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate postgres;
#[macro_use]
extern crate serde_repr;
#[macro_use]
extern crate enum_primitive_derive;
#[macro_use]
extern crate serde_enum_derive;

mod config;
mod helpers;
mod models;
mod services;

fn main() {
    let cnfg = config::Config::init().expect("Failed to init config");
    services::init_services(cnfg.clone());
}
