// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
fn coord(x:i32, y:i32) -> (i32, i32) {
    (x,y)
}
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn main() {
    let card_coord = (3,6);
    let (_,y) = coord(card_coord.0, card_coord.1);

    if y > 5{
        println!("{:?} is greater than 5", y)
    } else if y < 5 {
        println!("{:?} is less than 5", y)
    } else {
        println!("{:?} is equal to 5", y)
    }
}
