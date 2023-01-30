use crate::data::init_db;
use uuid::Uuid;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use sqlx::{Row, mysql::MySqlRow, Error, error::DatabaseError};

pub struct Request {
    uuid: String,
    title: String,
}

pub struct Message {
    uuid: String,
    r_uuid: String,
    text: String,
}

pub async fn generate_random_request() {
    let pool = init_db().await;
    let mut uuids = Vec::new();
    for _i in 0..10 {
        let uuid = Uuid::new_v4();
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        sqlx::query(
            "
                INSERT INTO request
                    (uuid, request_title)
                VALUES
                    (?, ?)
            ")
            .bind(uuid.to_string())
            .bind(rand_string)
            .execute(&pool)
            .await
            .expect("Programme planted");
        uuids.push(uuid);
    }

    for r_uuid in uuids {
        for _i in 0..5 {
            let uuid = Uuid::new_v4();
            let rand_string: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(30)
                .map(char::from)
                .collect();
            sqlx::query(
                "
                    INSERT INTO request_message
                        (uuid, request_uuid, message_text)
                    VALUES
                        (?, ?, ?)
                ")
                .bind(uuid.to_string())
                .bind(r_uuid.to_string())
                .bind(rand_string)
                .execute(&pool)
                .await
                .expect("Programme planted");
        }
    }
}

// pub async fn test_relations() {
//     let pool = init_db().await;
//     let request : Vec<Request> = Vec::new();
//     let rows = sqlx::query(
//         "
//             SELECT re.uuid, re.request_title, rm.uuid, rm.request_uuid, rm.message_text
//             FROM request as re 
//             JOIN request_message as rm
//             ON re.uuid = rm.request_uuid
//             WHERE re.uuid = ?
//         "
//     )
//     .bind(String::from("bc38200b-0165-4ef5-a34b-1637cffab8d4"))
//     .fetch_all(&pool)
//     .await
//     .expect("programme planted");
//     // println!("rows = {:#?}", rows);
//     for row in rows {
//         if request.iter().any(|r| r.uuid == row.get(0) as String) {

//         }
//         // println!("#################################################");
//         // let r_uuid: String = row.get(0);
//         // println!(" r_uuid = {}", r_uuid);
//         // let r_title: String = row.get(1);
//         // println!(" r_title = {}", r_title);
//         // let rm_uuid: String = row.get(2);
//         // println!(" rm_uuid = {}", rm_uuid);
//         // let rm_r_uuid: String = row.get(3);
//         // println!(" rm_r_uuid = {}", rm_r_uuid);
//         // let rm_message: String = row.get(4);
//         // println!(" rm_message = {}", rm_message);
//         // println!("#################################################");
//     }
// }

pub async fn test_error() {
    let pool = init_db().await;
    sqlx::query(
        "
            INSERT INTO user
                (uuid, first_name, last_name, phone, email, password)
            VALUES
                (?, ?, ?, ?, ?, ?)
        "
    )
    .bind(String::from("5c11531e-f6ad-4d7f-86e0-88827b19506d"))
    .bind(String::from("Tata"))
    .bind(String::from("Toto"))
    .bind(String::from("0654789878"))
    .bind(String::from("toto@toto.toto"))
    .bind(String::from("test"))
    .execute(&pool)
    .await
    .map_err(|e| {
        // println!("une erreur est survenue : {}", e);
        // println!("code erreur : {}", e.as_database_error().unwrap().code().unwrap());
        // println!("message erreur : {}", e.as_database_error().unwrap().message());
        let test = format!("{}", e.as_database_error().unwrap().code().unwrap());
        if test.eq("23000") == true {
            println!("test worked");
        }
        // println!("constrainte erreur : {}", e.as_database_error().unwrap().constraint().unwrap()); // sqlx version 5
    }).unwrap(); 
}

pub async fn test_not_found() {
    let pool = init_db().await;
    sqlx::query("SELECT * from user where uuid = 'qdsfqsdfqsd'").fetch_one(&pool).await
        .map_err(|e| {
            println!("une erreur est survenue : {}", e);
            match e {
                sqlx::Error::RowNotFound => println!("row not found"),
                _ => println!("other error"),
            }
            // println!("message erreur : {}", e.as_database_error().unwrap().message());
            // let test = format!("{}", e.as_database_error().unwrap().code().unwrap());
            // println!("test : {}", test);
            // if test.eq("23000") == true {
            //     println!("test worked");
            // }
        }).unwrap();
}


// TODO Vérifier si le paramètre générique fonctionne bien
// Normalement Send est auto implémenté pour String et u32 et bool
// pub async fn update_tire_by_string<'q, T>(
//     update_by: &str,
// 	update_value: &'q T,
// 	tire_uuid: &'q str,
// 	pool: &MySqlPool,
// ) -> Result<(), Error>
// where T: Encode<'q, MySql> + Type<MySql> + std::marker::Sync {
//     let query_span = tracing::info_span!(UPDATE_TIRE_BY);
//     if let None = UPDATE_AUTHORIZATION.iter().find(|&&s| s == update_by) {
// 		tracing::error!("Unauthorized user update, update was : {}", update_by);
// 		return Err(ErrorUnauthorized(UNAUTHORIZED_UPDATE));

//     }
//     let query = format!(
// 		"
// 		UPDATE user
// 		SET {} = ?
// 		WHERE uuid = ?
// 	",
// 		update_by
// 	);
// 	let mut query: Query<'_, MySql, _> = sqlx::query::<MySql>(&query);
// 	query = query.bind(update_value);
// 	query = query.bind(tire_uuid);
//     query.execute(*&pool).instrument(query_span).await.map_err(|e| {
// 		tracing::error!("Failed to update tire field -> {}, an error occured: {}",
// update_by, e.to_string()); 		return ErrorInternalServerError(e);
// 	})?;
//     Ok(())
// }
