use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::entities::comment::Comment;
use crate::entities::common::{default_vec, Image, PageData};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GamePageResponseData {
    pub games: PageData<GameSimple>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameSimple {
    #[serde(rename = "_id")]
    pub id: String,
    pub title: String,
    pub version: String,
    pub icon: Image,
    pub publisher: String,
    pub adult: bool,
    pub suggest: bool,
    #[serde(rename = "likesCount")]
    pub likes_count: i64,
    pub android: bool,
    pub ios: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameInfoResponseData {
    pub game: GameInfo,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameInfo {
    #[serde(rename = "_id")]
    pub id: String,
    pub title: String,
    pub version: String,
    pub icon: Image,
    pub publisher: String,
    pub adult: bool,
    pub suggest: bool,
    #[serde(rename = "likesCount")]
    pub likes_count: i64,
    pub android: bool,
    pub ios: bool,
    pub description: String,
    pub update_content: String,
    pub video_link: String,
    pub screenshots: Vec<Image>,
    #[serde(rename = "commentsCount")]
    pub comments_count: i64,
    #[serde(rename = "downloadsCount")]
    pub downloads_count: i64,
    #[serde(rename = "isLiked")]
    pub is_liked: bool,
    #[serde(rename = "androidLinks")]
    pub android_links: Vec<String>,
    #[serde(rename = "androidSize")]
    pub android_size: f32,
    #[serde(rename = "iosLinks")]
    pub ios_links: Vec<String>,
    #[serde(rename = "iosSize")]
    pub ios_size: f32,
    #[serde(rename = "updated_at")]
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameCommentsResponseData {
    pub comments: PageData<Comment>,
    #[serde(default = "default_vec")]
    pub top_comments: Vec<Comment>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameCommentChildrenResponseData {
    pub comments: PageData<Comment>,
}
