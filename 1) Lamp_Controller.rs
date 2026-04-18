struct Lamp {
    is_on: bool,
}

impl Lamp {
    fn turn_on(&mut self) {
        if self.is_on == true {
            println! {"It's Already Work"};
        } else {
            self.is_on = true;
        }
    }

    fn turn_off(&mut self) {
        if self.is_on == false {
            println! {"It's Already Don't Work"};
        } else {
            self.is_on = false;
        }
    }

    fn status(&self) {
        println!("{}", self.is_on);
    }
}

fn main() {
    let mut new: Lamp = Lamp { is_on: true };
    new.turn_on();
    new.turn_off();
    new.status();
    new.turn_off();
    new.turn_on();
    new.status();
}
