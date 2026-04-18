struct User {
    username: String,
    age: u8,
    active: bool,
}

impl User {
    fn new(username: &str, age: u8, active: bool) -> Self {
        Self {
            username: String::from(username),
            age,
            active,
        }
    }

    fn is_adult(&self) -> bool {
        if self.age >= 18 { true } else { false }
    }

    fn deactivate(&mut self) {
        if self.active == true {
            self.active = false;
        }
    }
}

fn main() {
    let mut user1: User = User::new("Vladimir", 24, true);
    println!(
        "The name of the user is {}, his age: {} and he is active? {}",
        user1.username, user1.age, user1.active
    );
    user1.is_adult();
    user1.deactivate();
    println!(
        "The name of the user is {}, his age: {} and he is active? {}",
        user1.username, user1.age, user1.active
    );
}
