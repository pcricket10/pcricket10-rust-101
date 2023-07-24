// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Locker {
    student_name: String,
    locker_number: Option<i32>,
}
fn main() {
    let locker1 = Locker {
        student_name: "Bob".to_owned(),
        locker_number: Some(12345),
    };
    let locker2 = Locker {
        student_name: "Billy".to_owned(),
        locker_number: None,
    };
    println!("Student Name: {}", locker1.student_name);

    match locker1.locker_number {
        Some(locker_number) => println!("locker number: {}", locker_number),
        None => println!("Locker number not specified"),
    }
    println!("Student Name: {}", locker2.student_name);

    match locker2.locker_number {
        Some(locker_number) => println!("locker number: {}", locker_number),
        None => println!("Locker number not specified"),
    }
}
