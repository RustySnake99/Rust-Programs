use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    let mut array: Vec<i32> = Vec::new();

    print!("Enter the number of elements in the array: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let size: usize = input.trim().parse().expect("Please enter a valid number...");

    for i in 0..size {
        input.clear();
        print!("Enter element {}: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let element: i32 = input.trim().parse().expect("Please enter a valid integer...");
        array.push(element);
    }

    println!("\nYou entered: {:?}", array);
    println!("Sum of elements: {}", array.iter().sum::<i32>());
    println!("Maximum element: {}", array.iter().max().unwrap());
    println!("Minimum element: {}", array.iter().min().unwrap());
    println!("Average of elements: {}", array.iter().sum::<i32>() as f64 / size as f64);
    println!("Reversed array: {:?}", array.iter().rev().collect::<Vec<&i32>>());
    println!("Sorted array: {:?}", {
        let mut sorted_array = array.clone();
        sorted_array.sort();
        sorted_array
    });

    input.clear();
    print!("\nEnter an element to search for: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let search_element: i32 = input.trim().parse().expect("Please enter a valid integer...");

    let found = array.iter().position(|&x| x == search_element);
    match found {
        Some(index) => {println!("Element {} found at index {}", search_element, index);},
        None => {println!("Element {} not found in the array.", search_element);}
    }
}