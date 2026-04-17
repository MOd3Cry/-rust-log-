#[derive(Debug)]
enum OrderStatus {
    Paid { amount: u32 },
    Sent,
    Delivered,
    Disputed(String),
}

impl OrderStatus {
    fn info(&self) {
        match self {
            Self::Paid { amount } => println!("Paid, {amount}"),
            Self::Sent => println!("Sent"),
            Self::Delivered => println!("Delivered"),
            Self::Disputed(reason) => println!("Disputed reason: {reason}"),
        };
    }
}

struct Order {
    customer: String,
    status: OrderStatus,
}

fn main() {
    let status: OrderStatus = OrderStatus::Sent;
    let order: Order = Order {
        customer: String::from("Joe Biden"),
        status,
    };
    let new_status: OrderStatus = OrderStatus::Disputed(String::from("broken"));
    println!("{new_status:?}");
    println!(
        "The Buyer Is {} And Status Now Is {:?}",
        order.customer, order.status
    );
    let paid: OrderStatus = OrderStatus::Paid { amount: 100 };
    println!("{paid:?}");
    paid.info();
    let delivered_result: OrderStatus = OrderStatus::Delivered;
    delivered_result.info();
}
