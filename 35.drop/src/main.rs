#![allow(unused_variables)]

struct Creature {
    name: String,
}

impl Creature {
    fn new(name: &str) -> Self {
        println!("{} enters the game", name);
        Creature { name: name.into() }
    }
}

impl Drop for Creature {
    fn drop(&mut self) {
        // when the object is out of scope
        println!("{} is dead", self.name);
        self.name = "".into();
    }
}

fn main() {
    let goblin = Creature::new("Jeff");
    println!("game proceeds");
    // goblin.drop(); // compilation error: explicit use of destructor method is not allowed
    drop(goblin); // force dropping the object, also the drop function move the value so you cannot use it afterwards
                  // goblin.name = "".into(); // compilation error
}

// Drop trait is helpful to implement a destructor like C++
