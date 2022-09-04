use std::mem;

fn main() {
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);
    println!("a has {} elements, first is {}", a.len(), a[0]);

    a[0] = 321;
    println!("a[0] = {}", a[0]);

    if a != [1, 2, 3, 4, 5] {
        println!("does not match")
    }

    let b = [1u16; 10]; // array initialized with 10 ones
    for i in 0..b.len() {
        println!("{}", b[i])
    }

    println!("b took up {} bytes", mem::size_of_val(&b));

    let mtx: [[f32; 3]; 2] = [[1., 0., 0.], [0., 2., 0.]];
    println!("{:?}", mtx);

    // print the diagonal
    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{i}][{j}] = {}", mtx[i][j])
            }
        }
    }
}

// an array is a datastructure when you know the size in advance
// you cannot compare arrays with different lengths, you'll get compilation error
// the initalized can be modified to use different size of used memory (ex: 2u8)
