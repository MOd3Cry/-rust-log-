struct BankAccount {
    username: String,
    balance: f64,
    activity: bool,
    close_account: bool,
}

impl BankAccount {
    fn new(username: &str, balance: f64, activity: bool, close_account: bool) -> Self {
        Self {
            username: String::from(username),
            balance,
            activity,
            close_account,
        }
    }

    fn deposit(&mut self, amount: f64) {
        if self.close_account {
            println!("Account Was Closed");
        } else {
            self.balance += amount;
        }
    }

    fn withdraw(&mut self, amount: f64) {
        if self.close_account {
            println!("Account Was Closed");
        } else {
            if self.balance < amount {
                println!("Not Enough Money\n");
            } else {
                self.balance -= amount;
            }
        }
    }

    fn check_balance(&self) {
        if self.close_account {
            println!("Account Was Closed");
        } else {
            println!("User: {}\nHave: {}", self.username, self.balance);
        }
    }

    fn check_status(&mut self) {
        if self.close_account {
            println!("Account Was Closed");
        } else {
            if self.activity {
                println!("Your Account Enabled\n");
            } else {
                println!("Your Account Was Closed\n");
                self.close_account = true;
                self.balance = 0.0;
            }
        }
    }
}

fn main() {
    let mut first_user: BankAccount = BankAccount::new("Alina", 25895.5, true, false);
    println!(
        "User: {}\nBalance: {}\nActivity: {}\n",
        first_user.username, first_user.balance, first_user.activity
    );
    let mut second_user: BankAccount = BankAccount::new("Masha", 255.5, false, false);
    println!(
        "User: {}\nBalance: {}\nActivity: {}\n",
        second_user.username, second_user.balance, second_user.activity
    );
    second_user.deposit(30895.5);
    println!(
        "User: {}\nBalance: {}\nActivity: {}\n",
        second_user.username, second_user.balance, second_user.activity
    );
    first_user.withdraw(30000.5);
    first_user.withdraw(20000.5);
    println!(
        "User: {}\nBalance: {}\nActivity: {}\n",
        first_user.username, first_user.balance, first_user.activity
    );
    second_user.check_balance();
    println!("\n");
    second_user.check_status();
    second_user.withdraw(0.5);
}
