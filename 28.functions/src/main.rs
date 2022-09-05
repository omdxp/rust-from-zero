fn print_value(x: i32) {
    println!("value = {x}");
}

fn increase(x: &mut i32) {
    *x += 1
}

fn product(x: i32, y: i32) -> i32 {
    x * y
}

fn main() {
    print_value(24);

    let mut z = 1;
    increase(&mut z); // pass as mutable reference
    println!("z = {z}"); // the original value is changed

    let a = 3;
    let b = 5;
    let p = product(a, b);
    println!("{a} * {b} = {p}")
}
