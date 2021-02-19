pub mod auth_info;
pub mod user;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use log::error;
use std::env;

pub fn establish_connection() -> Option<PgConnection> {
    dotenv().ok();

    let database_url = match env::var("DATABASE_URL") {
        Ok(db_url) => db_url,
        Err(e) => {
            error!("Could not parse DATABASE_URL: {}", e);
            return None
        }
    };

    match PgConnection::establish(&database_url) {
        Ok(conn) => Some(conn),
        Err(e) => {
            error!("Could not establish connection: {}", e);
            None
        }
    }
}

