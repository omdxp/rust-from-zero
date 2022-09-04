#![allow(dead_code)]

// 32 bits
union IntOrFloat {
    i: i32,
    f: f32,
}

fn process_value(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => println!("answer of life"),
            IntOrFloat { f } => println!("value = {}", f),
        }
    }
}

fn main() {
    let mut iof = IntOrFloat { i: 123 }; // we took 32 bits in memory and placed 123 integer into it
    iof.i = 234;

    let value = unsafe { iof.i };
    println!("iof.i = {}", value);

    process_value(IntOrFloat { f: 42. })
}

// in unions we or the compiler don't know what value is stored in it, that's why we use unsafe block
