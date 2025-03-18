// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

// * Use an enum for the tickets with data associated with each variant
enum Tickets {
    Backstage(i32),
    Vip(i32),
    Standard(i32)
}
impl Tickets {
    fn print_tick(self){
        match self{
            Self::Backstage(amount) => println!("Is a Backstage Ticket and priced at {:?} naira", amount),
            Self::Vip(amount) => println!("Is a VIP Ticket and priced at {:?} naira", amount),
            Self::Standard(amount) => println!("Is a Standard Ticket and priced at {:?} naira", amount)
        }
    }
}
struct Visitor {
    name: String,
    ticket: Tickets
}

impl Visitor {
    fn new(name: String, ticket: Tickets) -> Self {
        Visitor { name, ticket }
    }

    fn print_info(self){
        println!("Ticket owned by {:?}", self.name);
        self.ticket.print_tick()
    }
}



fn main() {
    // * Create one of each ticket and place into a vector
    let vip_visi = Visitor::new("Raj".to_owned(), Tickets::Vip(100));
    let stand_visi = Visitor::new("Raj_medium".to_owned(), Tickets::Standard(50));
    let back_visi = Visitor::new("Raj_small".to_owned(), Tickets::Backstage(20));
    let vistors = vec![vip_visi, stand_visi, back_visi];
// * Use a match expression while iterating the vector to print the ticket info
    for vistor in vistors{
        vistor.print_info();
    }
}
