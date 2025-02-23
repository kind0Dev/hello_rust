// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
fn add(a:i128, b:i128) -> i128{
    a + b
}
// * Use a function to display the result
fn display_add_result(a:i128){
    print!("{a:?}");
}
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    let x: i128 = add(1,1);
    display_add_result(x);
}
