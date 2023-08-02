// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();
    stock.insert("Chairs", 5);
    stock.insert("Beds", 3);
    stock.insert("Tables", 2);
    stock.insert("Couches", 0);
    let mut total = 0;

    for (item, quantity) in stock.iter() {
        match quantity {
            0 => println!("{} are out of stock", item),
            _ => println!("There are {} {} in stock", quantity, item),
        }
        total += quantity;
    }
    println!("There are {} items in stock", total);
}
