use serde::{Serialize, Deserialize};

use crate::Stripe::BillingDetails::BillingDetails;
use crate::Stripe::Outcome::Outcome;
use crate::Stripe::PaymentMethodDetails::PaymentMethodDetails;
use crate::Stripe::Refunds::Refunds;
use crate::Stripe::{Metadata, Timestamp};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FraudDetails {
    pub stripe_report: Option<String>,
    pub user_report: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Data {
    pub id: Option<String>,
    pub object: Option<String>,
    pub amount: Option<i64>,
    pub amount_captured: Option<i64>,
    pub amount_refunded: Option<i64>,
    pub application: Option<String>,
    pub application_fee: Option<String>,
    pub application_fee_amount: Option<i64>,
    pub balance_transaction: Option<String>,
    pub billing_details: BillingDetails,
    pub calculated_statement_descriptor: Option<String>,
    pub captured: Option<bool>,
    pub created: Option<Timestamp>,
    pub currency: Option<String>,
    pub customer: Option<String>,
    pub description: Option<String>,
    pub destination: Option<String>,
    pub dispute: Option<String>,
    pub disputed: Option<bool>,
    pub failure_code: Option<String>,
    pub failure_message: Option<String>,
    pub fraud_details: Option<FraudDetails>,
    pub invoice: Option<String>,
    pub livemode: Option<bool>,
    pub metadata: Option<Metadata>,
    pub on_behalf_of: Option<String>,
    pub order: Option<String>,
    pub outcome: Option<Outcome>,
    pub paid: Option<bool>,
    pub payment_intent: Option<String>,
    pub payment_method: Option<String>,
    pub payment_method_details: Option<PaymentMethodDetails>,
    pub receipt_email: Option<String>,
    pub receipt_number: Option<String>,
    pub receipt_url: Option<String>,
    pub refunded: Option<bool>,
    pub refunds: Option<Refunds>,
    pub review: Option<String>,
    pub shipping: Option<String>,
    pub source: Option<String>,
    pub source_transfer: Option<String>,
    pub statement_descriptor: Option<String>,
    pub statement_descriptor_suffix: Option<String>,
    pub status: Option<String>,
    pub transfer_data: Option<String>,
    pub transfer_group: Option<String>
}
