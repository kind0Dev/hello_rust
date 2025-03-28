// Topic: Lifetimes & Structures
//
// Requirements:
// * Display just the names and titles of persons from the mock-data.csv file
// * The names & titles must be stored in a struct separately from the mock
//   data for potential later usage
// * None of the mock data may be duplicated in memory
//
// Notes:
// * The mock data has already been loaded with the include_str! macro, so all functionality
//   must be implemented using references/borrows

const MOCK_DATA: &'static str = include_str!("mock-data.csv");

struct Person<'a>{
    title: Vec<&'a str>,
    name: Vec<&'a str>
}

impl <'a> Person<'a> {
    fn new(title:Vec<&'a str>,name:Vec<&'a str>) -> Self{
        Self { title, name }
    }

    fn print(&self){

        let zip = self.name.iter().zip(self.title.iter());
        for (n, t) in zip {
            println!("name: {:?} title: {:?}", n, t);
            println!("")
            
        }

    }
}

fn main() {
    let data: Vec<_> = MOCK_DATA.split('\n').skip(1).collect();
    let name: Vec<_> = data.iter().filter_map(|line|line.split(',').nth(1)).collect();
    let title: Vec<_> = data.iter().filter_map(|line|line.split(',').nth(4)).collect();

    let persons = Person::new(title, name);
    persons.print();
}
