use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::num::ParseIntError;

/// 排序方式
#[derive(Clone, Copy, Debug, Default)]
pub struct Sort(&'static str);

impl Sort {
    pub const SORT_DEFAULT: Sort = Sort("ua");
    pub const SORT_TIME_NEWEST: Sort = Sort("dd");
    pub const SORT_TIME_OLDEST: Sort = Sort("da");
    pub const SORT_LIKE_MOST: Sort = Sort("ld");
    pub const SORT_VIVE_MOST: Sort = Sort("vd");

    pub fn as_str(&self) -> &'static str {
        self.0
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PageData<T> {
    #[serde(deserialize_with = "fuzzy_i32")]
    pub total: i32,
    #[serde(deserialize_with = "fuzzy_i32")]
    pub limit: i32,
    #[serde(deserialize_with = "fuzzy_i32")]
    pub page: i32,
    #[serde(deserialize_with = "fuzzy_i32")]
    pub pages: i32,
    pub docs: Vec<T>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub original_name: String,
    pub path: String,
    pub file_server: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    #[serde(default = "default_string")]
    pub action: String,
}

fn fuzzy_i32<'de, D>(d: D) -> std::result::Result<i32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: Value = serde::Deserialize::deserialize(d)?;
    if value.is_i64() {
        Ok(value.as_i64().unwrap() as i32)
    } else if value.is_string() {
        let str = value.as_str().unwrap();
        let from: std::result::Result<i32, ParseIntError> = std::str::FromStr::from_str(str);
        match from {
            Ok(from) => Ok(from),
            Err(_) => Err(serde::de::Error::custom("parse error")),
        }
    } else {
        Err(serde::de::Error::custom("type error"))
    }
}

pub fn avatar_default() -> Image {
    Image {
        file_server: "".to_string(),
        path: "".to_string(),
        original_name: "".to_string(),
    }
}

pub fn default_vec<T>() -> Vec<T> {
    vec![]
}

pub fn default_string() -> String {
    "".to_string()
}
