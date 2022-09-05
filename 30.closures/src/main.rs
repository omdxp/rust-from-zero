fn say_hello() {
    println!("hello")
}

fn main() {
    let sh = say_hello;
    sh();

    // closure
    let plus_one = |x: i32| -> i32 { x + 1 };
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    // the compiler will know the used types when calling the closure
    let two = 2;
    let plus_two = |x| {
        let mut z = x;
        z += two;
        z
    };
    println!("{} + 2 = {}", 3, plus_two(3));

    // T: by value (make copy of variable)
    // &T: by reference
    // &mut T: by mutable reference (to change the original value)
    let plus_three = |x: &mut i32| *x += 3;
    let mut f = 12;
    plus_three(&mut f);
    println!("f = {}", f)
}
