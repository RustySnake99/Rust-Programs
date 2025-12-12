use std::io::{self, Write};
use rand::Rng;

enum Choice {Rock, Paper, Scissors}

fn main() {
    let mut input = String::new();
    print!("Enter '1' for Rock, '2' for Paper or '3' for Scissors: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let choice: u32 = input.trim().parse().unwrap();

    let player_choice = match choice {
        1 => Choice::Rock,
        2 => Choice::Paper,
        3 => Choice::Scissors,
        _ => {println!("Invalid input...."); return;}
    };
    let computer_choice = match rand::rng().random_range(1..=3) as u32 {
        1 => Choice::Rock,
        2 => Choice::Paper,
        _ => Choice::Scissors,
    };

    match (player_choice, computer_choice) {
        (Choice::Rock, Choice::Scissors) | (Choice::Paper, Choice::Rock) | (Choice::Scissors, Choice::Rock) => println!("You win!"),
        (a, b) if std::mem::discriminant(&a) == std::mem::discriminant(&b) => println!("Its a tie!"),
        _ => println!("Oops.... You lose...."),
    }
}