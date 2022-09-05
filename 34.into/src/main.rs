#![allow(dead_code)]
#![allow(unused_variables)]

struct Person {
    name: String,
}

impl Person {
    // fn new(name: &str) -> Self {
    //     Person {
    //         name: name.to_string(),
    //     }
    // }

    // fn new<S: Into<String>>(name: S) -> Self {
    fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Person { name: name.into() }
    }
}

fn main() {
    let john = Person::new("John");
    let name = "Jane".to_string();
    let jane = Person::new(&name); // name.as_ref()
    let does = Person::new("Doe");
}

// Into helps to define types that are convertable to certain type T
// into() converts to that type
