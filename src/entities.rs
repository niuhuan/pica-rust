extern crate serde;

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponseData {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComicListResponseData {
    pub comics: Vec<ComicSimple>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComicInfoResponseData {
    pub comic: ComicInfo,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub original_name: String,
    pub path: String,
    pub file_server: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComicSimple {
    #[serde(rename = "_id")]
    pub id: String,
    pub title: String,
    pub author: String,
    pub pages_count: i32,
    pub eps_count: i32,
    pub finished: bool,
    pub categories: Vec<String>,
    pub thumb: Image,
    pub likes_count: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComicInfo {
    #[serde(rename = "_id")]
    pub id: String,
    pub title: String,
    pub author: String,
    pub pages_count: i32,
    pub eps_count: i32,
    pub finished: bool,
    pub categories: Vec<String>,
    pub thumb: Image,
    pub likes_count: i32,
    #[serde(rename = "_creator")]
    pub creator: Creator,
    pub description: String,
    pub chinese_team: String,
    pub tags: Vec<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub allow_download: bool,
    pub views_count: i32,
    pub is_liked: bool,
    pub comments_count: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
    #[serde(rename = "_id")]
    pub id: String,
    pub gender: String,
    pub name: String,
    pub title: String,
    pub verified: bool,
    pub exp: i32,
    pub level: i32,
    pub characters: Vec<String>,
    pub avatar: Image,
    pub slogan: String,
    pub role: String,
    pub character: String,
}
