#![allow(non_snake_case)]
#![allow(unused)]
use serde::{Serialize, Deserialize};
use dotenv;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderStock {
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
    params: Vec<(String, String)>
}

impl Provider {
    fn new() -> Self {
        let code = dotenv::var("EASY4D_CODE").expect("Uninitialized variable");
        let token = dotenv::var("EASY4D_TOKEN"). expect("Unitialized variable");
        Provider { 
            params : vec![
                (String::from("CustomerNumber"), code),
                (String::from("AuthenticationToken"), token),
                (String::from("typerequete"), String::from("MD"))
            ]
        }
    }

    fn quantity(mut self, quantity: String) -> Self {
        self.params.push((String::from("Quantity"), quantity));
        self
    }

    fn article_code(mut self, article_code: String) -> Self {
        self.params.push((String::from("ArticleCode"), article_code));
        self
    }

    async fn send(self) -> ProviderStock {
        let url = dotenv::var("PROVIDER_URL").expect("uninitialized variable");

        let client = reqwest::Client::new();
		let response = client
			.get(&url)
            .query(&self.params)
            .send()
            .await
            .map_err(|e| {
                println!("Unable to send request an error occured : {:#?}", e);
            }).expect("La requete planté");

        let body: ProviderStock = response.json().await.map_err(|e| {
            println!("Unable to get body response an error occured : {:#?}", e);
        }).expect("Impossible de récupérer le body");

        body
    }

}

#[tokio::main]
async fn main() {
    println!("Second essai!");
    let stock = Provider::new()
        .quantity(String::from("4"))
        .article_code(String::from("4981910765547"))
        .send()
        .await;
    println!("provider stock = {:#?}", stock);
}
