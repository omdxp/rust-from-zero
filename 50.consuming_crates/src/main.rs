#![allow(unused_variables)]

use rand::Rng;

use building_modules_and_crates::greetings::german::{
    goodbye as german_goodbye, hello as german_hello,
};

fn main() {
    let mut rng = rand::thread_rng();
    let b: bool = rng.gen();
    println!(
        "{}\n{}",
        building_modules_and_crates::greetings::english::hello(),
        building_modules_and_crates::greetings::english::goodbye()
    );
    println!("{}\n{}", german_hello(), german_goodbye())
}

// crates are packages for rust
// you can add a crate using: cargo add <name>
