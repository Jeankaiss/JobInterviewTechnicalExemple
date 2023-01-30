use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Outcome {
    pub network_status: Option<String>,
    pub reason: Option<String>,
    pub risk_level: Option<String>,
    pub risk_score: Option<i64>,
    pub seller_message: Option<String>,
    pub Type: Option<String>,
}
