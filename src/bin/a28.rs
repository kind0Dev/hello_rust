// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing
#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

struct Shoe(Color);
impl Shoe {
    fn new(color:Color) -> Self {
        Self(color)
    }
}
struct Pants(Color);
impl Pants {
    fn new(color:Color) -> Self {
        Self(color)
    }
}

struct Shirt(Color);
impl Shirt {
    fn new(color:Color) -> Self {
        Self(color)
    }
}

fn print(shrt:Shirt, pant:Pants, shoe:Shoe) {
    println!("the shirt is {:?}", shrt.0);
    println!("the pants is {:?}", pant.0);
    println!("the shoe is {:?}", shoe.0);
}

fn main() {
    let shoe = Shoe::new(Color::Black);
    let pant = Pants::new(Color::Red);
    let shrt = Shirt::new(Color::Custom("justTry".to_owned()));
    print(shrt, pant, shoe);
}
