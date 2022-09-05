#![allow(unused_variables)]

fn main() {
    let v = vec![1, 2, 3]; // vector data is on the heap
                           // let v2 = v; // copying a pointer

    // println!("{:?}", v) // compilation error: use of borrowed value

    let foo = |v: Vec<i32>| ();
    // foo(v); // compilation error: use of borrowed value

    let u = 1;
    let u2 = u;
    println!("u = {}", u); // it happens for primitive since it's on the stack

    let u = Box::new(1); // now we are using the heap instead
    let u2 = u;
    // println!("u = {}", *u) // compilation error: use of borrowed value

    let print_vector = |x: Vec<i32>| -> Vec<i32> {
        println!("{:?}", x);
        x
    };
    let vv = print_vector(v); // we took ownership and return it back
    println!("{}", vv[0])
}

// only one variable owns memory at any given time
// since a vector is a pointer to its data you cannot use it after borrowing it
// whereas for primitives like i32 you can use it
