struct Employee {
    name: String,
    company: String,
    phone: String,
    department: String,
}

fn main() {
    let value = Employee {
        name: String::from("Rudraksh"),
        company: String::from("Google"),
        phone: String::from("+918882134219"),
        department: String::from("Python Developer")
    };

    println!("Employee {} of {} company details:", value.name, value.company);
    println!("\t- Phone Number: {}", value.phone);
    println!("\t- Department: {}", value.department);
}