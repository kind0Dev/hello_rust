// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "9" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
    let mut looop = 0;
    loop {
        if looop == 10 {
            break;
        }
        println!("{looop:?}");
        looop+=1;
    }
}
