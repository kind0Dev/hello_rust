// Welcome to the Number Guessing Game!
// I'm thinking of a number between 1 and 100.
// Take a guess: 50
// Too high!
// Take a guess: 25
// Too low!
// Take a guess: 37
// You guessed it! Congratulations!

use::std::io;

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn get_number(input: &String) -> Result<i32, String> {
    input.parse::<i32>().map_err(|_| "invalid guess, not a number!".to_string())
}

fn get_valid_number() -> i32 {
    loop {
        println!("Guess a number:");
        let input = get_input();
        if ["exit", "quit"].contains(&input.as_str()){
            println!("Goodbye!");
            std::process::exit(0);
        }
        match get_number(&input) {
            Ok(num) => return num,
            Err(e) => println!("invalid number! Please guess a real number: {}", e)
        }
    }
}

fn get_valid_guess_number() -> i32 {
    loop {
        println!("input a number to guess:");
        let input = get_input();
        if ["exit", "quit"].contains(&input.as_str()){
            println!("Goodbye!");
            std::process::exit(0);
        }
        match get_number(&input) {
            Ok(num) => return num + 7%2,
            Err(e) => println!("invalid number! Please add a gussable number: {}", e)
        }
    }
}

fn rand_guess_generator(guess:i32, user_guess:i32) {
        if user_guess > guess {
            println!("Too High, Try Again");
            try_again(guess);

        }else if user_guess < guess {
            println!("Too Low, Try Again");
            try_again(guess);
        } else{
            println!("You guessed it! {} Congratulations!", guess)
        }
    
}
fn try_again(guess:i32){
    loop {
        let user_guess_new = get_valid_number();
        rand_guess_generator(guess, user_guess_new);
        break;
    }
}

fn continue_guess(){
    loop {
        let number_to_guess_new = get_valid_guess_number();
        println!("I'm thinking of a number between 1 and 100.");
        let user_guess_new = get_valid_number();
        rand_guess_generator(number_to_guess_new, user_guess_new);
        break;
    }
}


fn main() {

    println!("Welcome to the Number Guessing Game!");
    continue_guess();
    loop {
        println!("Do you want to take another guess? (yes/no)");
        let again = get_input();
        if again.to_lowercase() == "yes" {
        continue_guess();
        } else {
            println!("Goodbye!");
            break;
        }
        
    }

}

//todo
// Input Validation: Make sure the player enters a valid number. Handle cases where they enter text or something that's not a number. You might need to look ahead slightly at error handling for this or keep it simple for now and assume valid input.

// Guess Counter: Keep track of the number of guesses the player has made and display it at the end.

// Limited Guesses: Give the player a limited number of guesses (e.g., 7 guesses). If they don't guess correctly within the limit, tell them they lost and reveal the secret number.

// Range Selection: Allow the player to choose the range of numbers (e.g., 1-100, 1-1000).

// Difficulty Levels: Implement difficulty levels that change the range or the number of guesses allowed.