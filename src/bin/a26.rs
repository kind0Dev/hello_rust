// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

use chrono::prelude::*;
fn main() {
let local: DateTime<Local> = Local::now(); 
let utc: DateTime<Utc> = Utc::now();
println!("local: {:?}", local.format("%a %b %e %T %Y").to_string());
println!("utc: {:?}", utc.format("%a %b %e %T %Y").to_string());

}
