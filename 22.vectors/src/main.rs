fn main() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);

    a.push(42);

    println!("a = {:?}", a);

    // usize isize (size depends on the system, it could be 32 bits or 64 bits)
    let idx: usize = 0;
    a[idx] = 321;
    println!("a[0] = {}", a[idx]);

    // Option
    match a.get(6) {
        Some(x) => println!("a[6] = {}", x),
        None => println!("error, no such element"), // handle the error
    }

    for x in &a {
        println!("{x}")
    }

    a.push(44);
    println!("{:?}", a);

    if let Some(last_element) = a.pop() {
        println!("{last_element} popped")
    }
    println!("{:?}", a);

    // pop all elements
    while let Some(x) = a.pop() {
        println!("{x} popped")
    }
    println!("{:?}", a);
}

// vectors are dynamic arrays
// indexes are usize
// if index is out of bounds, the main thread will be panicked
