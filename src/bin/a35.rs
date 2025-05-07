// Topic: Match guards & binding
//
// Summary:
// * A tile-based game requires different logic for different kinds
//   of tiles. Print different messages depending on the kind of
//   tile selected.
//
// Requirements:
// * Bricks:
//   * Colored bricks should print "The brick color is [color]"
//   * Other bricks should print "[Bricktype] brick"
// * Water:
//   * Pressure levels 10 and over should print "High water pressure!"
//   * Pressure levels under 10 should print "Water pressure level: [Pressure]"
// * Grass, Dirt, and Sand should all print "Ground tile"
// * Treasure Chests:
//   * If the treasure is Gold and the amount is at least 100, print "Lots of gold!"
// * Everything else shoud not print any messages
//
// Notes:
// * Use a single match expression utilizing guards to implement the program
// * Run the program and print the messages with at least 4 different tiles

#[derive(Debug, PartialEq)]
enum TreasureItem {
    Gold,
    SuperPower,
}

#[derive(Debug, PartialEq)]
struct TreasureChest {
    content: TreasureItem,
    amount: usize,
}

#[derive(Debug)]
struct Pressure(u16);

#[derive(Debug)]
enum BrickStyle {
    Dungeon,
    Gray,
    Red,
}

#[derive(Debug)]
enum Tile {
    Brick(BrickStyle),
    Dirt,
    Grass,
    Sand,
    Treasure(TreasureChest),
    Water(Pressure),
    Wood,
}

fn main() {
    let tile = Tile::Water(Pressure(11));
    let tilet = Tile::Treasure(TreasureChest { content: TreasureItem::Gold, amount: 100 });
    let tile2 = Tile::Wood;
    let tile3 = Tile::Sand;
    match tile2 {
        // * Bricks:
    //   * Colored bricks should print "The brick color is [color]"
    Tile::Brick(col @ BrickStyle::Gray | col @ BrickStyle::Red) => println!("The brick color is {:?}", col),
    //   * Other bricks should print "[Bricktype] brick"
    Tile::Brick(other) => println!("The brick color is {:?}", other),
    // * Water:
    //   * Pressure levels 10 and over should print "High water pressure!"
    Tile::Water(p) if p.0 >= 10 => println!("High water pressure!"),
    //   * Pressure levels under 10 should print "Water pressure level: [Pressure]"
    Tile::Water(p) if p.0 < 10 => println!("Water pressure level: {}", p.0),
    // * Grass, Dirt, and Sand should all print "Ground tile"
    tile @ Tile::Grass | tile @ Tile::Dirt | tile @ Tile::Sand => println!("Ground tile"),
    // * Treasure Chests:
    //   * If the treasure is Gold and the amount is at least 100, print "Lots of gold!"
    Tile::Treasure(t) if (t.content == TreasureItem::Gold) && (t.amount >= 100) => println!("Lots of gold!"),
    _ => ()
        
    }
// * Everything else shoud not print any messages
}
