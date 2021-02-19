use chrono::{NaiveDate, NaiveTime};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use log::error;

use crate::models::auth_infos::{AuthInfo, NewAuthInfo};

pub fn create_auth_info<'a>(
    conn: &PgConnection,
    user_id: &'a i32,
    password_hash: &'a str,
    last_changed_date: &'a NaiveDate,
    last_changed_time: &'a NaiveTime
) -> Option <AuthInfo> {
    use crate::schema::auth_infos;

    let new_auth_info = NewAuthInfo {
        user_id: user_id,
        password_hash: password_hash,
        last_changed_date: last_changed_date,
        last_changed_time: last_changed_time
    };

    match diesel::insert_into(auth_infos::table)
        .values(&new_auth_info)
        .get_result(conn) {
            Ok(auth_info) => Some(auth_info),
            Err(e) => {
                error!("Could not insert auth infos: {}", e);
                None
            }
        }
}
