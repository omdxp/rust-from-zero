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

// monomorphization (compile time process where print_it will be defined for String and i32 arguments)
fn print_it<T: Printable>(z: T) {
    println!("{}", z.format())
}

fn main() {
    let a = 123;
    let b = "hello".to_string();

    // this is static dispatch
    print_it(a);
    print_it(b)
}
