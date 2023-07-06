// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
    let mut i: i32 = 0;
    loop {
        i = i + 1;
        println!(" {i}");
        if i == 4 {
            break;
        }
    }
}
