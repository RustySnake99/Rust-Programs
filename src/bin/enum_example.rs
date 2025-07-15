enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
}

fn process_message(message: Message) {
    match message {
        Message::Quit => println!("Quit message received."),
        Message::Move {x, y} => println!("Move message received with coordinates: ({}, {})", x, y),
        Message::Write(txt) => println!("Write message received with text: {}", txt),
    }
}

fn main() {
    let message = vec![Message::Quit, Message::Move {x: 25, y: 444}, Message::Write(String::from("Rudraksh"))];
    for i in message {
        process_message(i);
    }
}