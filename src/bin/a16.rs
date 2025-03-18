// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

use core::num;

// * Use a struct containing the student's name and locker assignment
struct Student{
    name: String,
    locker: Option<i32>
}

// * The locker assignment should use an Option<i32>

fn main() {
    let raj = Student{ name: "raj".to_owned(), locker: Some(43)};
    let raj_medium = Student{ name: "raj_medium".to_owned(), locker: Some(5)};
    let raj_small = Student{ name: "raj_small".to_owned(), locker: None};
    let students = vec![raj, raj_medium, raj_small];
    for student in students{
        match student.locker {
            Some(num) => println!("student {:?} has a locker with assignment number {:?}", student.name, num),
            None => println!("student {:?} has no locker assignment", student.name)
        }
    }

}
