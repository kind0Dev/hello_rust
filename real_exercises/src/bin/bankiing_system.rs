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
enum Actions {
    Deposit,
    Withdraw,
    CheckBalance,
    Exit
}
struct Account{
    name: String,
    id: i32,
    balance: f64
}
impl Account {
    fn acct_creation() -> Self{
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

    fn deposit(&mut self) {
        println!("Enter deposit amount:");
        let amount = get_deposit_amount();
        match amount{
            Ok(num) => self.increment_balance(num),
            Err(e) => println!("Deposit unsuccessful! Error: {:?}", e)
        }
        self.user_option();
    }
    
    fn withdrawal(&mut self) {
        println!("Enter withdraw amount:");
        let amount = get_deposit_amount();
        match amount{
            Ok(num) => self.decrement_balance(num),
            Err(e) => println!("withdrawal unsuccessful! Error: {:?}", e)
        }
        self.user_option();
    }

    fn check_balance(&mut self){
        println!("The Account with {:} id has a balance of {:}", self.id, self.balance);
        self.user_option();
    }

    fn print_acct_info(&self){
        println!("Account Name: {:}", self.name);
        println!("Account id: {:?}", self.id);
        println!("Account Balance: {:?}", self.balance);
    }

    fn user_option(&mut self){
        match get_action(){
            Ok(Actions::Deposit) => self.deposit(),
            Ok(Actions::Withdraw) => self.withdrawal(),
            Ok(Actions::CheckBalance) => self.check_balance(),
            Ok(Actions::Exit) => exit(),
            Err(e) => println!("there was a problem with you option {:}", e)
        }
    }

    fn increment_balance( &mut self,  num: f64){
        self.balance += num;
        println!("Deposit successful! New balance: {:?}", self.balance);
        self.print_acct_info()
    }

    fn decrement_balance(&mut self,  num: f64){
        self.balance -= num;
        println!("withdrawal successful! New balance: {:?}", self.balance);
        self.print_acct_info()
    }
}



fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn get_deposit_amount() -> Result<f64, String> {
    let mut input;
    let mut amount;
        loop {
            input = get_input();
            amount = input.parse::<f64>().map_err(|_| "invalid deposit amount, not a number!".to_string());
            
            if amount == Err("invalid deposit amount, not a number!".to_string()){
                println!("Please enter a valid number")
            }else{
                break
            }
        }
        return amount;
}

fn get_action() -> Result<Actions, String> {
    let mut action_number;
    loop {
        println!("Select an action:
            1. Deposit
            2. Withdraw
            3. Check Balance
            4. Exit");
            let action = get_input();
        action_number = action.parse::<i32>().map_err(|_| "invalid action selected, not a number!".to_string());
        
        match action_number {
            Ok(1) => return Ok(Actions::Deposit),
            Ok(2) => return Ok(Actions::Withdraw),
            Ok(3)=> return Ok(Actions::CheckBalance),
            Ok(4) => return Ok(Actions::Exit),
            _ => println!("Please enter a valid action")
        };
        
    }


}


fn exit(){
    println!("Goodbye!");
    std::process::exit(0);
}



fn main(){
    println!("Welcome to Rust Bank System!");
    let mut user_account = Account::acct_creation();
    user_account.print_acct_info();
    user_account.user_option();
    

}