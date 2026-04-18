struct Counter {
    value: i32,
}

impl Counter {
    fn increment(&mut self) {
        self.value += 1;
    }
    fn decrement(&mut self) {
        if self.value == 0 {
            println!("Speed cannot be lower 0");
        } else {
            self.value -= 1;
        }
    }
}

fn main() {
    let mut number: Counter = Counter { value: 256 };
    number.decrement();
    println!("{}", number.value);
    number.increment();
    println!("{}", number.value);
}
