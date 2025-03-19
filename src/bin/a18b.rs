// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this


// * Use an enum to represent all types of employees
#[derive(Debug)]
enum Position {
    Maintenance,
    Marketing,
    Managers,
    LineSupervisors,
    KitchenStaff,
    AssemblyTechnicians
}

impl Position {
    fn check_position(&self) -> Result<&Self, String>{
        match self {
            Position::Maintenance => Ok(self),
            Position::Marketing => Ok(self),
            Position::Managers  => Ok(self),
            Position::LineSupervisors  => Ok(self),
            Position::KitchenStaff  => Ok(self),
            Position::AssemblyTechnicians => Ok(self),
            _ => Err("Not a position".to_owned()) 
        }
    }
}
#[derive(Debug)]
enum Status {
    OnContract,
    Terminated
}
impl Status {
    fn check_status(&self) -> Result<&Self, String>{
        match self {
            Status::OnContract => Ok(self),
            Status::Terminated => Err("Access deny not a staff".to_owned()) 
        }
    }
}
// * Use a struct to store the employee type and whether they are
//   still employed
#[derive(Debug)]
struct Employee{
    position: Position,
    status: Status
}

impl Employee {
    fn new(position:Position, status:Status) -> Self{
        Employee { position, status }
    }

    fn check_status(&self) -> Result<(), String>{
        let check_status = self.status.check_status()?;
        Ok(())
    }
    fn check_position(&self) -> Result<(), String>{
        let check_status = self.position.check_position()?;
        match check_status {
            Position::Maintenance => println!("Maintenance crews as access"),
            Position::Marketing => println!("Marketing department employees as access"),
            Position::Managers  => println!("Managers as access"),
            Position::LineSupervisors  => println!("Line supervisors no access"),
            Position::KitchenStaff  => println!("Kitchen staff no access"),
            Position::AssemblyTechnicians => println!("Assembly technicians no access")
        }
        Ok(())
    }

    fn verify_access(self) {
        match self.check_status() {
            Ok(()) => match self.check_position() {
                Ok(()) => (),
                Err(e) => println!("error: {:?}", e)
            }
            Err(e) => println!("error: {:?}", e)
            
        }
    }
}
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

fn main() {
    let employee1 = Employee::new(Position::Maintenance, Status::OnContract);
    let employee2 = Employee::new(Position::Marketing, Status::Terminated);
    let employee3 = Employee::new(Position::Managers, Status::OnContract);
    let employee4 = Employee::new(Position::LineSupervisors, Status::OnContract);
    let employee5 = Employee::new(Position::KitchenStaff, Status::OnContract);
    let employee6 = Employee::new(Position::AssemblyTechnicians, Status::Terminated);
    let employee7 = Employee::new(Position::Maintenance, Status::Terminated);

    let employees = vec![employee1, employee2, employee3, employee4, employee5, employee6, employee7];
    
    for employee in employees{
        employee.verify_access();

    }
}
