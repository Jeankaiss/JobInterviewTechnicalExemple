use uuid::Uuid;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

#[derive(Debug)]
pub struct Request {
    pub uuid: String,
    pub messages: Vec<Message>,
}

#[derive(Clone, Debug)]
pub struct Message {
    req_uuid: String,
    text: String,
}

pub fn create_requests() -> Vec<Request> {
    let mut requests = Vec::new();
    for _i in 0..3 {
        requests.push(Request {
            uuid: Uuid::new_v4().to_string(),
            messages: Vec::new(),
        });
    }
    requests
}

pub fn create_messages(uuids: &Vec<String>) -> Vec<Message> {
    let mut messages = Vec::new();
    let number = uuids.len();
    for _i in (0..10) {
        let uuid_number = rand::thread_rng().gen_range(0..number);
        let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
        messages.push(Message {
            req_uuid: uuids[uuid_number].clone(),
            text: rand_string,
        });
    }
    messages
}

fn main() {
    let mut requests = create_requests();
    let mut uuid = Vec::new();
    requests.iter().for_each(|req| uuid.push(req.uuid.clone()));
    let messages = create_messages(&uuid);

    println!("start processing");
    for req in &mut requests {
        for msg in &messages {
            if msg.req_uuid == req.uuid {
                req.messages.push(msg.clone());
            }
        }        
    }
    println!("end processing");

    println!("joined uuids : {}", uuid.join(", "));
    println!("affichage des requests : {:#?}", requests);
}
