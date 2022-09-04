fn main() {
    let mut x = 1;

    while x < 100 {
        x *= 2;
        if x == 64 {
            continue;
        }
        println!("x = {x}");
    }

    // while true
    let mut y = 1;
    loop {
        y *= 2;
        println!("y = {y}");
        if y == 1 << 10 {
            // 2^10
            break;
        }
    }
}
