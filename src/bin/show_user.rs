use kamaji::controller::{
    database::{
        establish_connection,
        user::fetch_user_by_id
    },
    io::{print_user, read_user_id}
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

    match fetch_user_by_id(&conn, &id) {
        Some(user) => {
            println!("Fetched user");
            print_user(&user);
        },
        None => return
    }
}
