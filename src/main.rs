use belajar_say_hello::{say_hello, say_goodbye};

fn main() {
    let response = say_hello("Eko");
    println!("{}", response);

    let response = say_goodbye("Eko");
    println!("{}", response);
}
