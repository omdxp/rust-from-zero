#![allow(unused_mut)]

fn main() {
    let print_vector = |x: &Vec<i32>| {
        println!("x[0] = {}", x[0]);
    };

    let v = vec![1, 2, 3];
    print_vector(&v); // borrowing v as ref
    println!("v[0] = {}", v[0]); // it is legal to use v after its been borrwed

    let mut a = 40;
    let b = &mut a; // a is been borrowed as mutable ref
    *b *= 2;
    println!("a = {}", a); // we can still use it

    let mut z = vec![3, 2, 1];
    for i in &z {
        println!("i = {}", i);
        // z.push(5); // compilation error: z it is borrowed as immutable ref
    }
}
