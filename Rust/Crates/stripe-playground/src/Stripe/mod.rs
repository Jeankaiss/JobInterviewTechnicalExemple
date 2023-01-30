pub mod Charges;
pub mod Data;
pub mod PaymentMethodOptions;
pub mod BillingDetails;
pub mod Outcome;
pub mod PaymentMethodDetails;
pub mod Refunds;
pub mod PaymentIntentStatus;
pub mod PaymentIntentNextAction;

use std::collections::HashMap;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};

pub type Timestamp = i64;
pub type Metadata = HashMap<String, String>;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PaymentIntent {
    pub id: Option<String>,
    pub object: Option<String>,
    pub amount: i64,
    pub amount_capturable: Option<i64>,
    pub amount_received: Option<i64>,
    pub application: Option<String>,
    pub application_fee_amount: Option<i64>,
    pub automatic_payment_methods: Option<String>,
    pub canceled_at: Option<Timestamp>,
    pub cancellation_reason: Option<String>,
    pub capture_method: Option<String>,
    pub charges: Charges::Charges,
    pub client_secret: Option<String>,
    pub confirmation_method: Option<String>,
    pub confirm: Option<bool>,
    pub created: Option<Timestamp>,
    pub currency: String,
    pub customer: Option<String>,
    pub description: Option<String>,
    pub invoice: Option<String>,
    pub last_payment_error: Option<String>,
    pub livemode: Option<bool>,
    pub metadata: Metadata,
    pub next_action: Option<PaymentIntentNextAction::PaymentIntentNextAction>,
    pub on_behalf_of: Option<String>,
    pub payment_method: Option<String>,
    pub payment_method_options: Option<HashMap<String, PaymentMethodOptions::PaymentMethodOptions>>,
    pub payment_method_types: Option<Vec<String>>,
    pub processing: Option<String>,
    pub receipt_email: Option<String>,
    pub review: Option<String>,
    pub setup_future_usage: Option<bool>,
    pub shipping: Option<String>,
    pub source: Option<String>,
    pub statement_descriptor: Option<String>,
    pub statement_descriptor_suffix: Option<String>,
    pub status: Option<PaymentIntentStatus::PaymentIntentStatus>,
    pub transfer_data: Option<String>,
    pub transfer_group: Option<String>
}

impl PaymentIntent {
    pub fn new(
        amount: i64,
        currency: String,
        confirm: Option<bool>,
        payment_method: Option<String>,
    ) -> Self {
        let mut pi = PaymentIntent::default();
        pi.amount = amount;
        pi.currency = currency;
        pi.confirm = confirm;
        pi.payment_method = payment_method;
        pi
    }

    pub async fn create(self) -> Result<PaymentIntent, Error> {
        let stripe_key = dotenv::var("STRIPE_SECRET_KEY").expect("Uninitialized variable");

        let client = reqwest::Client::new();
        let res = client.post("https://api.stripe.com/v1/payment_intents")
            .basic_auth::<String, String>(stripe_key, None)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(serde_qs::to_string(&self).expect("SERDE QS FAILED"))
            .send()
            .await;

        match res {
            Ok(response) => {
                match response.status() {
                    StatusCode::OK => {
                        let pir = response.json::<PaymentIntent>().await;
                        // return Ok(pir)
                    },
                    StatusCode::BAD_REQUEST => {
                        // return InternalServerError()
                    },
                    _ => {}, // return InternalServerError()
                }
            },
            Err(_) => {},// return InternalServerError(),
        };
    }

    pub async fn confirm(self, pi_id: &str) -> Result<PaymentIntent, Error> {
        let stripe_key = dotenv::var("STRIPE_SECRET_KEY").expect("Uninitialized variable");

        let client = reqwest::Client::new();
        let res = client.post(format!("https://api.stripe.com/v1/payment_intents/{}/confirm", pi_id).as_str())
            .basic_auth::<String, String>(stripe_key, None)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(serde_qs::to_string(&self).expect("SERDE QS FAILED"))
            .send()
            .await;

        match res {
            Ok(response) => {
                match response.status() {
                    StatusCode::OK => {
                        let pir = response.json::<PaymentIntent>().await;
                        // return Ok(pir)
                    },
                    StatusCode::BAD_REQUEST => {
                        // return InternalServerError()
                    },
                    _ => {}, // return InternalServerError()
                }
            },
            Err(_) => {},// return InternalServerError(),
        };
    }
}
