enum Transaction {
    Deposit(f64),
    Withdraw(f64),
}

struct BankAccount {
    username: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw_money(&mut self, amount: Transaction) {
        if self.balance >= *amount.unpacked() {
            self.balance -= *amount.unpacked();
            println!("User {} have {} money", self.username, self.balance);
        } else {
            println!("Not Enough Money");
        }
    }

    fn deposit_money(&mut self, amount: Transaction) {
        self.balance += *amount.unpacked();
        println!("User {} have {} money", self.username, self.balance);
    }
}

impl Transaction {
    fn unpacked(&self) -> &f64 {
        match self {
            Self::Deposit(x) => x,
            Self::Withdraw(y) => y,
        }
    }
}

fn main() {
    let mut new_user: BankAccount = BankAccount {
        username: String::from("Kirill"),
        balance: 90.0,
    };
    let _statistics: Transaction = Transaction::Deposit(910.0);
    new_user.deposit_money(_statistics);
    let _statistics: Transaction = Transaction::Withdraw(10000.0);
    new_user.withdraw_money(_statistics);
    let _statistics: Transaction = Transaction::Withdraw(1000.0);
    new_user.withdraw_money(_statistics);
}
