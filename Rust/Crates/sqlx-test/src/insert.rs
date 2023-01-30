use sqlx::{MySql, query::Query};

use crate::data::{User, init_db};

pub trait Insertable {
    fn table_name(&self) -> String;
    fn insert_fields_name(&self) -> String;
    fn insert_field_bind(&self) -> String;
    fn insert_values(&self) -> Vec<String>;
}

impl Insertable for User {
    
    fn table_name(&self) -> String {
        String::from("user")
    }
    
    fn insert_fields_name(&self) -> String {
        String::from("(uuid, first_name, last_name, phone, email, password)")
    }

    fn insert_field_bind(&self) -> String {
        String::from("(?, ?, ?, ?, ?, ?)")
    }

    fn insert_values(&self) -> Vec<String> {
        let first_name = match &self.first_name {
            Some(first_name) => String::from(first_name),
            None => String::from(""),
        };
        let last_name = match &self.last_name {
            Some(last_name) => String::from(last_name),
            None => String::from(""),
        };
        vec![
            first_name,
            last_name,
            self.phone.to_string(),
            self.email.to_string(),
            self.password.to_string(),
        ]
    }
}

pub async fn insert_multiple_data<T>(datas: Vec<T>) //-> Vec<String>
where T: Insertable {
    let pool = init_db().await;

    let mut binds = Vec::new();
    for data in datas.iter() {
        binds.push(data.insert_field_bind());
    }
    let binds = binds.join(", ");

    let query = format!("INSERT INTO {}
        {}
        VALUES {}", datas[0].table_name(), datas[0].insert_fields_name(), binds);

    let mut query: Query<'_, MySql, _> = sqlx::query(&query);

    for item in datas.iter() {
        query = query.bind(uuid::Uuid::new_v4().to_string());
        for value in item.insert_values() {
            query = query.bind(value);
        }
    }
    query.execute(&pool)
    .await
    .map_err(|e| {
        println!("There was an error, error = {}", e);
    }).expect("Erreur");
}

pub async fn insert_single_data<T>(data: T) //-> Vec<String>
where T: Insertable {
    let pool = init_db().await;

    let query = format!("
        INSERT INTO {}
        {}
        VALUES {}", data.table_name(), data.insert_fields_name(), data.insert_field_bind()
    );

    let mut query: Query<'_, MySql, _> = sqlx::query(&query);

    query = query.bind(uuid::Uuid::new_v4().to_string());
    for value in data.insert_values() {
        query = query.bind(value);
    }
    query.execute(&pool)
    .await
    .map_err(|e| {
        println!("There was an error, error = {}", e);
    }).expect("Erreur");
}