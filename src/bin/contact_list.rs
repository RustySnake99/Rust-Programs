use std::io::{self, Write};

struct Contact {
    name: String,
    email: String,
    phone: String,
}

impl Contact {
    fn new(name: String, email: String, phone: String) -> Self {
        Contact { name, email, phone }
    }
    fn display(&self) {
        println!("Name: {}", self.name);
        println!("Email ID: {}", self.email);
        println!("Phone Number: {}", self.phone);
    }
}

fn main() {
    let mut input = String::new();
    let mut contact_list = Vec::<Contact>::new();

    loop {
        input.clear();
        print!("Enter 1 to add a contact\nEnter 2 to view saved contacts\nEnter 3 to exit: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let choice: u32 = input.trim().parse().expect("Please enter a valid choice number...");

        match choice {
            1 => {
                input.clear();
                print!("Enter the name: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                let name = input.trim().to_string();

                input.clear();
                print!("Enter the Email ID: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                let email = input.trim().to_string();

                input.clear();
                print!("Enter the mobile number: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                let phone = input.trim().to_string();

                contact_list.push(Contact::new(name, email, phone));
                println!("New contact added successfully!\n");
            }

            2 => {
                if contact_list.is_empty() {
                    println!("No contacts were found....\n");
                } else {
                    for i in &contact_list {
                        i.display();
                        println!("\n");
                    }
                }
            }

            3 => {
                println!("Thanks for using! Exiting the program now!");
                break;
            }

            _ => {
                println!("Invalid choice! Please enter a valid option.\n");
                continue;
            }
        }
    }
}