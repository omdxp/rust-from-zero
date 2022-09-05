use std::fmt::Debug;

#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Square {
    side: f64,
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// fn print_info(shape: impl Shape + Debug) {
// fn print_info<T: Shape + Debug>(shape: T) { // this one is better to reuse types for multiple arguments (trait bound syntax)
fn print_info<T>(shape: T)
where
    T: Shape + Debug,
{
    println!("{:?}", shape);
    println!("The area is {}", shape.area())
}

fn main() {
    let c = Circle { radius: 2. };
    print_info(c);
}
