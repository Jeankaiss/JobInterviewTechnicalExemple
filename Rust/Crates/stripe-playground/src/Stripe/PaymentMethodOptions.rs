use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Card {
    pub installments: Option<String>,
    pub network: Option<String>,
    pub request_three_d_secure: Option<String>
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PaymentMethodOptions {
    pub card: Option<Card>
}
