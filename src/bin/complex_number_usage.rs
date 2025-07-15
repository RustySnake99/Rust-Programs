use std::io::{self, Write};
use num_complex::Complex;

fn main() {
    let mut input = String::new();
    println!("Input for first complex number 'a':");
    print!("Enter the real part: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let r1: f32 = input.trim().parse().expect("Please enter a valid number.....");
    input.clear();

    print!("Enter the imaginary part: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let i1: f32 = input.trim().parse().expect("Please enter a valid number....");
    input.clear();

    println!("\nInput for second complex number 'b':");
    print!("Enter the real part: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let r2: f32 = input.trim().parse().expect("Please enter a valid number.....");
    input.clear();

    print!("Enter the imaginary part: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let i2: f32 = input.trim().parse().expect("Please enter a valid number....");

    let complex1 = Complex::new(r1, i1);
    let complex2 = Complex::new(r2, i2);

    println!("\n-\tComplex number a: {}", complex1);
    println!("-\tComplex number b: {}", complex2);
    println!("-\tSum of a and b = {}", complex1 + complex2);
    println!("-\tDifference of a and b = {}", complex1 - complex2);
    println!("-\tProduct of a and b = {}", complex1 * complex2);
    println!("-\tQuotient of a and b = {}", complex1 / complex2);
    println!("-\tMagnitude of a = |a| = {}", complex1.norm());
    println!("-\tMagnitude of b = |b| = {}", complex2.norm());
    println!("-\tConjugate of a = {}", complex1.conj());
    println!("-\tConjugate of b = {}", complex2.conj());
}