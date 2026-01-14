use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::entities::common::{avatar_default, default_string, Image};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserProfileResponseData {
    pub user: UserProfile,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PunchResponseData {
    pub res: PunchStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    #[serde(rename = "_id")]
    pub id: String,
    pub gender: String,
    pub name: String,
    pub title: String,
    pub verified: bool,
    pub exp: i32,
    pub level: i32,
    pub characters: Vec<String>,
    #[serde(default = "avatar_default")]
    pub avatar: Image,
    pub birthday: String,
    pub email: String,
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,
    pub is_punched: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PunchStatus {
    #[serde(default = "default_string")]
    pub status: String,
    #[serde(default = "default_string")]
    pub punch_in_last_day: String,
}
