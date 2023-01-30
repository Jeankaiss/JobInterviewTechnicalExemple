use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Checks {
    pub address_line1_check: Option<String>,
    pub address_postal_code_check: Option<String>,
    pub cvc_check: Option<String>
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Card {
    pub brand: Option<String>,
    pub checks: Option<Checks>,
    pub country: Option<String>,
    pub exp_month: Option<i64>,
    pub exp_year: Option<i64>,
    pub fingerprint: Option<String>,
    pub funding: Option<String>,
    pub installments: Option<String>,
    pub last4: Option<String>,
    pub network: Option<String>,
    pub three_d_secure: Option<String>,
    pub wallet: Option<String>,
    pub Type: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PaymentMethodDetails {
    pub card: Option<Card>,
    pub Type: Option<String>
}
