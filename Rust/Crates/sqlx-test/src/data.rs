use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use sqlx::FromRow;

pub async fn init_db() -> MySqlPool {
    let database_url = "mysql://test:test@localhost/test";
	MySqlPoolOptions::new()
        .max_connections(100)
        .connect(&database_url)
        .await
        .expect("databse connexion refused")
}

pub enum TestUser {
    One,
    Two,
    Three,
    Four,
}

pub fn get_user(user: TestUser) -> User {
    match user {
        TestUser::One => {
            User {
                uuid: "001c5616-3029-4321-834b-0741871cbc6c".to_string(),
                first_name: Some("Toto".to_string()),
                last_name: Some("Tata".to_string()),
                phone: "0644958745".to_string(),
                email: "test@test.fr".to_string(),
                password: "test".to_string(),
            }
        }
        TestUser::Two => {
            User {
                uuid: "fc1a93e5-9dd9-4e2c-ba31-3b7f3ac7d1ff".to_string(),
                first_name: Some("Test".to_string()),
                last_name: Some("Test".to_string()),
                phone: "0644945778".to_string(),
                email: "toto@toto.fr".to_string(),
                password: "toto".to_string(),
            }
        }
        TestUser::Three => {
            User {
                uuid: "dfc5b455-9e8c-41ea-b746-7eda1703af5a".to_string(),
                first_name: Some("Thomas".to_string()),
                last_name: Some("Pesquet".to_string()),
                phone: "0699999999".to_string(),
                email: "thomas.pesquet@lunar.com".to_string(),
                password: "objectiflune".to_string(),
            }
        }
        TestUser::Four => {
            User {
                uuid: uuid::Uuid::new_v4().to_string(),
                first_name: Some("null".to_string()),
                last_name: Some("null".to_string()),
                phone: "null".to_string(),
                email: "null".to_string(),
                password: "null".to_string(),
            }
        }
    }
}

#[derive(Debug, FromRow)]
pub struct User {
    pub uuid: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: String,
    pub email: String,
    pub password: String,
}
