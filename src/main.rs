mod config;

fn main() -> Result<(), Box<std::error::Error>> {
    let cnfg = config::Config::init()?;
    println!("{:?}", cnfg);
    Ok(())
}
