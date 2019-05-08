use crate::config;

use postgres::{Connection, TlsMode};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

pub fn init_database(cnfg: &config::Config, retry_count: i16) -> Result<Connection, Box<Error>> {
    let conn_string: &str = &cnfg.db_connection;

    let delay = Duration::from_secs(1);
    let mut try_num: i16 = 1;

    loop {
        match Connection::connect(conn_string, TlsMode::None) {
            Ok(conn) => {
                println!("Connected to database on {} try", try_num);
                return Ok(conn);
            }
            Err(err) => {
                println!("Connection failed on {} retry with error: {}", try_num, err);
                try_num += 1;
                if try_num > retry_count {
                    return Err(err.into());
                }
                sleep(delay);
            }
        }
    }
}
