use std::io::stdin;

use rand::Rng;

fn main() {
    let number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Enter your guess: ");
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>(); // '\n'
                match parsed {
                    Ok(guess) => {
                        if guess < 1 || guess > 100 {
                            println!("Your guess is out of range");
                        } else if guess < number {
                            println!("Your guess is too low");
                        } else if guess > number {
                            println!("Your guess is too high");
                        } else {
                            println!("Correct!!!");
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Could not parse your input. {}. Try again!", e)
                    }
                }
            }
            Err(_) => continue,
        }
    }
}
