#![allow(dead_code)]
#![allow(unused_variables)]

fn other() {
    let a = 2; // block scoped (exists only in this function)
}

fn scope_and_shadowing() {
    let a = 123; // existing in inner scope as well (shadowing)
    let a = 7; // can be overrided
    println!("a = {a}");
    {
        let b = 456; // exists only in this scope (between curly braces)
        println!("inside, b = {b}");
        let a = 777;
        println!("inside, a = {a}");
    }
}

fn main() {
    scope_and_shadowing();
}
