use std::io::{self, Write};

fn longest_word<'a>(sentence: &'a str) -> Option<&'a str> {
    sentence.split_whitespace().max_by_key(|w| w.len())
}

fn main() {
    let mut input = String::new();
    print!("Enter the string to process: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    match longest_word(&input) {
        Some(word) => println!("'{}' was the longest word found.", word),
        None => println!("No words found.... Please try again!"),
    }
}