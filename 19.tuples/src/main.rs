#![allow(unused_variables)]

fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}

fn main() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    // destructuring
    let (sum, product) = sp;
    println!("sum = {}, product = {}", sum, product);

    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2); // tuple of tuples
    println!("{:?}", combined);
    println!("last element = {}", combined.1 .1);

    let ((sum1, prod1), (sum2, prod2)) = combined;
    let foo = (true, 42., -1);
    println!("{:?}", foo);

    let answer = (42,); // tuple of a single element (notice the comma)
    println!("{:?}", answer)
}

// with tuples you can return mismatch types unlike arrays
