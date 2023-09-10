use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct MetricAPI {
    pub id: String,
    pub mtype: String,

    #[serde(default,skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(default,skip_serializing_if = "Option::is_none")]
    pub delta: Option<i64>,
}