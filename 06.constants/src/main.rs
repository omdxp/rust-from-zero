const ANSWER_OF_LIFE: u8 = 42; // no fixed address

static mut Z: i32 = 123; // mutable variable accessible anywhere (unsafe)

fn main() {
    println!("{}", ANSWER_OF_LIFE); // at compilation the value is replaced (as if you wrote 42)
    unsafe {
        Z = 777;
        println!("{}", Z);
    }
}
