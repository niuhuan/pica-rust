use serde::{Deserialize, Serialize};

use crate::entities::common::{avatar_default, default_string, default_vec, Image, PageData};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CommentsResponse {
    pub comments: PageData<Comment>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
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

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
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
    #[serde(default)]
    pub role: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CommentChildrenResponseData {
    pub comments: PageData<Comment>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MyCommentsResponseData {
    pub comments: PageData<MyComment>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MyComment {
    #[serde(rename = "_id")]
    pub id: String,
    pub content: String,
    #[serde(rename = "_comic")]
    pub comic: MyCommentComic,
    pub hide: bool,
    pub created_at: String,
    #[serde(rename = "likesCount")]
    pub likes_count: i64,
    #[serde(rename = "commentsCount")]
    pub comments_count: i64,
    #[serde(rename = "isLiked")]
    pub is_liked: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MyCommentComic {
    #[serde(rename = "_id")]
    pub id: String,
    pub title: String,
}
