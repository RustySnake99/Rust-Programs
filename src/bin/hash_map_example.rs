use std::collections::HashMap;

fn main() {
    let mut capital_cities = HashMap::new();

    capital_cities.insert("USA", "Washington DC");
    capital_cities.insert("India", "New Delhi");
    capital_cities.insert("Norway", "Oslo");

    println!("The Hash Map created is:\n{:?}\n", capital_cities);

    //looping through a Hash-Map:
    for (i, j) in capital_cities {
        println!("The capital of {} is {}",i , j);
    }
}