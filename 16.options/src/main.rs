fn main() {
    let x = 3.;
    let y = 2.;

    // Option -> Some(v) | None
    let result = if y != 0. { Some(x / y) } else { None };

    match result {
        Some(z) => println!("{} / {} = {}", x, y, z),
        None => println!("cannot divide by zero"),
    }

    if let Some(z) = result {
        println!("result = {z}")
    }

    // while there is assignment (in this case is always true)
    while let Some(z) = result {
        println!("result = {z}")
    }
}
