#![allow(unused_variables)]

fn main() {
    // arithmetic
    let mut a = 2 + 3 * 4; // they all follow precedence
    println!("{a}");

    a += 1; // rust has no prefix or postfix operators like -- or ++
    println!("{a}");

    println!("remainder of {} / {} = {}", a, 3, a % 3);

    let a_cuped = i32::pow(a, 3);
    println!("{a} cuped is {a_cuped}");

    let b = 2.5;
    let b_cuped = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{b} cuped = {b_cuped}, {b}^pi = {b_to_pi}");

    // bitwise (only available to integers)
    let c = 1 | 2; // | OR & AND ^XOR ! NOR
                   // 01 | 10 = 11 == 3_10
    println!("1 | 2 = {c}");
    let two_to_10 = 1 << 10;
    println!("2^10 = {two_to_10}");

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0; // true
                                                // > <= >= ==
    let x = 5;
    let x_is_5 = x == 5; // true
}
