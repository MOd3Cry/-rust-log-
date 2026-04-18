enum Command {
    Start,
    Stop,
    Restart,
}

impl Command {
    fn check(&self) {
        match self {
            Self::Start => println!("System Started"),
            Self::Stop => println!("System Stopped"),
            Self::Restart => println!("System Restarted"),
        }
    }
}

fn main() {
    let new: Command = Command::Start;
    new.check();
    let new: Command = Command::Stop;
    new.check();
    let new: Command = Command::Restart;
    new.check();
}
