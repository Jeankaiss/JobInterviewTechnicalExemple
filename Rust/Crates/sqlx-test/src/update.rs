use sqlx::{MySql, MySqlPool, mysql::{MySqlPoolOptions, MySqlRow}, query::Query, Row};
use crate::data::{User, init_db};

pub trait Updatable {
    fn table_name(&self) -> String;
    fn field_update(&self) -> String;
    fn field_value(&self) -> Vec<String>;
}

impl Updatable for User {
    fn table_name(&self) -> String {
        String::from("user")
    }

    fn field_update(&self) -> String {
        String::from("SET uuid = ?, first_name = ?, last_name = ?, phone = ?, email = ?, password = ?")
    }

    fn field_value(&self) -> Vec<String> {
        let first_name = match &self.first_name {
            Some(first_name) => String::from(first_name),
            None => String::from(""),    
        };
        let last_name = match &self.last_name {
            Some(last_name) => String::from(last_name),
            None => String::from(""),    
        };
        vec![
            self.uuid.to_string(),
            first_name,
            last_name,
            self.phone.to_string(),
            self.email.to_string(),
            self.password.to_string()
        ]
    }
}

struct UserPassword {
    password: String,
}

impl Updatable for UserPassword {
    fn table_name(&self) -> String {
        String::from("user")
    }

    fn field_update(&self) -> String {
        String::from("SET password = ?")
    }

    fn field_value(&self) -> Vec<String> {
        vec![self.password.to_string()]
    }
}

pub async fn update_user<T>(user: T)
where T: Updatable {
    let pool = init_db().await;
    let fields_value = user.field_value();
    println!("fields to update : {}", user.field_update());
    let query = format!("UPDATE {} {}", user.table_name(), user.field_update());
    println!("query = {}", query);

    let mut state : Query<'_, MySql, _> = sqlx::query(&query);
    for value in fields_value {
        state = state.bind(value)
    }
    state.execute(&pool)
    .await
    .map_err(|e| {
        println!("There was an error, error = {}", e);
    }).expect("Erreur");
}