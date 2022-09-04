#![allow(dead_code)]

use std::mem;

struct Point {
    x: f64,
    y: f64,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn main() {
    let p1 = origin(); // stack allocated
    let p2 = Box::new(origin()); // heap allocated (take smaller size in memory)

    println!("p1 takes up {} bytes", mem::size_of_val(&p1)); // 16 bytes (x: 8 bytes, y: 8 bytes)
    println!("p2 takes up {} bytes", mem::size_of_val(&p2)); // 8 bytes (Box stores the address only)

    let p3 = *p2; // get the value of the p2 address (Unboxing: relocating from the heap back to the stack)
    println!("{}", p3.x)
}
