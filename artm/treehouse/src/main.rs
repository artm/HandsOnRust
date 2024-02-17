use std::io::stdin;

fn ask_string() -> String {
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Failed to read line");
    buf.trim().to_lowercase()
}

#[derive(Debug)]
enum GuestAction {
    Greet,
    GreetWithNote { note: String },
    Probation,
    Refuse,
}

#[derive(Debug)]
struct Invitee {
    name: String,
    greeting: String,
    action: GuestAction,
}

impl Invitee {
    fn new(name: &str, greeting: &str, action: GuestAction) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
            action,
        }
    }

    fn greet(&self) {
        match &self.action {
            GuestAction::Greet => println!("{}", self.greeting),
            GuestAction::GreetWithNote { note } => {
                println!("{}", self.greeting);
                println!("{note}");
            }
            GuestAction::Probation => {
                println!("{}", self.greeting);
                println!("{} is on probation", self.name);
            }
            GuestAction::Refuse => println!("Go away, {}. You aren't welcome here.", self.name),
        }
    }
}

fn main() {
    let mut guest_list = vec![
        Invitee::new("artm", "Привет, артм!", GuestAction::Greet),
        Invitee::new(
            "dima",
            "Moi, jonge!",
            GuestAction::GreetWithNote {
                note: String::from("Schoenen uit"),
            },
        ),
        Invitee::new("miekje", "Hallo, Miekje", GuestAction::Greet),
        Invitee::new("babaika", "", GuestAction::Refuse),
    ];
    loop {
        println!("Hello, what's your name?");
        let name = ask_string();
        let guest = guest_list.iter().find(|invitee| invitee.name == name);
        match guest {
            Some(guest) => guest.greet(),
            None => {
                if name.is_empty() {
                    break;
                }
                println!("Hi, {name}! How would you like to be greeted?");
                let greeting = ask_string();
                let friend = Invitee::new(&name, &greeting, GuestAction::Probation);
                guest_list.push(friend);
            }
        }
    }
    println!("Final guest list:");
    println!("{:?}", guest_list);
}
