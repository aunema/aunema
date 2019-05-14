use crate::config::Config;

#[derive(Clone, Debug)]
pub struct ProviderCron<'a> {
    pub cnfg: &'a Config,
}

pub fn init(cnfg: &Config) {
    let _provider = ProviderCron { cnfg };
}
