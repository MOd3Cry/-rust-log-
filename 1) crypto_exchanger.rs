use std::io;

const MIN_KEY_LENGTH: usize = 10;

fn check_access(key: &str) -> bool {
    if key.len() > MIN_KEY_LENGTH {
        true
    } else {
        false
    }
}

fn add_funds(users_balance: &mut f64, users_amount: f64) -> f64 {
    let completed_balance = *users_balance + users_amount;
    completed_balance
}

fn main() {
    println!("TYPE YOUR LOGIN AND KEY DATA");
    println!("Input Your Login:");
    let mut binding_login = String::new();
    io::stdin()
        .read_line(&mut binding_login)
        .expect("Pls Type Correct String");
    let user_login = binding_login.trim();
    println!("Input Your User Account Key:");
    let mut binding_key = String::new();
    io::stdin()
        .read_line(&mut binding_key)
        .expect("Pls Type Correct String");
    let user_key = binding_key.trim();
    let check_value = check_access(user_key);
    if (user_login.len() > 0) && (check_value == true) {
        println!("Your Data Was Accepted");
        println!("Input Your Balance (Float):");
        let mut binding_balance = String::new();
        io::stdin()
            .read_line(&mut binding_balance)
            .expect("Pls Type Correct String");
        let mut user_balance: f64 = binding_balance.trim().parse().expect("Pls Type Integer");
        println!("Input Your Amount For Adding (Float):");
        let mut binding_amount = String::new();
        io::stdin()
            .read_line(&mut binding_amount)
            .expect("Pls Type String Value");
        let user_amount: f64 = binding_amount.trim().parse().expect("Pls Type Integer");
        let new_balance_stat = add_funds(&mut user_balance, user_amount);
        println!("New Balance Stats: {}", new_balance_stat);
    } else {
        println!("Your Data Was Rejected");
    }
}
