// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function


//
// Notes:
// * Use a struct for a persons age, name, and favorite color
struct Person{
    age: u8,
    name: String,
    favorite_color: String

}
// * The color and name should be stored as a String
impl Person {
    fn new(age:u8, name:String, favorite_color:String) -> Self{
        Person { age, name, favorite_color }
    }
    fn print_info(self){
        println!("name: {:?}", self.name);
        println!("age: {:?}", self.age);
        println!("fav_color: {:?}", self.favorite_color);
    }
}


fn main() {
    // * Create and store at least 3 people in a vector
    let raj = Person::new(20, "raj".to_owned(), String::from("purple"));
    let small_raj = Person::new(4, "small_raj".to_owned(), "blue".to_owned());
    let medium_raj = Person::new(9, "medium_raj".to_owned(), "green".to_owned());

    let people = vec![raj, small_raj, medium_raj];
// * Iterate through the vector using a for..in loop
    for person in people{
        if person.age <= 10{
            person.print_info();
        }
    }
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function
    // * Print out the name and favorite colors of people aged 10 and under
}
