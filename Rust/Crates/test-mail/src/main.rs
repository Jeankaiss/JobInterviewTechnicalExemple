use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use rand::{distributions::Alphanumeric, Rng, thread_rng};

const SIB_API_KEY : &str = "xkeysib-TOTOKEY";

#[derive(Debug, Serialize, Default, Deserialize)]
pub struct OrderDetail {
    pub designation: String,
    pub price: f32,
    pub quantity: u32,
}

impl OrderDetail {
    pub fn get_some(number: u32) -> Vec<OrderDetail> {
        let mut od = Vec::new();
        for _ in 0..number {
            od.push(OrderDetail {
                designation: rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(40)
                .map(char::from)
                .collect(),
                price: thread_rng().gen_range(115.0..365.0),
                quantity: thread_rng().gen_range(0..10),
            })
        }
        od
    }
}

#[derive(Debug, Serialize, Default)]
pub struct Contact {
    email: String,
    name: Option<String>,
}
#[derive(Debug, Serialize, Default)]
pub struct Email {
    sender: Contact,
    #[serde(rename = "to")]
    receiver: Vec<Contact>,
    subject: String,
    #[serde(rename = "htmlContent")]
    html: Option<String>
}

impl Email {
    pub fn new() -> Self {
        Email::default()
    }

    pub fn from(mut self, sender: Contact) -> Self {
        self.sender = sender;
        self
    }

    pub fn to(mut self, receiver: Contact) -> Self {
        self.receiver.push(receiver);
        self
    }

    pub fn subject(mut self, subject: String) -> Self {
        self.subject = subject;
        self
    }

    pub fn body(mut self, body: Option<String>) -> Self {
        self.html = body;
        self
    }

    pub async fn send(self) -> Result<String, String> {
        let client = reqwest::Client::new();
        let response  = client.post("https://api.sendinblue.com/v3/smtp/email")
            .header("Accept", "application/json")
            .header("api-key", SIB_API_KEY)
            .json(&self)
            .send()
            .await
            .map_err(|e| format!("Error occured : {}", e))?;

        let status = response.status().as_u16();
        let mut body: HashMap<String, String> = response
            .json()
            .await
            .map_err(|e| format!("Error occured : {}", e))?;

        match status {
            201 => Ok(body.remove("messageId").unwrap_or("".to_string())),
            _ => {
                let message = body.remove("message").unwrap_or("Unknown error".to_string());
                Err(message)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let po = OrderDetail::get_some(10);

    let mut order_table = format!(
		r#"
		<table>
			<thead>
				<tr>
					<th>Pneu</th>
					<th>Prix</th>
					<th>Quantié</th>
					<th>Total</th>
				</tr>
			</thead>
			<tbody>
	"#
	);

	for product in &po {
		order_table = format!(
			" {}
			<tr>
				<td>{}</td>
				<td>{}</td>
				<td>{}</td>
				<td>{}</td>
			</tr>
			",
			order_table,
			product.designation,
			product.price,
			product.quantity,
			(product.price * product.quantity as f32)
		);
	}

	order_table = format!(
		"
		{}
			</tbody>
		</table>
		",
		order_table
	);

	let mut email_corpse = format!(
		r#"
		<!DOCTYPE html>
		<html>
			<body style="background-color: #f1f1f1">
				<div id="container" style="background-color: white;">
					<div id="container-head" style="background-color: #2e3841; color: white; text-align: center; padding: 10px;">
						<h1 style="padding: 0px; margin: 0px;">Pneu-privé.fr</h1>
					</div>
					<div id="container-body" style="background-color: white; padding: 5%; font-size: 1.3em;">
						<p>Chèr(e) cliente / client</p>
						<p>
							Nous vous remercions pour votre commande.
							Veuillez trouver ci-joint le récapitulatif de votre commande.
						</p>
	"#
	);
	email_corpse = format!("{} {}", email_corpse, order_table);
	email_corpse = format!(
		"{}
						<p>Cordialement l'équipe de Pneu-privé.fr</p>
					</div>
				</div>
			</body>
		</html>
	",
		email_corpse
	);

    let res = Email::new()
        .from(Contact {name : Some(String::from("TOTO TITI")), email : String::from("TOTO.TITI@gmail.com")})
        .to(Contact {name : Some(String::from("TOTO TITI")), email : String::from("TOTO.TITI@gmail.com")})
        .subject(String::from("Hello from rust test with send in blue"))
        .body(Some(String::from(email_corpse)))
        .send()
        .await;

    println!("res = {}", res.unwrap());
}

// let res = Email::new()
//         .from(Contact {name : Some(String::from("TOTO TITI")), email : String::from("TOTO.TITI@gmail.com")})
//         .to(Contact {name : Some(String::from("TOTO TITI")), email : String::from("TOTO.TITI@gmail.com")})
//         // .to(Contact {name : Some(String::from("TOTO TITI")), email : String::from("TOTO.TITI@gmail.com")})
//         // .to(Contact {name : Some(String::from("TOTO TITI")), email : String::from("TOTO.TITI@gmail.com")})
//         .subject(String::from("Hello from rust test with send in blue"))
//         .body(Some(String::from(
//             r#"
//             <!DOCTYPE html>
//             <html>
//                 <body style="background-color: #f1f1f1">
//                     <div id="container" style="background-color: white;">
//                         <div id="container-head" style="background-color: #2e3841; color: white; text-align: center; padding: 10px;">
//                             <h1 style="padding: 0px; margin: 0px;">Pneu-privé.fr</h1>
//                         </div>
//                         <div id="container-body" style="background-color: white; padding: 5%; font-size: 1.3em;">
//                             <p>Chèr(e) cliente / client</p>
//                             <p>
//                                 Nous avons bien pris en compte votre demande de changement d'email.
//                                 Afin de terminer la procedure veuillez cliquer sur le lien ci dessous.
//                             </p>
//                             <p>Bien Cordialement l'équipe de Pneu-privé.fr.</p>
//                             <div style="background-color: #37cdff; padding: 15px; margin-left: 30%; margin-top: 50px; width: 350px; text-align: center; border-radius: 30px;">
//                                 <a href="" style="color: white; text-decoration: none;">Confirmer votre changement d'email</a>
//                             </div>
//                         </div>
//                     </div>
//                 </body>
//             </html>
//             "#
//         )))
//         .send()
//         .await;

//     println!("res = {}", res.unwrap());
