use std::io::{self, Write};

fn hex_to_rgb(hex_val: &str) -> Option<(u8, u8, u8)> {
    let hex_val = hex_val.trim().trim_start_matches("#");
    if hex_val.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex_val[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex_val[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex_val[4..6], 16).ok()?;
    Some((r, g, b))
}

fn print_colour_block(red: u8, green: u8, blue: u8) {
    for _ in 0..5 {
        for _ in 0..10 {
            print!("\x1b[48;2;{};{};{}m  \x1b[0m", red, green, blue);
        }
        println!();
    }
}

fn main() {
    print!("Enter the colour to be shown in hex format (Ex: #0e0e0e): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match hex_to_rgb(&input.trim()) {
        Some((r, g, b)) => {
            println!("Preview of Colour {}", input);
            print_colour_block(r, g, b);
        }
        None => {
            eprintln!("Invalid colour entered.... Please check the input and try again....");
        }
    }
}