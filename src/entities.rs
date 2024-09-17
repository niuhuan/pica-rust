extern crate serde;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::num::ParseIntError;

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

#[derive(Debug, Serialize, Deserialize)]
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

fn fuzzy_i32<'de, D>(d: D) -> std::result::Result<i32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: serde_json::Value = serde::Deserialize::deserialize(d)?;
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
pub struct ComicSearchResponseData {
    pub comics: PageData<ComicInSearch>,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub original_name: String,
    pub path: String,
    pub file_server: String,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
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

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComicInfo {
    #[serde(rename = "_id")]
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub author: String,
    pub pages_count: i32,
    pub eps_count: i32,
    pub finished: bool,
    pub categories: Vec<String>,
    pub thumb: Image,
    pub likes_count: i32,
    #[serde(rename = "_creator")]
    pub creator: Creator,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
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

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComicInSearch {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(default)]
    pub author: String,
    pub categories: Vec<String>,
    #[serde(default, rename = "chineseTeam")]
    pub chinese_team: String,
    pub created_at: String,
    #[serde(default)]
    pub description: String,
    pub finished: bool,
    #[serde(rename = "likesCount")]
    pub likes_count: i64,
    pub tags: Vec<String>,
    pub thumb: Image,
    pub title: String,
    #[serde(rename = "totalLikes")]
    pub total_likes: Option<i64>,
    #[serde(rename = "totalViews")]
    pub total_views: Option<i64>,
    pub updated_at: String,
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

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
    #[serde(rename = "_id")]
    pub id: String,
    pub gender: String,
    pub name: String,
    pub title: String,
    pub verified: Option<bool>,
    pub exp: i32,
    pub level: i32,
    pub characters: Vec<String>,
    #[serde(default = "avatar_default")]
    pub avatar: Image,
    #[serde(default)]
    pub slogan: String,
    pub role: String,
    #[serde(default)]
    pub character: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComicEpPicture {
    #[serde(rename = "_id")]
    pub id: String,
    pub media: Image,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    #[serde(default = "default_string")]
    pub action: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentsResponse {
    pub comments: PageData<Comment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    #[serde(rename = "_id")]
    pub id: String,
    pub content: String,
    #[serde(rename = "_user")]
    pub user: CommentUser,
    #[serde(rename = "isTop")]
    pub is_top: bool,
    pub hide: bool,
    pub created_at: String,
    #[serde(rename = "likesCount")]
    pub likes_count: i64,
    #[serde(rename = "commentsCount")]
    pub comments_count: i64,
    #[serde(rename = "isLiked")]
    pub is_liked: bool,
    #[serde(rename = "_comic", default = "default_string")]
    pub comic: String,
    #[serde(rename = "_game", default = "default_string")]
    pub game: String,
    #[serde(rename = "_parent", default = "default_string")]
    pub parent: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentUser {
    #[serde(rename = "_id")]
    pub id: String,
    pub gender: String,
    pub name: String,
    pub title: String,
    pub verified: bool,
    pub exp: i64,
    pub level: i64,
    #[serde(default = "default_vec")]
    pub characters: Vec<String>,
    #[serde(default = "avatar_default")]
    pub avatar: Image,
    pub role: String,
}

fn avatar_default() -> Image {
    Image {
        file_server: "".to_string(),
        path: "".to_string(),
        original_name: "".to_string(),
    }
}

fn default_vec<T>() -> Vec<T> {
    vec![]
}

fn default_string() -> String {
    "".to_string()
}
