fn use_slice(slice: &mut [i32]) {
    slice[0] = 123; // change will affect also the data array
}

fn main() {
    let mut data = [1, 2, 3, 4, 5];
    let slice: &[i32] = &data[1..4];
    println!("first = {}, len = {}", slice[0], slice.len());

    use_slice(&mut data);
    println!("{:?}", data)
}

// slices does not have a fixed length
