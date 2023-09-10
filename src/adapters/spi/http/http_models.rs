use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Metrics {
    pub id: String,
    pub mtype: String,
    pub value: f64,
    pub delta: i64,
}