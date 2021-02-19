use kamaji::controller::{
    database::{
        establish_connection,
        user::del_user_by_id
    },
    io::read_user_id
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
    let amount = del_user_by_id(&conn, &id);
    println!("Deleted {} users with id {}", amount, id);
}
