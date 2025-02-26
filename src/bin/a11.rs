// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
struct Item{
    quantity: i32,
    id: i32
}
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
fn display_quty(item: &Item){
    println!("the quntity is: {:?}", item.quantity)
}
// * Create a function to display the id number, with the struct as a parameter
fn display_id(item: &Item){
    println!("the id is: {:?}", item.id)
}

fn main() {
    let fish = Item{
        quantity: 43,
        id: 2
    };
    display_quty(&fish);
    display_id(&fish);
}
