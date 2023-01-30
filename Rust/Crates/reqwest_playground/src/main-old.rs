#![allow(non_snake_case)]
use serde::{Serialize, Deserialize};
use dotenv;
// use std::collections::HashMap;

// {
//     "easy4dcode": "S938398",
//     "ean": "6956647620238",
//     "designation": "205/55 R 16 91H RANDOMBRAND TX1",
//     "quantity": 5,
//     "unitprice": "36,92",
//     "deliveryestimated": "21/01/2021",
//     "errorcode": 0,
//     "quantity_MD": 5,
//     "unitprice_MD": "36,92",
//     "deliveryestimated_MD": "21/01/2021"
//   }

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub easy4dcode: String,
    pub ean: String,
    pub designation: String,
    pub quantity: String,
    pub unitprice: String,
    pub deliveryestimated: String,
    pub errorcode: i32,
    pub quantity_MD: String,
    pub unitprice_MD: String,
    pub deliveryestimated_MD: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Provider {
    #[serde(rename = "CustomerNumber")]
    customer_number: String,
    #[serde(rename = "AuthenticationToken")]
    auth_token: String,
    #[serde(rename = "ArticleCode")]
    article_code: String,
    #[serde(rename = "Quantity")]
    quantity: i32,
    #[serde(rename = "typerequete")]
    request_type: String,
}

impl Provider {
    fn new() -> Self {
        let code = dotenv::var("EASY4D_CODE").expect("Uninitialized variable");
        let token = dotenv::var("EASY4D_TOKEN"). expect("Unitialized variable");
        let mut provider = Provider::default();
        provider.customer_number = code;
        provider.auth_token = token;
        provider.request_type = String::from("MD");
        provider
    }

    fn article_code(mut self, code: String) -> Self {
        self.article_code = code;
        self
    }

    fn quantity(mut self, quantity: i32) -> Self {
        self.quantity = quantity;
        self
    }

    async fn send(self) {
        let url = dotenv::var("PROVIDER_URL").expect("Unitialized variable");
        println!("URL = {}", url);

        let url = format!("{}?CustomerNumber={}&AuthenticationToken={}&ArticleCode={}&Quantity={}&typerequete={}",
            url, self.customer_number, self.auth_token, self.article_code, self.quantity, self.request_type);
        let client = reqwest::Client::new();
		let response = client
			.get(&url)
            .send()
            .await
            .map_err(|e| {
                println!("Unable to send request an error occured : {:#?}", e);
            }).expect("La requete planté");

        println!("RESPONSE CODE : {}", response.status().as_u16());

        let body: Product = response.json().await.map_err(|e| {
            println!("Unable to get body response an error occured : {:#?}", e);
		}).expect("Impossible de récupérer le body");

        println!("BODY = {:#?}", body);
    }
}

#[tokio::main]
async fn main() {
    Provider::new()
        .article_code(String::from("4981910765547"))
        .quantity(50)
        .send()
        .await;
}
