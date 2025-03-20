// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

#[derive(PartialEq,Debug)]
enum State {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate
}
impl State {
    fn verify_input(self){
        match self {
            Self::Off => println!("Powering off"),
            Self::Sleep => println!("Sleeping"),
            Self::Reboot => println!("Rebooting"),
            Self::Shutdown => println!("Shutting down"),
            Self::Hibernate => println!("Hibernating"),
        }
    }

    fn new(input:&str) -> Result<Self, String>{
        match input {
            "OFF" => Ok(Self::Off),
            "SLEEP" => Ok(Self::Sleep),
            "REBOOT" => Ok(Self::Reboot),
            "SHUTDOWN" => Ok(Self::Shutdown),
            "HIBERNATE" => Ok(Self::Hibernate),
            _ => Err("kindly try again not an input".to_owned())
        }

    }
}
fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("no input given");
    input.trim().to_string().to_uppercase()

}


fn main() {
    println!("Welcome kindly choose an action");
    println!("Off");
    println!("Sleep");
    println!("Reboot");
    println!("Shutdown");
    println!("Hibernate");
    loop {
        let user_input = get_input();
        let check_input = State::new(&user_input);
        // match check_input {
        //     Ok(s) => s.verify_input(),
        //     Err(e) => println!("error: {:?}", e)
        // }
        if check_input == Err("kindly try again not an input".to_owned()) {
            println!("error: {:?}", check_input);
            continue;
        }else {
            check_input.unwrap().verify_input();
            break;
        }
    }
}
