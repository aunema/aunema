use crate::config;
use postgres::{Connection, TlsMode};

pub fn init_database(cnfg: &config::Config) -> Result<(), Box<std::error::Error>> {
    let conn_string: &str = &cnfg.db_connection;
    let conn = Connection::connect(conn_string, TlsMode::None)?;

    conn.execute("INSERT INTO videos (created_at) VALUES (1000), (2500)", &[])?;

    for row in &conn.query("SELECT id, created_at FROM videos", &[])? {
        let id: i32 = row.get(0);
        let created_at: i64 = row.get(1);
        println!("{} {}", id, created_at);
    }
    Ok(())
}
