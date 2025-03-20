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

use std::{io::{self, Error}, num::ParseIntError};

enum Menu {
    Add,
    View,
}
impl Menu {
    fn match_menu(opt:i32) -> Option<Self>{
        match opt {
            1 => Some(Self::Add),
            2 => Some(Self::View),
            _ => None
        }
    }
}
struct Bill {
    name: String,
    balance: i32
}

impl Bill {
    fn new() -> Result<Self, Error> {
        let name = get_input()?;
        let balance = get_input()?.parse().unwrap();

        Ok(Self { name, balance})
    }


}

fn get_input() -> io::Result<String>{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}

fn match_opt() -> Result<u8, ParseIntError>{
    let opt = get_input().unwrap().parse::<u8>()?;
        Ok(opt) 

}

fn main() {
    let mut bill_list = vec![];
    println!("Welcome what will you like to do?");
    println!("1. Add Bill");
    println!("2. View Bill");
    let opt_chos = match_opt()
    let user = Bill::new();
    match user {
        Ok(bill ) => bill_list.push(bill),
        Err(e) => println!("error: {:?}", e)
    }
}
