struct Person<'a> {
    name: &'a str, // name exists as long as the Person exists
}

// implemenation should have the same lifetime as well to use the name
// also it is not obligatory to name the lifetime 'a as above, you can name it whatever
impl<'a> Person<'a> {
    fn talk(&self) {
        println!("Hi, my name is {}.", self.name)
    }
}

fn main() {
    let person = Person { name: "Omar" };
    person.talk()
}
