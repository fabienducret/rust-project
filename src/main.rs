mod entities;
mod hello;

fn main() {
    println!("Welcome on your Library Manager !");

    let name = hello::ask_for_name();
    println!("Hello, {}", name);
}
