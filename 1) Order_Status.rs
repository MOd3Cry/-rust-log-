enum OrderStatus {
    Pending,
    Shipped,
    Delivered,
    Cancelled,
}

impl OrderStatus {
    fn info(&self) {
        match self {
            Self::Pending => println!("Order Was Pending"),
            Self::Shipped => println!("Order Was Shipped"),
            Self::Delivered => println!("Order Was Delivered"),
            Self::Cancelled => println!("Order Was Cancelled"),
        }
    }
}

fn main() {
    let status_pending: OrderStatus = OrderStatus::Pending;
    status_pending.info();
    let status_shipped: OrderStatus = OrderStatus::Shipped;
    status_shipped.info();
    let status_delivered: OrderStatus = OrderStatus::Delivered;
    status_delivered.info();
    let status_cancelled: OrderStatus = OrderStatus::Cancelled;
    status_cancelled.info();
}
