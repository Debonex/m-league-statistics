use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UMDGameItem {
    pub id: String,
    pub time: String,
    pub cmd: String,
    pub args: Vec<String>,
}
