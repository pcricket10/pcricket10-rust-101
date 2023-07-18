// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

struct Box {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}
struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}
impl Dimensions {
    fn print(&self) {
        println!("width: {}", self.width);
        println!("height: {}", self.height);
        println!("depth: {}", self.depth);
    }
}
enum Color {
    Red,
    Green,
    Blue,
}
impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("Red"),
            Color::Green => println!("Green"),
            Color::Blue => println!("Blue"),
        }
    }
}

impl Box {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {}", self.weight);
    }
}
fn main() {
    let new_box = Box {
        dimensions: Dimensions {
            width: 1.2,
            height: 3.4,
            depth: 5.6,
        },
        weight: 2.2,
        color: Color::Green,
    };
    new_box.print();
}
