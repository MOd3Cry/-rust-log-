use std::io;

const MIN_KEY_LENGTH: usize = 8;

fn validate_data(k_length: String) -> (bool, String) {
    if k_length.len() < MIN_KEY_LENGTH {
        (false, k_length)
    } else {
        (true, k_length)
    }
}

fn main() {
    println!("Create Your Login");
    let mut login_input = String::new();
    io::stdin()
        .read_line(&mut login_input)
        .expect("Reading Error");
    let login = login_input.trim();

    println!("{}", login);
    println!("Create Your Password");
    let mut key_input = String::new();
    io::stdin()
        .read_line(&mut key_input)
        .expect("Error Reading");
    let key_length = key_input.trim();
    let key: u64 = key_input
        .trim()
        .parse()
        .expect("All Symbols Needs To Be Integer");

    let tup = validate_data(key_length.to_string());

    if tup.0 == true {
        println!("Your Login: {}", login);
        println!("Your Key: {}", key);
    } else {
        println!("Your Key So Short.. Min - {}!", MIN_KEY_LENGTH);
    }
}
