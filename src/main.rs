use belajar_say_hello::{say_hello, say_goodbye, say_hello_to_everyone, say_goodbye_to_everyone};

fn main() {
    let response = say_hello("Eko");
    println!("{}", response);

    let response = say_hello_to_everyone();
    println!("{}", response);

    let response = say_goodbye("Eko");
    println!("{}", response);

    let response = say_goodbye_to_everyone();
    println!("{}", response);
}
