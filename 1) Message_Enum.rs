enum Message {
    Text(String),
    Number(i32),
    Quit,
    Coords(i32, i32),
}

fn process_message(msg: Message) {
    match msg {
        Message::Text(s) => {
            println!("{}", s);
        }
        Message::Number(n) => {
            println!("{}", n*2);
        }
        Message::Quit => {
            println!("Quitting...");
        }
        Message::Coords(x, y) => {
            println!("Your X: {}\nYour Y: {}", x, y);
        }
    }
}

fn main() {
    let m1 = Message::Text(String::from("Hello"));
    process_message(m1);

    let m2 = Message::Number(10);
    process_message(m2);

    let m3 = Message::Quit;
    process_message(m3);
    
    let m4 = Message::Coords(35, 60);
    process_message(m4);
}