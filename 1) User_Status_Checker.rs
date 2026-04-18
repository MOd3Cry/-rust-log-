enum UserStatus {
    Active,
    Banned,
    Guest,
}

impl UserStatus {
    fn check_status(&self) {
        match self {
            Self::Active => println!("Full access"),
            Self::Banned => println!("Access denied"),
            Self::Guest => println!("Limited access"),
        }
    }
}

fn main() {
    let first_user: UserStatus = UserStatus::Guest;
    first_user.check_status();

    let second_user: UserStatus = UserStatus::Active;
    second_user.check_status();

    let third_user: UserStatus = UserStatus::Banned;
    third_user.check_status();
}
