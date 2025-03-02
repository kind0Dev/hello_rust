// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

fn main() {
    let numbers = vec![10,20,30,40];
    let mut total: i32 = 0;
    let len = numbers.len();
    for num in numbers{
        total += num;
        if num == 30 {
            println!("thirty")
        }
    }
    println!("a list of {:} numbers are total to {:}", len, total)
}
