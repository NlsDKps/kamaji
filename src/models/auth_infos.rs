use chrono::{NaiveDate, NaiveTime};
use crate::schema::auth_infos;

#[derive(Queryable)]
pub struct AuthInfo {
    pub id: i32,
    pub user_id: i32,
    pub password_hash: String,
    pub last_changed_date: NaiveDate,
    pub last_changed_time: NaiveTime

}

#[derive(Insertable)]
#[table_name="auth_infos"]
pub struct NewAuthInfo<'a> {
    pub user_id: &'a i32,
    pub password_hash: &'a str,
    pub last_changed_date: &'a NaiveDate,
    pub last_changed_time: &'a NaiveTime
}

