// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    let add1: i32 = add(1, 2); //3
    let add2: i32 = add(4, 3); //7
    let add3: i32 = add(2, -5); // -3
    display_result(add1);
    display_result(add2);
    display_result(add3);
}
fn display_result(result: i32) {
    println!("{result:?}");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
