#![allow(dead_code)]
#![allow(unused_variables)]

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), // tuple
    Cmyk {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    },
}

fn main() {
    let c = Color::Cmyk {
        cyan: 10,
        magenta: 2,
        yellow: 3,
        black: 255,
    };

    match c {
        // match will let you at compilation time what to cover
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0) => println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
        Color::Cmyk {
            cyan: _,
            magenta: _,
            yellow: _,
            black: 255,
        } => println!("black"),
        Color::Cmyk {
            cyan,
            magenta,
            yellow,
            black,
        } => println!("cmyk({cyan}, {magenta}, {yellow}, {black})"),
    }
}
