use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InitInfo {
    pub status: String,
    pub addresses: Vec<String>,
    pub waka: String,
    #[serde(rename = "adKeyword")]
    pub ad_keyword: String,
}
