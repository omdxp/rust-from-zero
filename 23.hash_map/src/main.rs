use std::collections::HashMap;

fn main() {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("a square has {} sides", shapes["square"]);

    for (key, value) in &shapes {
        println!("{}: {}", key, value)
    }

    shapes.insert("square".into(), 5); // changing the value
    println!("{:?}", shapes);

    shapes.entry("circle".into()).or_insert(1); // if we don't have circle entry, insert with value 1
    println!("{:?}", shapes);
    {
        let actual = shapes.entry("circle".into()).or_insert(2);
        *actual = 0;
    }
    println!("{:?}", shapes);
}
