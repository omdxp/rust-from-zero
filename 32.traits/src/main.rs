trait Animal {
    fn create(name: &'static str) -> Self; // static function (create an instance of Animal) (factory function)
    fn name(&self) -> &'static str; // instance function (takes &self)
    fn talk(&self) {
        // default implementation
        println!("{} cannot talk", self.name())
    }
}

struct Human {
    name: &'static str,
}

impl Animal for Human {
    fn create(name: &'static str) -> Self {
        Human { name: name }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    // the talk method doesn't need to be implemented
    fn talk(&self) {
        println!("{} says hello", self.name)
    }
}

struct Cat {
    name: &'static str,
}

impl Animal for Cat {
    fn create(name: &'static str) -> Self {
        Cat { name: name }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    // the talk method doesn't need to be implemented
    fn talk(&self) {
        println!("{} says meow", self.name)
    }
}

// implement custom trait to Vec
trait Summabe<T> {
    fn sum(&self) -> T;
}

impl Summabe<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut res = 0;
        for x in self {
            res += x;
        }
        res
    }
}

fn main() {
    let h: Human = Animal::create("John"); // So cool!
    h.talk();

    let c = Cat::create("Garfield");
    c.talk();

    let a = vec![1, 2, 3];
    println!("sum = {}", a.sum())
}

// trait is like interface in other programming languages
