extern crate serde;

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

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

pub struct SwitchAddress(&'static str);

impl SwitchAddress {
    pub const ADDRESS1: Option<SwitchAddress> = Some(SwitchAddress("172.67.7.24:443"));
    pub const ADDRESS2: Option<SwitchAddress> = Some(SwitchAddress("104.20.180.50:443"));
    pub const ADDRESS3: Option<SwitchAddress> = Some(SwitchAddress("172.67.208.169:443"));

    pub fn as_str(&self) -> &'static str {
        self.0
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterDto {
    // 邮箱
    pub email: String,
    // 8字以上
    pub password: String,
    // 2 - 50 字
    pub name: String,
    // 2012-01-01
    pub birthday: String,
    // m, f, bot
    pub gender: String,
    pub answer1: String,
    pub answer2: String,
    pub answer3: String,
    pub question1: String,
    pub question2: String,
    pub question3: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponseData {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfileResponseData {
    pub user: UserProfile,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PunchResponseData {
    pub res: PunchStatus,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PunchStatus {
    status: String,
    #[serde(default = "default_string")]
    punch_in_last_day: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageData<T> {
    pub total: i32,
    pub limit: i32,
    pub page: i32,
    pub pages: i32,
    pub docs: Vec<T>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComicListResponseData {
    pub comics: Vec<ComicSimple>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComicPageResponseData {
    pub comics: PageData<ComicSimple>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComicInfoResponseData {
    pub comic: ComicInfo,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComicEpsResponseData {
    pub eps: PageData<ComicEp>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComicEpPicturePageResponseData {
    pub pages: PageData<ComicEpPicture>,
    // pub ep: ComicEp // no order, todo
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
pub struct ComicEp {
    #[serde(rename = "_id")]
    pub id: String,
    pub title: String,
    pub order: i32,
    #[serde(rename = "updated_at")]
    pub updated_at: DateTime<Utc>,
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
    #[serde(default = "avatar_default")]
    pub avatar: Image,
    pub slogan: String,
    pub role: String,
    pub character: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComicEpPicture {
    #[serde(rename = "_id")]
    pub id: String,
    pub media: Image,
}

fn avatar_default() -> Image {
    Image {
        file_server: "".to_string(),
        path: "".to_string(),
        original_name: "".to_string(),
    }
}

fn default_string() -> String { "".to_string() }
