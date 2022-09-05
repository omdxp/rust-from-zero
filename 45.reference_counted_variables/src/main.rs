use std::rc::Rc;

struct Person {
    name: Rc<String>,
}

impl Person {
    fn new(name: Rc<String>) -> Self {
        Person { name: name }
    }
    fn greet(&self) {
        println!("Hi, my name is {}", self.name)
    }
}

fn main() {
    let name = Rc::new("John".to_string());
    println!(
        "name = {}, name has {} strong pointers",
        name,
        Rc::strong_count(&name) // 1
    );
    {
        let person = Person::new(name.clone()); // clone() will increment the reference count
        println!(
            "name = {}, name has {} strong pointers",
            name,
            Rc::strong_count(&name) // 2
        );
        person.greet();
    }
    println!(
        "name = {}, name has {} strong pointers",
        name,
        Rc::strong_count(&name) // 1 (because the person has been cleaned up)
    );
}
