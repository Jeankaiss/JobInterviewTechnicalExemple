use serde::{Serialize, Deserialize};
use crate::Stripe::Data::Data;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Charges {
    pub object: Option<String>,
    pub data: Vec<Data>,
    pub has_more: Option<bool>,
    pub total_count: Option<i64>,
    pub url: Option<String>
}
