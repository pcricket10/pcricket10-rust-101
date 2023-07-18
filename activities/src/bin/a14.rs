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
struct Person {
    age: u16,
    name: String,
    favorite_color: String,
}
impl Person {
    fn print(&self) {
        println!("{}", self.name);
        println!("{}", self.favorite_color);
    }
}
fn main() {
    let people = vec![
        Person {
            age: 11,
            name: "Bob".to_string(),
            favorite_color: "blue".to_string(),
        },
        Person {
            age: 4,
            name: "Joe".to_string(),
            favorite_color: "gray".to_string(),
        },
        Person {
            age: 10,
            name: "Billy".to_string(),
            favorite_color: "purple".to_string(),
        },
    ];
    for person in people {
        if person.age <= 10 {
            person.print();
        }
    }
}
