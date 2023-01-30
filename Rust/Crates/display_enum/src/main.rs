use std::fmt;

pub enum Address {
    Delivery,
    Billing,
    AssemblyStation,
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "({}, {})", self.x, self.y)
        match self {
            Address::Delivery => write!(f, "Delivery"),
            Address::Billing => write!(f, "Billing"),
            Address::AssemblyStation => write!(f, "AssemblyStation"),
        }
    }
}
fn main() {
    let address = Address::Delivery;

    println!("address = {}", address);
}
