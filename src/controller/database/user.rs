use chrono::{NaiveDate, NaiveTime};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use log::error;

use crate::models::users::{User, NewUser};

pub fn create_user<'a>(
    conn: &PgConnection,
    name: &'a str,
    email: &'a str,
    creation_date: &'a NaiveDate,
    creation_time: &'a NaiveTime
) -> Option<User> {
    use crate::schema::users;

    let new_user = NewUser {
        name: name,
        email: email,
        creation_date: creation_date,
        creation_time: creation_time
    };

    match diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn) {
            Ok(user) => Some(user),
            Err(e) => {
                error!("Could not create new user: {}", e);
                None
            }
        }
}

pub fn del_user_by_id<'a>(
    conn: &PgConnection,
    user_id: &'a i32
) -> usize {
    use crate::schema::users::dsl::*;

    match diesel::delete(users.filter(id.eq(user_id)))
        .execute(conn) {
            Ok(amount) => amount,
            Err(e) => {
                error!("Could not delete user: {}", e);
                0
            }
        }
}

pub fn fetch_user_by_id<'a>(
    conn: &PgConnection,
    user_id: &'a i32
) -> Option<User> {
    use crate::schema::users::dsl::*;

    match users.filter(id.eq(user_id))
        .load::<User>(conn) {
            Ok(mut users_by_id) => {
                if users_by_id.len() == 0 {
                    None
                } else {
                    let user = users_by_id.remove(0);
                    Some(user)
                }
            },
            Err(e) => {
                error!("Could not fetch user: {}", e);
                None
            }
        }
}

pub fn set_user_name<'a>(
    conn: &PgConnection,
    user_id: &'a i32,
    new_name: &'a str
) -> Option<User> {
    use crate::schema::users::dsl::*;

    match diesel::update(users.find(user_id))
        .set(name.eq(new_name))
        .get_result::<User>(conn) {
            Ok(user) => Some(user),
            Err(e) => {
                error!("Could not update user: {}", e);
                None
            }
        }
}

pub fn set_user_email<'a>(
    conn: &PgConnection,
    user_id: &'a i32,
    new_email: &'a str
) -> Option<User> {
    use crate::schema::users::dsl::*;

    match diesel::update(users.find(user_id))
        .set(email.eq(new_email))
        .get_result::<User>(conn) {
            Ok(user) => Some(user),
            Err(e) => {
                error!("Could not update user: {}", e);
                None
            }
        }
}

