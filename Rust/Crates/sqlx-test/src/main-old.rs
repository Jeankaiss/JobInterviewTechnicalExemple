use sqlx::{MySql, MySqlPool, mysql::{MySqlPoolOptions, MySqlRow}, query::Query, Row};

#[derive(Debug)]
struct User {
    uuid: String,
    first_name: Option<String>,
    last_name: Option<String>,
    phone: String,
    email: String,
    password: String,
}

async fn init_db() -> MySqlPool {
    let database_url = "mysql://test:test@localhost/test";
	MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("databse connexion refused")
}

fn get_test_parameters() -> Vec<(String, String)> {
    let mut params = Vec::new();

    params.push((
         "first_name".to_string(),
         "Toto".to_string()
    ));
    params.push((
        "last_name".to_string(),
        "Tata".to_string()
    ));
    params
}

async fn complex_test() {
    let pool = init_db().await;

    let params = get_test_parameters();

    let query = String::from(r#"
        SELECT * FROM user WHERE ? = ?, AND ? = ?
    "#);

    let mut state : Query<'_, MySql, _> = sqlx::query(&query);

    for (key, val) in params {
        state = state.bind(key).bind(val);
    }
    let user = state.map(|row: MySqlRow| User{
        uuid: row.get(0),
        first_name: row.get(1),
        last_name: row.get(2),
        phone: row.get(3),
        email: row.get(4),
        password: row.get(5),
    }).fetch_one(&pool)
    .await
    .map_err(|e| {
        println!("There was an error, error = {}", e);
    }).expect("Erreur");

    println!("user found = {:#?}", user);
}

async fn simple_test() {
    let pool = init_db().await;

    let query = String::from("SELECT * FROM user WHERE first_name = ?");

    let user = sqlx::query(&query)
    .bind("Toto".to_string())
    .map(|row: MySqlRow| User{
        uuid: row.get(0),
        first_name: row.get(1),
        last_name: row.get(2),
        phone: row.get(3),
        email: row.get(4),
        password: row.get(5),
    }).fetch_one(&pool)
    .await
    .map_err(|e| {
        println!("There was an error, error = {}", e);
    }).expect("Erreur");

    println!("user found = {:#?}", user);
}

#[tokio::main]
async fn main() {
    simple_test().await;
    // complex_test().await;
}
