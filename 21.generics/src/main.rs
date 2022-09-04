#![allow(dead_code)]
#![allow(unused_variables)]

struct Point<T, V> {
    x: T,
    y: V,
}

struct Line<T, V> {
    start: Point<T, V>,
    end: Point<T, V>,
}

fn main() {
    let a: Point<u16, u8> = Point { x: 2, y: 1 };
    let b = Point { x: 1.2, y: 3 };

    let line = Line {
        start: a,
        end: Point { x: 0, y: 1 },
    };

    // turbo fish syntax
    let p: Point<i32, f64> = Point { x: 2, y: 2.1 };
}
