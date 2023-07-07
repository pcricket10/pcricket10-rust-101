// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}
fn main() {
    print_color_name(Color::Yellow);
}
fn print_color_name(color: Color) {
    match color {
        Color::Red => println!("you have chosen red"),
        Color::Blue => println!("you have chosen blue"),
        Color::Green => println!("you have chosen green"),
        Color::Yellow => println!("you have chosen yellow"),
    }
}
