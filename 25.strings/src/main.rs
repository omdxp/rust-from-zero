#![allow(unused_variables)]
#![allow(unused_mut)]

fn main() {
    // &str (stack allocated construct)
    let s = "hello there!"; // default type is &str (a string slice)

    for c in s.chars().rev() {
        println!("{c}")
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("first letter is {}", first_char)
    }

    // String (heap allocated construct)
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1
    }
    println!("{}", letters);

    // &str <> String
    let u: &str = &letters; // deref conversion

    // concatenation
    // String + &str (cannot concatenate String + String)
    // let z = letters + "abc";
    // let z = letters + &letters;

    let mut abc = String::from("hello world");
    let mut abc = "hello world".to_string();
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}", abc.replace("ello", "goodbye"))
}

// strings are encoded with utf-8 so when indexing a string, each byte will take an index
