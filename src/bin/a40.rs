// Topic: Smart Pointers & RefCell
//
// Summary:
//   A vehicle rental company wants to access the rentals available
//   at storefront locations. Create a program that provides access
//   to storefront rentals from the corporate headquarters.
//
// Requirements:
// * Corporate must be able to access the rentals at a storefront
// * Storefronts must be able to rent out vehicles
// * Rentals have the following attributes:
//   - Type of vehicle
//   - Vehicle Identification Number (VIN)
//   - Vehicle status:
//     * Available, Unavailable, Maintenance, Rented
//
// Notes:
// * Use Rc and RefCell to create shared mutable data structures
// * Create at least two rentals and ensure that Corporate and StoreFront
//   can both access the rental information
// * Test your program by changing the vehicle status from both a storefront
//   and from corporate

use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
enum VehicleStatus{
    Available, 
    Unavailable, 
    Maintenance, 
    Rented
}

#[derive(Debug, Clone)]
struct RentalInfo{
    vehicle_type: String,
    vin_number: u64,
    vehicle_status: VehicleStatus
}

type RentalsInfo = Rc<RefCell<Vec<RentalInfo>>>;

#[derive(Debug)]
struct Corporate(RentalsInfo);


#[derive(Debug)]
struct StoreFront(RentalsInfo);

fn main() {
    let benz = Rc::new(RefCell::new(vec![RentalInfo{
        vehicle_type: "benz".to_string(),
        vin_number: 20,
        vehicle_status: VehicleStatus::Available
    }]));

    let corp = Corporate(Rc::clone(&benz));
    let store = StoreFront(Rc::clone(&benz));

    let new_info = vec![RentalInfo{
        vehicle_type: "camry".to_string(),
        vin_number: 50,
        vehicle_status: VehicleStatus::Maintenance}];

    dbg!(corp.0.borrow());
    dbg!(store.0.borrow());
    store.0.replace(new_info);
    dbg!(store.0.borrow());
    dbg!(corp.0.borrow());


}
