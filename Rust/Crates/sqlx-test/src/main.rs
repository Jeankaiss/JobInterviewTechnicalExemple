#[allow(unused_imports)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]

pub mod data;
pub mod insert;
pub mod update;
pub mod select;
pub mod users;
pub mod playground;

use std::time::Instant;

use data::{
    TestUser,
    get_user,
};
use update::{
    update_user,
};

use insert::{
    insert_multiple_data,
    insert_single_data,
};

use select::{
    select_user,
};

use users::find_all_user_with_limit;

use crate::data::init_db;

use sqlx::{
    Row,
    mysql::MySqlRow,
};

#[tokio::main]
async fn main() {

    let now = Instant::now();

    //INSERT SINGLE
    // let user = get_user(TestUser::One);
    // insert_single_data(user).await;

    // INSERT MULTIPLE
    // for _j in 0..100 {
    //     let mut users = Vec::new();
    //     for _i in 0..500 {
    //         users.push(get_user(TestUser::One));
    //         users.push(get_user(TestUser::Two));
    //         users.push(get_user(TestUser::Three));
    //     }
    //     insert_multiple_data(users).await;
    // }

    // UPDATE
    // let user = get_user(TestUser::Four);
    // update_user(user).await;

    // SELECT
    // let user_list = select_user().await;
    // println!("Liste des utilisateurs en base de données : {:#?}", user_list);

    // TRUNCATE
    // let mut s = String::from("WHERE uuid = ?, ");
    // s.truncate(s.len() - 1);
    // println!("s = {}", s);

    // LOOP
    // let v = vec!["1", "2", "3"];
    // println!("len = {}", v.len());
    // for _i in 0..v.len() {
    //     println!("toto");
    // }

    // LIMIT
    // let users = find_all_user_with_limit(2, 5).await;
    // println!("vector result : {:#?}", users);
    // println!("vector len = {}", users.len());

    // let pool = init_db().await;
    // sqlx::query("UPDATE user set ? = ? WHERE uuid = ?")
    //     .bind(format!("\"{}\"", String::from("first_name")))
    //     .bind(String::from("ZAERZAERAZERAZERAZERAZERAZERAZER"))
    //     .bind(String::from("5c11531e-f6ad-4d7f-86e0-88827b19506d"))
    //     .execute(&pool).await.expect("programme planted");

    // let pool = init_db().await;
    // let row = sqlx::query("SELECT COUNT(uuid) FROM user")
    //     .fetch_one(&pool)
    //     .await.expect("programme planted");
    // let row_number : i64 = row.get(0);

    // println!("number of users : {}", row_number);

    // playground::generate_random_request().await;
    // playground::test_relations().await;
    // playground::test_error().await;
    playground::test_not_found().await;

        // UPDATE user set "first_name" = ZAERZAERAZERAZERAZERAZERAZERAZER WHERE uuid = 5c11531e-f6ad-4d7f-86e0-88827b19506d
    println!("Le programme a durée {} seconde(s)", now.elapsed().as_secs());
}
