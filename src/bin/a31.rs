// Topic: Trait Objects
//
// Summary:
//   A contractor wants a program that can sum the cost of materials based
//   on how many square meters are required for a job.
//
// Requirements:
// * Calculate multiple material types with different costs
// * Must be able to process a list of varying materials
// * Material types and cost includes:
//   * Carpet - $10 per square meter
//   * Tile - $15 per square meter
//   * Wood - $20 per square meter
// * Square meters must be taken into account
//
// Notes:
// * Create a trait that can be used to retrieve the cost of a material
// * Create trait objects and store them in a vector for processing
// * Use a function to calculate the total cost
// * Process at least 3 different materials

trait Cost {
    fn amount(&self) -> f64;
}
struct Carpet(f64);
impl Cost for Carpet {
    fn amount(&self) -> f64 {
        self.0 * 10.0
    }
}
struct Tile(f64);
impl Cost for Tile {
    fn amount(&self) -> f64 {
        self.0 * 15.0
    }
}
struct Wood(f64);
impl Cost for Wood {
    fn amount(&self) -> f64 {
        self.0 * 20.0
    }
}

fn cal_total_cost(materials:Vec<Box<dyn Cost>>) -> f64 {
    materials.iter().map(|mat|mat.amount()).sum()
}

fn main() {
    let carpet = Carpet(2.0);
    let wood = Wood(5.0);
    let tile = Tile(1.0);
    let materials: Vec<Box<dyn Cost>> = vec![Box::new(carpet), Box::new(wood), Box::new(tile)];
    let total = cal_total_cost(materials);
    println!("total: {}", total);
}
