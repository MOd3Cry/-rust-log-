enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Day {
    fn weekend_checker(&self) {
        match self {
            Self::Monday => println!("Work Day"),
            Self::Tuesday => println!("Work Day"),
            Self::Wednesday => println!("Work Day"),
            Self::Thursday => println!("Work Day"),
            Self::Friday => println!("Work Day"),
            Self::Saturday => println!("Weekend"),
            Self::Sunday => println!("Weekend"),
        }
    }
}

fn main() {
    let new_day: Day = Day::Monday;
    new_day.weekend_checker();
    let new_day: Day = Day::Tuesday;
    new_day.weekend_checker();
    let new_day: Day = Day::Wednesday;
    new_day.weekend_checker();
    let new_day: Day = Day::Thursday;
    new_day.weekend_checker();
    let new_day: Day = Day::Friday;
    new_day.weekend_checker();
    let new_day: Day = Day::Saturday;
    new_day.weekend_checker();
    let new_day: Day = Day::Sunday;
    new_day.weekend_checker();
}
