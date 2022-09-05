use std::str;

trait Animal {
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name())
    }
}

struct Human {
    name: &'static str,
}

impl Animal for Human {
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("hello, my name is {}", self.name())
    }
}

struct Cat {
    name: &'static str,
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says meow", self.name())
    }
}

enum Creature {
    Human(Human),
    Cat(Cat),
}

fn main() {
    let mut creatures = Vec::new();
    creatures.push(Creature::Human(Human { name: "John" }));
    creatures.push(Creature::Cat(Cat { name: "Garfield" }));
    for c in creatures {
        match c {
            Creature::Human(h) => h.talk(),
            Creature::Cat(c) => c.talk(),
        }
    }

    println!("{}", "*".repeat(20));

    let mut animals: Vec<Box<dyn Animal>> = Vec::new();
    animals.push(Box::new(Human { name: "John" }));
    animals.push(Box::new(Cat { name: "Garfield" }));
    for a in animals {
        a.talk()
    }

    println!("{}", "*".repeat(20));

    let mut animals: Vec<&dyn Animal> = Vec::new();
    animals.push(&Human { name: "John" });
    animals.push(&Cat { name: "Garfield" });
    for a in animals {
        a.talk()
    }
}
