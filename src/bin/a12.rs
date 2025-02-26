// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
struct Box{
    dimensions: i32,
    weight: i32,
    color: Color

}
// * Use an enum for the box color
#[derive(Debug)]
enum Color {
    Blue,
    Green,
    Purple
}
impl Box {
    fn create_green_box() -> Self{
        Box{
            dimensions: 2,
            weight: 34,
            color: Color::Green
        }
    }
    fn create_box(dim: i32, weight: i32, col: Color) -> Self{
        Box{
            dimensions: dim,
            weight: weight,
            color: col
        }
    }

    fn print_info(&self){
        println!("This {:?} {:?} dimentional box has a weight of {:?}kg", self.color, self.dimensions, self.weight)
    }
}
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

fn main() {
    let blue_box = Box{
        dimensions: 3,
        weight: 34,
        color: Color::Blue
    };
    blue_box.print_info();
    let green_box = Box::create_green_box();
    green_box.print_info();
    let purple_box = Box::create_box(3, 44, Color::Purple);
    purple_box.print_info();
}
