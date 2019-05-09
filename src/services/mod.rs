pub mod provider;
pub mod publisher;

use crate::config::Config;
use crate::helpers::database;

pub fn init_services(cnfg: &Config) {
    let conn = database::init_database(&cnfg, 5).expect("Failed to init database");

    // conn.execute("INSERT INTO videos (created_at) VALUES (1000), (2500)", &[])
    //     .expect("Failed while insert");
    // for row in &conn
    //     .query("SELECT id, created_at FROM videos", &[])
    //     .expect("Failed while select")
    // {
    //     let id: i32 = row.get(0);
    //     let created_at: i64 = row.get(1);
    //     println!("{} {}", id, created_at);
    // }

    println!("{:?}, {:?}", cnfg, conn);

    let _provider_dlr_http = provider::delivery::http::init(cnfg);
}
