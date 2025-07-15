use std::io::{self, Write};
use rand::{rng, Rng};

fn main() {
    let mut input = String::new();

    print!("Enter the lower bound: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let l_bound: u32 = input.trim().parse().expect("Please enter a valid positive integer!");

    input.clear();
    print!("Enter the upper bound: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let u_bound: u32 = input.trim().parse().expect("Please enter a valid positive integer!");

    if l_bound >= u_bound {
        println!("❌ Lower bound must be less than upper bound.");
        return;
    }

    let mut rng = rng();  // ✅ new version of thread_rng()
    let result = rng.random_range(l_bound..=u_bound);  // ✅ replaces gen_range()

    println!("🎲 The generated number is: {}", result);
}
