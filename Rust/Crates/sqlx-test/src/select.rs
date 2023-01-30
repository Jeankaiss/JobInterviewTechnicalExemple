use crate::data::{User, init_db};
use sqlx::Row;

pub async fn select_user() -> Vec<User> {
    let pool = init_db().await;
    let mut user_list = Vec::new();

    let results = sqlx::query("SELECT * FROM user")
        .fetch_all(&pool)
        .await;

    match results {
        Ok(results) => {
            for result in results {
                user_list.push(
                    User {
                        uuid: result.get(0),
                        first_name: result.get(1),
                        last_name: result.get(2),
                        phone: result.get(3),
                        email: result.get(4),
                        password: result.get(5),
                    }
                );
            }
        }
        Err(e) => return Vec::new(),
    };
    return user_list;
}