use colored::*;
use rand::{Rng, rng};
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("{}", "🎮 Welcome to Guess the Number!".bold().cyan());
    let mut input = String::new();
    let mut number_generator = rng();

    print!("Choose your difficulty level (1 = Easy; 2 = Medium; 3 = Difficult): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let d: u32 = input
        .trim()
        .parse()
        .expect("Please enter a valid option......");

    let (max_number, max_attempts) = match d {
        1 => (50, 10),
        2 => (100, 8),
        3 => (200, 7),
        _ => {
            println!("Invalid value.... defaulting to {}", "MEDIUM".yellow());
            (100, 7)
        }
    };
    let secret_number = number_generator.random_range(1..=max_number);
    let mut attempts = 0;

    println!(
        "You would have {} attempts ot guess a random number between 1 and {}",
        max_attempts, max_number
    );
    loop {
        input.clear();
        if attempts > max_attempts {
            println!(
                "Sorry, max attempts exceeded, the actual number was {}....",
                secret_number
            );
            break;
        }
        attempts += 1;
        print!("Enter guess {}: ", attempts);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let guess: u32 = input.trim().parse().unwrap();

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too low!".blue()),
            Ordering::Greater => println!("{}", "Too high!".magenta()),
            Ordering::Equal => {
                println!(
                    "{}",
                    format!(
                        "Congratulations!! You guessed the correct number in {} attempts",
                        attempts
                    )
                );
                break;
            }
        }
    }
}
