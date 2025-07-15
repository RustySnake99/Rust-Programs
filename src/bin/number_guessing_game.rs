use std::io::{self, Write};
use rand::{rng, Rng};

fn main() {
    let mut rng = rng();
    let n = rng.random_range(1..=100);
    let mut attempts = 0;
    let mut input = String::new();

    println!("A random number is generated which has to be guessed in 10 attempts based on the hints provided!");

    loop {
        input.clear();
        print!("\nEnter guess #{}: ", attempts + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let input :u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid positive integer....");
                attempts -= 1;
                continue;
            }
        };
        attempts += 1;

        if input < n {
            println!("GUESS IS TOO LOW!");
        } else if input > n {
            println!("GUESS IS TOO HIGH!");
        } else {
            println!("Congratulations! You guessed the correct number {} in {} attempts!!", input, attempts);
            break;
        }
    }
}