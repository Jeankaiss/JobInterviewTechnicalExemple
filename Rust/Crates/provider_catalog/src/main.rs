#![allow(non_snake_case)]


use std::time::Instant;

pub mod Constants;
pub mod Transfer;
pub mod Database;
pub mod Extract;
pub mod Schema;

use Constants::{
    UNABLE_TO_DESERIALIZE_CATALOG,
    UNABLE_TO_DESERIALIZE_STOCK,
    UNABLE_TO_DELETE_EASY4D_TIRES,
    UNABLE_TO_ADD_EASY4D_TIRES
};

#[tokio::main]
async fn main() {
    let now = Instant::now();

    dotenv::dotenv().ok();

    let mut stock = Transfer::get_provider_stock();
    let stock = Extract::deserialize_stock(&mut stock).expect(UNABLE_TO_DESERIALIZE_STOCK);

    let mut tires = Transfer::get_provider_catalog();
    let mut tires = Extract::deserialize_tires(&mut tires, stock).expect(UNABLE_TO_DESERIALIZE_CATALOG);

    tires.sort_by_cached_key(|element| String::from(&element.ean));
    tires.dedup_by(|x, y| x.ean.eq(&y.ean));

    Database::delete_easy4d_tires().await.expect(UNABLE_TO_DELETE_EASY4D_TIRES);
    Database::add_easy4d_tires(tires).await.expect(UNABLE_TO_ADD_EASY4D_TIRES);

    println!("Le programme a durée {} seconde(s)", now.elapsed().as_secs());
}
