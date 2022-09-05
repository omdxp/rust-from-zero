#![allow(dead_code)]
#![allow(unused_variables)]

struct Person {
    name: String,
}

impl Person {
    // fn get_ref_name(&self) -> &String {
    fn get_ref_name<'a>(&'a self) -> &'a String {
        // the compiler does this for you
        &self.name
    }
}

// 'z is a custom lifetime that suggest that the Company and Person should have the same lifetime
struct Company<'z> {
    name: String,
    ceo: &'z Person,
}

fn main() {
    let boss = Person {
        name: String::from("Elon Musk"),
    };
    let tesla = Company {
        name: String::from("Tesla"),
        ceo: &boss,
    };

    let z: &String;
    {
        let p = Person {
            name: "John".to_string(),
        };
        z = p.get_ref_name();
        println!("{}", *z)
    }
}

// &'static str: static here is a lifetime and it means this variable will live as long as the program lives
// lifetimes are heplful when you specify that certain objects should have same liftimes or different lifetimes
