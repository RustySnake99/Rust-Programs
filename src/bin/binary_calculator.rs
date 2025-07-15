use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    print!("Enter an expression (e.g., 101 + 110): ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input

    io::stdin().read_line(&mut input).unwrap();
    let tokens: Vec<&str> = input.trim().split(' ').collect();

    if tokens.len() != 3 {
        println!("Please enter a valid expression in the format: <binary1> <operator> <binary2>");
        return;
    }
    let x: f64 = tokens[0].parse().unwrap();
    let operator = tokens[1];
    let y: f64 = tokens[2].parse().unwrap();

    let result = match operator {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => {
            if y == 0.0 {
                println!("Error: Division by zero is not allowed.");
                return;
            }
            x / y
        }
        _ => {
            println!("Unsupported operator: {}", operator);
            return;
        }
    };
    println!("Result: {}", result);
}