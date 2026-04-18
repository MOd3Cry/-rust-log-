enum Transaction {
    Deposit(f64),
    Withdraw(f64),
}

struct BankAccount {
    username: String,
    balance: f64,
}

impl BankAccount {
    fn new (
        username: &str,
        balance: f64,
    ) -> Self {
        Self {
            username: String::from(username),
            balance,
        }
    }

    fn balance_action(&mut self, msg: Transaction) {
        match msg {
            Transaction::Deposit(d) => {
                self.balance += d;
            }
            Transaction::Withdraw(w) => {
                if self.balance >= w {
                    self.balance -= w;
                } else {
                    println!("Not Enough Money");
                }
            }
        }
    }
}

fn main() {
    let mut user_bank: BankAccount = BankAccount::new("Alex", 42.0);
    println!("The Name Of User: {}\nAnd His Balance: {}", user_bank.username, user_bank.balance);
    user_bank.balance_action(Transaction::Deposit(8.0));
    println!("\nThe Name Of User: {}\nAnd His Balance: {}", user_bank.username, user_bank.balance);
    user_bank.balance_action(Transaction::Withdraw(8.0));
    println!("\nThe Name Of User: {}\nAnd His Balance: {}", user_bank.username, user_bank.balance);
}
