use chrono::{NaiveDate, NaiveTime, Utc};
use kamaji::controller::{
    crypto::hash_password,
    database::establish_connection,
    database::user::create_user,
    database::auth_info::create_auth_info,
    io::{read_string, print_user},
};

/// Calculates today
fn get_date_time() -> (NaiveDate, NaiveTime) {
    let today = Utc::now();
    let date = today.date().naive_utc();
    let time = today.time();
    (date, time)
}

fn main() {
    env_logger::init();
    let connection = match establish_connection() {
        Some(conn) => conn,
        None => return
    };

    let (date, time) = get_date_time();

    println!("Insert user name: ");
    let name = read_string();
    println!("Insert user email: ");
    let email = read_string();
    println!("Insert user password: ");
    let password = read_string();
    let password = hash_password(&password);

    let user = match create_user(&connection, &name, &email, &date, &time) {
        Some(user) => {
            println!("Saved user");
            print_user(&user);
            user
        },
        None => return
    };
    let auth_info = match create_auth_info(&connection, &user.id, &password, &date, &time) {
        Some(auth_info) => auth_info,
        None => return
    };
    println!("Saved authentication infos with password: {}!", auth_info.password_hash);
}
