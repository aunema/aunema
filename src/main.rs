mod config;
mod services;

use services::init_services;

fn main() {
    let cnfg = config::Config::init().expect("Failed to init config");
    init_services(&cnfg);
}
