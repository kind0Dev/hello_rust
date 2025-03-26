// Topic: Generics & Structures
//
// Requirements:
// * Create a Vehicle structure that is generic over traits Body and Color
// * Create structures for vehicle bodies and vehicle colors and implement the
//   Body and Color traits for these structures
// * Implement a 'new' function for Vehicle that allows it to have any body
//   and any color
// * Create at least two different vehicles in the main function and print their
//   info
//
// Notes:
// * Examples of car bodies can be Truck, Car, Scooter
// * Examples of colors could be red, white, black
// * It is not necessary to have data fields or function implementations
//   for the vehicle bodies/colors

trait Body {
    fn body(&self);
}
trait Color {
    fn color(&self);
}



struct  Vehicle<T,U> 
where T: Body ,
    U: Color,
{
    body: T,
    color: U,
}

impl <T: Body , U: Color> Vehicle<T, U> {
    fn  new(body:T,color:U)->Self
    where T: Body,
    U: Color,
    {
        Vehicle{body:body, color:color}
    }
}

#[derive(Debug)]
struct Truck;
impl Body for Truck {
    fn body(&self) {
        println!("the {:?} as good body", self)
    }

}

#[derive(Debug)]
struct Car;
impl Body for Car {
    fn body(&self) {
        println!("the {:?} as good body", self)
    }
}

#[derive(Debug)]
struct White;
impl Color for White{
    fn color(&self) {
        println!("the color is {:?}", self)
    }
}


fn print_vehcle<T,U>(vehc:Vehicle<T,U>) 
where T: Body, U: Color,
{
    vehc.body.body();
    vehc.color.color();

}

fn main() {
    let body = Car{};
    let color = White;
    let vehicle1 = Vehicle::new(body, color);

    print_vehcle(vehicle1);
    
}
