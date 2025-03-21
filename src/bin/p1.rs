// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use core::num;
use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct Bill {
    name: String,
    amount: f32,
}

struct Bills {
    bill: HashMap<String, Bill>
}
impl Bills {
    fn new() -> Self {
        Self { bill: HashMap::new() }
    }

    fn add(&mut self, bill:Bill) {
        self.bill.insert(bill.name.clone(), bill);
    }

    fn remove(&mut self, name:String){
        if self.bill.remove(&name).is_some() {
            println!("Bill removed successfully!");
        } else {
            println!("Bill not found.");
        }
    }

    fn edit(&mut self, name:String) {

        if let Some(bill) = self.bill.get_mut(&name) {
            println!("Current amount: ${:.2}", bill.amount);
            println!("Enter new amount or press enter to cancel:");
            let new_amount = match read_input_amount() {
                Some(num) => num,
                None => {
                    println!("Edit canceled.");
                    return;
                }
            };
            
            bill.amount = new_amount;
            println!("Bill updated successfully!");
        } else {
            println!("Bill not found.");
        }
    }

    fn get_all(&self) -> HashMap<String, Bill> {
        if self.bill.is_empty(){
            println!("No bills available.");
        } 
        return self.bill.clone()
        
    
        
    }
}

enum Menu {
    AddBill,
    ViewBill,
    RemoveBill,
    EditBill,
    Exit
}
impl Menu {
    fn new(opt:&str) -> Option<Self> {
        match opt {
            "1" => Some(Menu::AddBill),
            "2" => Some(Menu::ViewBill),
            "3" => Some(Menu::RemoveBill),
            "4" => Some(Menu::EditBill),
            "5" => Some(Menu::Exit),
            _ => None,
        }
    }

}

fn main() {
    let mut bills = Bills::new();

    loop {
        println!("\nInteractive Bill Manager");
        println!("1. Add a bill");
        println!("2. View bills");
        println!("3. Remove a bill");
        println!("4. Edit a bill");
        println!("5. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        //let choice = ;
        let choice_menu = match read_input() {
            Some(str) => Menu::new(&str),
            None => Some(Menu::Exit)
        };
        use crate::Menu::*;
        match choice_menu {
            Some(AddBill) => add_bill(&mut bills),
            Some(ViewBill) => view_bills(&bills),
            Some(RemoveBill) => remove_bill(&mut bills),
            Some(EditBill) => edit_bill(&mut bills),
            Some(Exit) => break,
            _ => println!("Invalid option, try again."),
        }
    }
}

fn add_bill(bills: &mut Bills) {
    println!("Enter bill name:");
    let name = match read_input() {
        Some(str) => str,
        None => return
    };
    println!("Enter amount:");
    let amount: f32 = match read_input_amount() {
        Some(val) => val,
        None => return
    };

    let bill = Bill { name, amount };
    bills.add(bill);
    println!("Bill added successfully!");
}

fn view_bills(bills: &Bills) {
    for bill in bills.get_all().values() {
            println!("{}: ${:.2}", bill.name, bill.amount);
        }
    
}

fn remove_bill(bills: &mut Bills) {
    println!("Enter bill name to remove:");
    view_bills(bills);
    let name = match read_input(){
        Some(name) => name,
        None => return
    };
    bills.remove(name);
}

fn edit_bill(bills: &mut Bills) {
    println!("Enter bill name to edit:");
    view_bills(bills);
    let name = match read_input() {
        Some(name) => name,
        None => return
    };
    
    bills.edit(name);
}

fn read_input() -> Option<String> {
    let mut input = String::new();
    while io::stdin().read_line(&mut input).is_err() {
        println!("please enter you data again");
    }
    let input_trim = input.trim().to_owned();
    if input_trim == "" {
        return None
    } else {
        return Some(input_trim)
    }
}

fn read_input_amount() -> Option<f32> {

    println!("Amount:");
    loop {
        let input = match read_input() {
            Some(input) => input,
            None => return None,
        };
        if &input == "" {
            return None;
        }
        let parsed_input: Result<f32, _> = input.parse();
        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter a number"),
        }
    }
    

}
