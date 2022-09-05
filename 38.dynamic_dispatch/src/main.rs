trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        format!("i32: {}", *self)
    }
}

impl Printable for String {
    fn format(&self) -> String {
        format!("string: {}", *self)
    }
}

fn print_it(z: &dyn Printable) {
    // it will look on the z type as dynamic, if it's i32 it'll call the format impl for i32, same for String
    println!("{}", z.format()) // it'll happen at runtime so it is more expensive call
}

fn main() {
    let a = 123;
    let b = "hello".to_string();

    // this is dynamic dispatch (all information about a is i32 or b is String is lost)
    print_it(&a); // we only have a trait object
    print_it(&b)
}
