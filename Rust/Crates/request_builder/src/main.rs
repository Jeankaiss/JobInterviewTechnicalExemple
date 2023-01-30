use uuid::Uuid;

pub struct User {
    pub uuid: Uuid,
    pub name: String,
    pub address: String,
}

fn main() {
    let user = User {
        uuid: Uuid::new_v4(),
        name: String::from("TOTO"),
        address: String::from("TOTO ADRESSE")
    };
    let addr_types = vec!["delivery", "billing", "assembly_station"];

    let mut query = String::from("INSERT INTO address
    (uuid, user_uuid, address, addr_type)
    VALUES ");

    for addr_type in addr_types {
        query = format!("{} ({}, {}, {}, {}), ",
            query, Uuid::new_v4().to_string(), user.uuid.to_string(), user.address, addr_type);
    }

    query.truncate(query.len() - 2);
    println!("query = {}", query);
}
