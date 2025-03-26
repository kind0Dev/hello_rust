// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Perimeter {
    fn perimeter(&self) -> f64;
}

struct Square{
    length:f64,
}

impl Perimeter for Square {
    fn perimeter(&self) -> f64 {
        self.length * 4.0
    }
}

struct Triangle{
    a:f64,
    b:f64,
    c:f64,
}

impl Perimeter for Triangle {
    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }
}

fn shape_perimeter(shape: impl Perimeter) -> f64 {
    shape.perimeter()
}

fn main() {
    let squ = Square{length: 20.0};
    let tri = Triangle{a: 2.0, b: 3.0, c: 2.0};
    let squ_per = shape_perimeter(squ);
    let tri_per = shape_perimeter(tri);
    println!("squ peri: {:?}", squ_per);
    println!("tri peri: {:?}", tri_per);
}
