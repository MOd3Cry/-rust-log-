struct Car {
    brand: String,
    max_speed: u16,
    max_gas: f64,
    current_gas: f64,
    gas_consumption: f64,
}

impl Car {
    fn new(
        brand: &str,
        max_speed: u16,
        max_gas: f64,
        current_gas: f64,
        gas_consumption: f64,
    ) -> Self {
        Self {
            brand: String::from(brand),
            max_speed,
            max_gas,
            current_gas,
            gas_consumption,
        }
    }

    fn drive(&mut self, distance: f64) {
        let total_gas_consumed: f64 = distance * self.gas_per_km();
        if total_gas_consumed > self.current_gas {
            println!("You Dont Have Enough Gas");
        } else {
            let result_value: f64 = self.current_gas - total_gas_consumed;
            self.current_gas -= total_gas_consumed;
            println!("After Track {}", result_value);
        }
    }

    fn gas_per_km(&self) -> f64 {
        self.gas_consumption / 100.0
    }

    fn fast_check(&self, other_car: &Car) -> bool {
        self.max_speed > other_car.max_speed
    }

    fn refill(&mut self, refuel_request: f64) {
        if self.current_gas + refuel_request <= self.max_gas {
            self.current_gas += refuel_request;
        } else {
            println!("Refuel Request To Much");
        }
    }

    fn destroy(self) {
        println!("Car {} Sent To The Junkyard", self.brand);
    }

    fn race_for_survival(self, car2: Car) -> Car {
        if self.max_speed > car2.max_speed {
            println!("Car {} win this race", self.brand);
            car2.destroy();
            self
        } else {
            println!("Car {} win this race", car2.brand);
            self.destroy();
            car2
        }
    }
}

fn main() {
    let mut my_car: Car = Car {
        brand: String::from("Ferrari"),
        max_speed: 320,
        max_gas: 55.0,
        current_gas: 55.0,
        gas_consumption: 20.0,
    };

    println!(
        "My car {} with max speed {}, max gas {}, current gas {} and gas consumption {}",
        my_car.brand, my_car.max_speed, my_car.max_gas, my_car.current_gas, my_car.gas_consumption
    );

    let distance: f64 = 40.0;
    let refuel_request: f64 = 5.0;

    my_car.drive(distance);

    let mut my_second_car: Car = Car::new("Lamborgini", 350, 60.0, 60.0, 25.0);
    my_second_car.drive(distance);

    let mut my_second_car_upgrade: Car = Car {
        current_gas: 80.0,
        max_gas: 80.0,
        brand: my_second_car.brand.clone(),
        ..my_second_car
    };
    println!(
        "My car {} with max speed {}, max gas {}, current gas {} and gas consumption {}",
        my_second_car_upgrade.brand,
        my_second_car_upgrade.max_speed,
        my_second_car_upgrade.max_gas,
        my_second_car_upgrade.current_gas,
        my_second_car_upgrade.gas_consumption
    );
    println!(
        "My car {} with max speed {}, max gas {}, current gas {} and gas consumption {}",
        my_second_car.brand,
        my_second_car.max_speed,
        my_second_car.max_gas,
        my_second_car.current_gas,
        my_second_car.gas_consumption
    );
    println!("Does Lamborgini Faster Than Ferrari?");
    let result_fast_check: bool = my_second_car_upgrade.fast_check(&my_car);
    println!("The Answer: {}", result_fast_check);
    println!("");
    println!("Gas Refill Check");
    println!(
        "Upgraded Lamborgini First Gas Stat: {}",
        my_second_car_upgrade.current_gas
    );
    my_second_car_upgrade.drive(distance);
    my_second_car_upgrade.refill(refuel_request);
    println!("After Refill: {}", my_second_car_upgrade.current_gas);
    let winner: Car = my_second_car.race_for_survival(my_car);
    println!("The Winner: {}", winner.brand);
}
