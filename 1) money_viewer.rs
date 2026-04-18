enum Money {
    USD(f64),
    EUR(f64),
}

impl Money {
    fn check(&self) {
        match self {
            Self::USD(x) => {
                println! {"Amount: {x} USD"};
            }
            Self::EUR(y) => {
                println! {"Amount: {y} EUR"};
            }
        }
    }
}

fn main() {
    let new: Money = Money::USD(267.5);
    new.check();
    let new: Money = Money::EUR(345.95);
    new.check();
}
