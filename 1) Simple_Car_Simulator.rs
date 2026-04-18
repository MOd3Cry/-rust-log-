struct Car {
    brand: String,
    speed: f64,
}

impl Car {
    fn new(brand: &str, speed: f64) -> Self {
        Self {
            brand: String::from(brand),
            speed,
        }
    }

    fn accelerate(&mut self, amount: f64) {
        self.speed += amount;
    }

    fn brake(&mut self, amount: f64) {
        self.speed -= amount;
    }

    fn compare(&self, car: &Car) {
        if self.speed > car.speed {
            println!("Winner Car: {}", self.brand);
        } else {
            println!("Winner Car: {}", car.brand);
        }
    }
}

fn main() {
    let mut first_car: Car = Car::new("Lamborgini", 256.0);
    let mut second_car: Car = Car::new("Ferrari", 242.0);
    first_car.compare(&second_car);
    first_car.brake(56.0);
    second_car.accelerate(58.0);
    second_car.compare(&first_car);
}
