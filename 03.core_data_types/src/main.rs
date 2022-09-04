use std::mem;

fn main() {
    // unsigned >= 0 (0..255)
    let a: u8 = 123; // 8 bits
    println!("a = {a}");

    // a = 456; // cannot assign to immutable

    // mut
    let mut b: i8 = 0; // mutable signed 8 bits integer
    println!("b = {}", b);
    b = 42;
    println!("b = {b}");

    let mut c = 123456789; // 32-bit signed integer (infered)
    println!("c = {c}, size = {} bytes", mem::size_of_val(&c));
    c = -1;
    println!("c = {c} after modification");

    // i8 u8 i16 u16 i32 u32 i64 u64

    let z: isize = 123; // isize/usize (size of memory address for the current os)
    let size_of_z = mem::size_of_val(&z);
    println!(
        "z = {z}, takes up {size_of_z} bytes, {}-bit os",
        size_of_z * 8
    );

    // character (unicode)
    let d: char = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    // decimal
    let e = 2.5; // double-precision, 8 bytes or 64  bits, f64
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    // boolean
    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
    let f = 4 > 0; // true
    println!("f = {}, size = {} bytes", f, mem::size_of_val(&f));
}
