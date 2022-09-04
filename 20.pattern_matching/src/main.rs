#![allow(dead_code)]

enum Color {
    Rgb(u8, u8, u8),
    Cmyk {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    },
}

fn how_many(x: i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        12 => "a dozen",
        _z @ 9..=11 => "lots of", // named range (to use it elsewhere)
        _ if (x % 2 == 0) => "some",
        _ => "a few",
    }
}

fn main() {
    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x))
    }
    let point = (3, 4);

    match point {
        (0, 0) => println!("origin"),
        (0, y) => println!("x axis, y = {y}"),
        (ref x, 0) => println!("y axis, x = {x}"), // passed by reference
        (_, y) => println!("(?, {y})"),
    }

    let c = Color::Cmyk {
        cyan: 2,
        magenta: 14,
        yellow: 14,
        black: 255,
    };
    match c {
        Color::Rgb(r, g, b) => println!("rgb({r}, {g}, {b})"),
        Color::Cmyk { black: 255, .. } => println!("black"), // don't care about other fileds
        Color::Cmyk {
            cyan,
            magenta,
            yellow,
            black,
        } => println!("cmyk({cyan}, {magenta}, {yellow}, {black})"),
    }
}
