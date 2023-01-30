use serde::{Serialize, Deserialize};
use serde_json::value::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentNextActionType {
    RedirectToUrl,
    UseStripeSdk,
    Other,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PaymentIntentNextActionRedirectToUrl {
    pub return_url: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PaymentIntentNextAction {
    pub Type: Option<PaymentIntentNextActionType>,
    pub redirect_to_url: Option<PaymentIntentNextActionRedirectToUrl>,
    pub use_stripe_sdk: Option<Value>,
}
