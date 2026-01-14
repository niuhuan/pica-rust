use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::entities::common::{avatar_default, Image, PageData};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CategoriesResponseData {
    pub categories: Vec<Category>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    #[serde(rename = "_id")]
    pub id: String,
    pub title: String,
    pub description: String,
    pub thumb: Image,
    #[serde(rename = "isWeb")]
    pub is_web: bool,
    pub active: bool,
    pub link: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComicListResponseData {
    pub comics: Vec<ComicSimple>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComicPageResponseData {
    pub comics: PageData<ComicSimple>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComicInfoResponseData {
    pub comic: ComicInfo,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComicEpsResponseData {
    pub eps: PageData<ComicEp>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComicEpPicturePageResponseData {
    pub pages: PageData<ComicEpPicture>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComicSearchResponseData {
    pub comics: PageData<ComicInSearch>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CollectionsResponseData {
    pub collections: Vec<Collection>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComicSimple {
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
    #[serde(default)]
    pub likes_count: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
    #[serde(default)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComicEp {
    #[serde(rename = "_id")]
    pub id: String,
    pub title: String,
    pub order: i32,
    #[serde(rename = "updated_at")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
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
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub character: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComicEpPicture {
    #[serde(rename = "_id")]
    pub id: String,
    pub media: Image,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub title: String,
    pub comics: Vec<ComicSimple>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Knight {
    #[serde(flatten)]
    pub creator: Creator,
    #[serde(rename = "comicsUploaded")]
    pub comics_uploaded: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardOfKnightResponseData {
    pub users: Vec<Knight>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HotKeywordsResponseData {
    pub keywords: Vec<String>,
}
