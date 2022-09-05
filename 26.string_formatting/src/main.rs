fn main() {
    let name = "Omar";
    let greeting = format!("hi, I'm {name}, nice to meet you");
    println!("{greeting}");

    let hello = "hello";
    let rust = "rust";
    let hello_rust = format!("{}, {}!", hello, rust);
    println!("{}", hello_rust);

    let run = "run";
    let forest = "forest";
    let rfr = format!("{0}, {1}, {0}", run, forest);
    println!("{rfr}");

    let info = format!(
        "the name's {last}. {first} {last}.",
        first = "James",
        last = "Bond"
    );
    println!("{}", info);

    let mixed = format!(
        "{1} {} {0} {} {data}",
        "alpha",
        "beta",
        // "gamma", // compilation error: unused argument
        data = "delta",
    );
    println!("{}", mixed)
}

// the format! macro is useful to format strings
// format! returns a String (heap allocated construct)
