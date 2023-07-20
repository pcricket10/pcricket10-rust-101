// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info
#[derive(Debug)]
enum Ticket {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64),
}
// struct Ticket {
//     name: String,
//     ticket_type: TicketType,
//     ticket_price: f32,
// }
// impl Ticket {
//     fn print(&self) {
//         println!("{:?}", self.name);
//         println!("{:?}", self.ticket_type);
//         println!("{:?}", self.ticket_price);
//     }
// }

fn main() {
    let tickets: Vec<Ticket> = vec![
        Ticket::Backstage(1000.00, "Joe".to_owned()),
        Ticket::Vip(80.00, "Bob".to_owned()),
        Ticket::Standard(60.00),
    ];
    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, holder) => {
                println!("ticket holder: {}, ticket price: {}", holder, price)
            }
            Ticket::Vip(price, holder) => {
                println!("ticket holder: {}, ticket price: {}", holder, price)
            }
            Ticket::Standard(price) => println!("ticket price: {}", price),
        }
    }
}
