// Welcome to Rust CLI Calculator!
// Enter first number: 10
// Enter second number: 5
// Enter an operation (+, -, *, /): +
// Result: 10 + 5 = 15

// Do you want to perform another calculation? (yes/no): yes

// Enter first number: 8
// Enter second number: 0
// Enter an operation (+, -, *, /): /
// Error: Cannot divide by zero!
use std::io;

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn get_number(input: &String) -> Result<f64, String> {
    input.parse::<f64>().map_err(|_| "Invalid number!".to_string())
}

fn get_valid_number() -> f64 {
    loop {
        println!("Enter a number:");
        let input = get_input();
        if ["exit", "quit", "e", "E"].contains(&input.as_str()) {
            println!("Goodbye!");
            std::process::exit(0);
        }
        match get_number(&input) {
            Ok(num) => return num,
            Err(_) => println!("Invalid number! Please enter a valid number."),
        }
    }
}

fn get_op() -> Result<char, String> {
    let input = get_input();
    let op = input.chars().next().ok_or("Invalid operator!".to_string())?;
    match op {
        '+' | '-' | '*' | '/' => Ok(op),
        _ => Err("Invalid operator!".to_string()),
    }
}

fn get_valid_op() -> char {
    loop {
        println!("Enter an operator (+, -, *, /):");
        match get_op() {
            Ok(op) => return op,
            Err(_) => println!("Invalid operator! Please enter +, -, *, or /."),
        }
    }
}

fn calculate(a: f64, b: f64, op: char) -> Result<f64, String> {
    match op {
        '+' => Ok(a + b),
        '-' => Ok(a - b),
        '*' => Ok(a * b),
        '/' => {
            if b == 0.0 {
                Err("Cannot divide by zero!".to_string())
            } else {
                Ok(a / b)
            }
        }
        _ => Err("Invalid operator!".to_string()),
    }
}

fn main() {
    loop {
        println!("Welcome to Rust CLI Calculator!");
        
        let a = get_valid_number();
        let op = get_valid_op();
        let b = get_valid_number();

        match calculate(a, b, op) {
            Ok(result) => println!("Result: {} {} {} = {}", a, op, b, result),
            Err(e) => println!("Error: {}", e),
        }

        println!("Do you want to perform another calculation? (yes/no)");
        let again = get_input();
        if again.to_lowercase() != "yes" {
            println!("Goodbye!");
            break;
        }
    }
}

