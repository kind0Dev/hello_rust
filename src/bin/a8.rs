// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
#[derive(Debug)]
enum Flavor {
    Vanilla,
    Banana,
    Apple
}
// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    name: String,
    flavor: Flavor,
    fluid_ounce: f64

}
// * Use a function to print out the drink flavor and ounces
fn print_drink_info(drink: Drink) -> Flavor {
    println!("{:?} Drink has fluid ounces of {:?} and a flavor of {:?}", drink.name, drink.fluid_ounce, drink.flavor);
    return drink.flavor;
}
// * Use a match expression to print the drink flavor
fn print_flavor(drink_flavor: Flavor){
    println!("with a {:?} flavor", drink_flavor)
}

fn main() {
    let lacasera_flavor = Flavor::Apple;
    let pepsi_flavor = Flavor::Vanilla;
    let nutriyo_flavor = Flavor::Banana;

    let lacasera = Drink{
        name: "Lacasera".to_string(),
        flavor: lacasera_flavor,
        fluid_ounce: 2.1
    };

    let pepsi = Drink{
        name: "Pepsi".to_string(),
        flavor: pepsi_flavor,
        fluid_ounce: 2.2
    };
    let nutriyo = Drink{
        name: "NutriYo".to_string(),
        flavor: nutriyo_flavor,
        fluid_ounce: 2.5
    };

    print_flavor(print_drink_info(lacasera));
    print_drink_info(pepsi);
    print_drink_info(nutriyo);
}
