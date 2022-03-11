use entities::user::User;
use std::io;

pub fn ask_for_name() -> String {
    println!("Please enter your name :");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let user = User::new(name);

    return user.get_name();
}
