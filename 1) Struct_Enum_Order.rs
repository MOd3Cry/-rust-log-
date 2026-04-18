struct Order {
    id: u32,
    status: OrderStatus,
}

enum OrderStatus {
    Delivered,
    Cancelled,
}

impl Order {
    fn update_status(&mut self, updated_status: OrderStatus) {
        self.status = updated_status;
    }
}

impl OrderStatus {
    fn info(&self) {
        match self {
            Self::Delivered => println!("Order Was Delivered Succesfull"),
            Self::Cancelled => println!("Order Was Cancelled"),
        }
    }
}

fn main() {
    let mut first_status: OrderStatus = OrderStatus::Delivered;
    let mut user_order: Order = Order {
        id: 192,
        status: first_status,
    };
    println!("User id: {}", user_order.id);
    user_order.status.info();
    let mut second_status: OrderStatus = OrderStatus::Cancelled;
    user_order.update_status(second_status);
    println!("User id: {}", user_order.id);
    println!("User status:");
    user_order.status.info();
}
