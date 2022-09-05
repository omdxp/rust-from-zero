#![allow(unused_variables)]

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let b: bool = rng.gen();
}

// crates are packages for rust
// you can add a crate using: cargo add <name>
