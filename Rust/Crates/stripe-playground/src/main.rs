#![allow(non_snake_case)]

pub mod Stripe;

use dotenv::dotenv;

use Stripe::PaymentIntent;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pi = PaymentIntent::new(
        9537,
        String::from("eur"),
        Some(true),
        Some(String::from("pm_qdrgQFGQDFGQ356HF"))
    );
    // println!("DEFINITION DE PI : {:#?}", pi);
    pi.create().await;
}
