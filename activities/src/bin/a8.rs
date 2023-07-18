// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum DrinkFlavors {
    Grape,
    Lemon,
    Cherry,
}

struct Drink {
    flavor: DrinkFlavors,
    fluid_ounces: f32,
}
fn main() {
    let drink1 = Drink {
        flavor: DrinkFlavors::Grape,
        fluid_ounces: 12.0,
    };
    let drink2 = Drink {
        flavor: DrinkFlavors::Lemon,
        fluid_ounces: 420.69,
    };
    let drink3 = Drink {
        flavor: DrinkFlavors::Cherry,
        fluid_ounces: 0.1,
    };

    print_drinks(drink1);
    print_drinks(drink2);
    print_drinks(drink3);
}

fn print_drinks(drink: Drink) {
    match drink.flavor {
        DrinkFlavors::Cherry => println!("cherry drink has {} fluid ounces", drink.fluid_ounces),
        DrinkFlavors::Grape => println!("grape drink has {} fluid ounces", drink.fluid_ounces),
        DrinkFlavors::Lemon => println!("lemon drink has {} fluid ounces", drink.fluid_ounces),
    }
}
