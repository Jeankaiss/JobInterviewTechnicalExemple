use sqlx::Error;
use sqlx::mysql::{MySql, MySqlPoolOptions, MySqlPool};
use sqlx::query::Query;
use crate::Schema::Tire;

use crate::Constants::{
    UNINITIALIZED_VARIABLE,
    DATABASE_CONNECTION_REFUSED,
};

pub async fn init_db() -> MySqlPool {
    let database_url = dotenv::var("DATABASE_URL").expect(UNINITIALIZED_VARIABLE);
    let db_max_connection = dotenv::var("DB_MAX_CON").expect(UNINITIALIZED_VARIABLE).parse::<u32>().unwrap();
    let pool = MySqlPoolOptions::new()
        .max_connections(db_max_connection)
        .connect(&database_url)
        .await
        .expect(DATABASE_CONNECTION_REFUSED);
    pool
}

pub async fn delete_easy4d_tires() -> Result<(), Error> {
    let pool = init_db().await;

    sqlx::query(
        "
            DELETE FROM tire
            WHERE is_easy4d = true
        "
    ).execute(&pool)
    .await?;
    Ok(())
}

fn build_query(tires: &Vec<Tire>) -> String {
    let mut query = String::from(
        "
            INSERT INTO tire
                (uuid, ean, designation, model, tire_type, brand, summer, winter, four_seasons,
                width, diameter, height, construction, tube_type, load_index, speed, runflat,
                fuel_consumption, wet_grip, noise, homologation, segment, price, promotion_raw,
                percent_promotion, stock, active, is_easy4d, easy4d_code)
            VALUES
        "
    );
    for _ in 0..tires.len() {
        query.push_str(&format!("(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?),"));
    }
    query.truncate(query.len() - 1);
    query
}

pub async fn db_add(tires: &Vec<Tire>, pool: &MySqlPool) -> Result<(), Error> {
    let query = build_query(&tires);
    let mut query: Query<'_, MySql, _> = sqlx::query(&query);
    
    for tire in tires {
        query = query.bind(&tire.uuid);
        query = query.bind(&tire.ean);
        query = query.bind(&tire.designation);
        query = query.bind(&tire.model);
        query = query.bind(&tire.tire_type);
        query = query.bind(&tire.brand);
        query = query.bind(&tire.summer);
        query = query.bind(&tire.winter);
        query = query.bind(&tire.four_seasons);
        query = query.bind(&tire.width);
        query = query.bind(&tire.diameter);
        query = query.bind(&tire.height);
        query = query.bind(&tire.construction);
        query = query.bind(&tire.tube_type);
        query = query.bind(&tire.load_index);
        query = query.bind(&tire.speed);
        query = query.bind(&tire.runflat);
        query = query.bind(&tire.fuel_consumption);
        query = query.bind(&tire.wet_grip);
        query = query.bind(&tire.noise);
        query = query.bind(&tire.homologation);
        query = query.bind(&tire.segment);
        query = query.bind(&tire.price);
        query = query.bind(&tire.promotion_raw);
        query = query.bind(&tire.percent_promotion);
        query = query.bind(&tire.stock);
        query = query.bind(&tire.active);
        query = query.bind(&tire.is_easy4d);
        query = query.bind(&tire.easy4d_code);
    }
    query.execute(&*pool).await?;

    Ok(())
}

pub async fn add_easy4d_tires(mut tires: Vec<Tire>) -> Result<(), Error> {
    let pool = init_db().await;

    let mut sp = tires.split_off(1000);
    
    while sp.len() > 1000 {
        db_add(&tires, &pool).await?;
        tires = sp;
        sp = tires.split_off(1000);
    }
    
    tires.append(&mut sp);
    db_add(&tires, &pool).await?;
    
    Ok(())
}
