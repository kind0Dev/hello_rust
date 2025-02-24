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
    return input.trim().to_string();
}
fn get_number(input: &String) -> Result<f64, String> {
    //let mut input = String::new();
    //io::stdin().read_line(&mut input).expect("Failed to read input");
    match input.parse::<f64>(){
        Ok(num) => Ok(num),
        _ => Err("Invalid number!".to_string()),
    }
    
    //map_err(|_| "Invalid number!".to_string())
}
fn get_op() -> Result<char, String> {
    let mut op = String::new();
    io::stdin().read_line(&mut op).expect("Failed to read input");
    let op = op.trim().chars().next().unwrap_or(' ');
    match op {
        '+' => Ok(op),
        '-' => Ok(op),
        '*' => Ok(op),
        '/' => Ok(op),
        _ => Err("Invalid operator!".to_string()),
    }
}

fn calculate(a: f64, b: f64, op: char) -> Result<f64, String> {
    match op {
        '+' => Ok(a + b),
        '-' => Ok(a - b),
        '*' => Ok(a * b),
        '/' => if b == 0.0 { Err("Cannot divide by zero!".to_string()) } else { Ok(a / b) },
        _ => Err("Invalid operator!".to_string()),
    }
}

fn main() {

loop {
    println!("Welcome to Rust CLI Calculator:");
    println!("please enter a number to begin:");
    let input = get_input();
    if input == "exit" || input == "quit" || input == "e" || input == "E" {
        println!("goodbye");
        break;
    }
    let mut a;
    a = get_number(&input);
    while a == Err("Invalid number!".to_string()) {
        println!("Invalid Number please insert a real number");
        a = get_number(&input);
    }
    
    println!("Enter operator (+, -, *, /):");
    let mut operation;
    operation = get_op();
    while operation == Err("Invalid operator!".to_string()) {
        println!("Invalid math operator please insert a real number");
        operation = get_op();
    }


    println!("Enter second number:");
    let input2 = get_input();
    if input2 == "exit" || input2 == "quit" || input2 == "e" || input2 == "E" {
        println!("goodbye");
        break;
    }
    let mut b;
    b = get_number(&input2);
    while b == Err("Invalid number!".to_string()) {
        println!("Invalid Number please insert a real number");
        b = get_number(&input2);
    }
    let x = a.expect("invalid number");
    let y = b.expect("invalid number");
    let op = operation.expect("invalid operation");
    match calculate(x, y, op) {
        Ok(result) => println!("Result: {x} {op} {y} = {result}"),
        Err(e) => println!("Error: {}", e),
    }
}
}
