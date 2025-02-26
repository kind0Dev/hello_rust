/*
Welcome to Rust Bank System!

Enter your name: John Doe
Account created successfully! Your account ID is 1001.

Select an action:
1. Deposit
2. Withdraw
3. Check Balance
4. Exit

Enter choice: 1
Enter deposit amount: 500
Deposit successful! New balance: 500.0

Enter choice: 2
Enter withdrawal amount: 300
Withdrawal successful! New balance: 200.0

Enter choice: 3
Account ID: 1001
Name: John Doe
Balance: $200.0

Enter choice: 4
Goodbye!
 */

use std::io;

struct Account{
    name: String,
    id: i32,
    balance: f64
}
enum Actions {
    Deposit,
    Withdraw,
    CheckBalance,
    Exit
}


fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn get_deposit_amount(input: &String) -> Result<f64, String> {
    input.parse::<f64>().map_err(|_| "invalid deposit amount, not a number!".to_string())
}

fn get_action_number(input: &String) -> Result<Actions, String> {
    let action_number = input.parse::<i32>().map_err(|_| "invalid action selected, not a number!".to_string());
    match action_number {
        Ok(1) => return Ok(Actions::Deposit),
        Ok(2) => return Ok(Actions::Withdraw),
        Ok(3)=> return Ok(Actions::CheckBalance),
        Ok(4) => return Ok(Actions::Exit),
        _ => Err("No Action for that".to_string())
    }
}
fn acct_creation() -> Account{
    println!("Enter you name:");
    let name = get_input();
    let id =  1;
    println!("Welcome {name}, Account created successfully! Your account ID is {:?}.", id);
    return Account{
        name: name,
        id: id,
        balance: 0.0
    };
}

fn user_option(curr_user: &Account){
    let action = get_input();
    match get_action_number(&action){
        Ok(Actions::Deposit) => deposit(&curr_user),
        Ok(Actions::Withdraw) => withdrawal(&curr_user),
        Ok(Actions::CheckBalance) => print_acct_info(&curr_user),
        Ok(Actions::Exit) => exit(),
        Err(e) => println!("there was a problem with you option {:}", e)
    }
}

fn deposit(account: &Account) {
    println!("Enter deposit amount:");
    let input = get_input();
    let amount = get_deposit_amount(&input);
    match amount{
        Ok(num) => increment_balance(&account,  num),
        Err(e) => println!("Deposit unsuccessful! Error: {:?}", e)
    }
}

fn withdrawal(account: &Account) {
    println!("Enter deposit amount:");
    let input = get_input();
    let amount = get_deposit_amount(&input);
    match amount{
        Ok(num) => decrement_balance(&account,  num),
        Err(e) => println!("withdrawal unsuccessful! Error: {:?}", e)
    }
}

fn increment_balance( mut account: &Account,  num: f64){
    let mut clone_acct= account.clone_from(&account);
    // = &mut account.balance;
    
    clone_acct = clone_acct + num;


    println!("Deposit successful! New balance: {:?}", account.balance);
    print_acct_info(&account)
}

fn decrement_balance(mut account: &Account,  num: f64){
    account.balance -= num;
    println!("withdrawal successful! New balance: {:?}", account.balance);
    print_acct_info(&account)
}
fn exit(){
    println!("Goodbye!");
    std::process::exit(0);
}





fn print_acct_info(account: &Account){
    println!("Account Name: {:}", account.name);
    println!("Account id: {:?}", account.id);
    println!("Account Balance: {:?}", account.balance);
}

fn main(){
    println!("Welcome to Rust Bank System!");
    let mut user_accout = acct_creation();
    print_acct_info(&user_accout);
    loop{
    println!("Select an action:
    1. Deposit
    2. Withdraw
    3. Check Balance
    4. Exit");
        user_option(&user_accout);
    }

}