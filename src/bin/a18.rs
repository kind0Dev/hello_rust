// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase


// Notes:
// * Use a struct to store at least the age of a customer
struct Customer{
    id: i32,
    age: i32
}
impl Customer {
    fn new(id:i32, age:i32) -> Self{
        Customer { id, age }
    }
    
    // * Use a function to determine if a customer can make a restricted purchase
    fn check_restriction(&self) -> Result<String, String> {
        if self.age >= 21 {
            Ok("Okay to purchase restricted item".to_owned())
        }else {
            Err("you are below 18 cant purchase restricted item".to_owned())
        }
    }
    fn print_customer(self) -> Result<(), String>{
        let restriction_msg = self.check_restriction()?;
        println!("user id:{:?} is {:?}", self.id, restriction_msg);
        Ok(())
    }
}
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

fn main() {
    let customer1 = Customer::new(1, 22);
    let rest_res = customer1.print_customer();
    println!("restricted?: {:?}", rest_res)
}
