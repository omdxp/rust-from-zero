use std::{sync::Arc, thread};

struct Person {
    name: Arc<String>,
}

impl Person {
    fn new(name: Arc<String>) -> Self {
        Person { name: name }
    }
    fn greet(&self) {
        println!("Hi, my name is {}.", self.name)
    }
}

fn main() {
    let name = Arc::new("John".to_string());
    let person = Person::new(name.clone());

    let t = thread::spawn(move || person.greet());
    println!("name = {}", name);
    t.join().unwrap()
}

// Rc are limited to a single thread
// Arc can be used for different threads (thread safe way)
