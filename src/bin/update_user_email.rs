use kamaji::controller::{
    database::{
        establish_connection,
        user::set_user_email
    },
    io::{
        print_user,
        read_string,
        read_user_id
    },
};

fn main() {
    env_logger::init();
    let id = match read_user_id() {
        Some(id) => id,
        None => return
    };
    let conn = match establish_connection() {
        Some(conn) => conn,
        None => return
    };

    println!("Insert new email: ");
    let email = read_string();

    match set_user_email(&conn, &id, &email) {
        Some(user) => {
            println!("Updated user");
            print_user(&user);
        },
        None => return
    }
}
