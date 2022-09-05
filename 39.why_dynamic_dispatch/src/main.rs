struct Circle {
    radius: f64,
}

struct Square {
    side: f64,
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn main() {
    let shapes: [&dyn Shape; 4] = [
        &Circle { radius: 1. },
        &Square { side: 3. },
        &Circle { radius: 2. },
        &Square { side: 4. },
    ];

    for (i, shape) in shapes.iter().enumerate() {
        println!("shape #{} has area {}", i, shape.area())
    }
}

// dynamic dispatch does has its place like the above case even if it costs performance
// there are cases where you don't know types at compile time
