use std::io;

enum Converter {
    CtoF(f64),
    FtoC(f64),
}

impl Converter {
    fn convert_method(&self) -> f64 {
        match self {
            Self::CtoF(x) => *x * (9.0 / 5.0) + 32.0,
            Self::FtoC(y) => (*y - 32.0) * (5.0 / 9.0),
        }
    }
}

fn convertion(users_choices: u8, users_temperature: f64) {
    match users_choices {
        1 => {
            println!("{}", Converter::CtoF(users_temperature).convert_method());
        }
        2 => {
            println!("{}", Converter::FtoC(users_temperature).convert_method());
        }
        _ => println!("Pls Type Correct Choice"),
    }
}

fn main() {
    println!("Pls Type Your Temperature Value:");
    let mut user_temperature: String = String::new();
    io::stdin()
        .read_line(&mut user_temperature)
        .expect("Pls Type String");

    let parsed_user_temperature: f64 = user_temperature.trim().parse().expect("Pls Type Number");

    println!("Temperature Converter:\n1) C_TO_F\n2) F_TO_C");
    let mut user_choice: String = String::new();

    io::stdin()
        .read_line(&mut user_choice)
        .expect("Pls Type String");

    let parsed_user_choice: u8 = user_choice.trim().parse().expect("Pls Type Number");

    convertion(parsed_user_choice, parsed_user_temperature);
}
