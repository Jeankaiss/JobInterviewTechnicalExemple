use sqlx::{MySql, MySqlPool, mysql::{MySqlPoolOptions, MySqlRow}, query::Query, Row};

async fn init_db() -> MySqlPool {
    let database_url = "mysql://test:test@localhost/test";
	MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("databse connexion refused")
}



#[tokio::main]
async fn main() {
    let updatable = vec!["first_name", "last_name", ""];

    println!("res = {}", updatable.join(" = ?, "));
}
