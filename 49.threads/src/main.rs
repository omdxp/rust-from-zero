use std::{thread, time};

fn main() {
    let handle = thread::spawn(|| {
        for _ in 1..10 {
            print!("+");
            thread::sleep(time::Duration::from_millis(500))
        }
    });

    for _ in 1..10 {
        print!("-");
        thread::sleep(time::Duration::from_millis(300))
    }

    handle.join().unwrap()
}
