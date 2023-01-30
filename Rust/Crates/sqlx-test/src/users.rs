use crate::data::{User, init_db};
use sqlx::Row;
use sqlx::FromRow;

pub async fn find_all_user_with_limit(
	page: u32,
	result_number: u32,
) -> Vec<User> {
	let pool = init_db().await;
	let mut users = Vec::new();
	let offset = (page - 1) * result_number;
	let results = sqlx::query(
		"
		SELECT * FROM user LIMIT ?, ?
	").bind(offset).bind(result_number)
	.fetch_all(&pool)
	.await
	.map_err(|e| {
		println!("ERROR");
	}).expect("ERRORS USER");
	for result in results {
		users.push(User {
			uuid: result.get(0),
			first_name: result.get(1),
			last_name: result.get(2),
			phone: result.get(3),
			email: result.get(4),
			password: result.get(5),
		})
	}
	users
}