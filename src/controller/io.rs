use log::error;
use std::{io::stdin, env::args};

use crate::models::users::User;

/// Reads string from command line
pub fn read_string() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input[..(input.len() - 1)].to_string() // Drop the newline character
}

/// Prints user to command line
pub fn print_user<'a>(user: &'a User) {
    println!("-------------------------");
    println!("ID: {}", user.id);
    println!("Name: {}", user.name);
    println!("E-Mail: {}", user.email);
    println!("Created on {} at {}", user.creation_date, user.creation_time);
}

/// Read user id from arguments
pub fn read_user_id() -> Option<i32> {
    match args().nth(1) {
        Some(id) => {
            match id.parse::<i32>() {
                Ok(id) => Some(id),
                Err(e) => {
                    error!("Could not parse argument: {}", e);
                    None
                }
            }
        }
        None => {
            error!("You must at least provide one argument");
            None
        }
    }
}
