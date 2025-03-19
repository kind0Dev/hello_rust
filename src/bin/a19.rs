// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

struct Stock {
    name:String,
    remaining: i32
}
impl Stock {
    fn new(name:String, remaining:i32) -> Self{
        Self { name, remaining }
    }
}
fn main() {
    let stock1 = Stock::new("Chairs".to_owned(), 5);
    let stock2 = Stock::new("Beds".to_owned(), 3);
    let stock3 = Stock::new("Tables".to_owned(), 2);
    let stock4 = Stock::new("Couches".to_owned(), 0);

    let mut stocks = HashMap::new();
    stocks.insert(1, stock1);
    stocks.insert(2, stock2);
    stocks.insert(3, stock3);
    stocks.insert(4, stock4);

    for (id, stock) in stocks{
        match stock.remaining {
            0 => println!("{:?} stock with id {:?} is out of stock", stock.name, id),
            _ => println!("{:?} stock with id {:?} has {:?} items remain", stock.name, id, stock.remaining)
        }
    }


}
