use chrono::{NaiveDate, NaiveTime};
use crate::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub creation_date: NaiveDate,
    pub creation_time: NaiveTime
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub creation_date: &'a NaiveDate,
    pub creation_time: &'a NaiveTime
}

