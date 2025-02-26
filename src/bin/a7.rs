// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print
#[derive(Debug)]
enum Color {
    Black,
    White,
    Red
}

fn print_favorite_color(color: Color) {
    match color {
        Color::Black => println!("the color was {color:?}"),
        Color::White => println!("the color was {color:?}"),
        Color::Red => println!("the color was {color:?}"),
    }
}
fn main() {
    let my_favorite_color = Color::Red;
    print_favorite_color(my_favorite_color);

}
