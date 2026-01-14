use crate::client::Client;
use crate::entities::{ComicSimple, PageData, Sort};
use crate::types::Result;
use std::future::IntoFuture;
use std::pin::Pin;

/// 漫画查询 Builder，同步实现 IntoFuture，可直接 `.await`
pub struct ComicsBuilder<'a> {
    pub(crate) client: &'a Client,
    pub(crate) category: Option<String>,
    pub(crate) tag: Option<String>,
    pub(crate) author: Option<String>,
    pub(crate) creator_id: Option<String>,
    pub(crate) chinese_team: Option<String>,
    pub(crate) sort: Sort,
    pub(crate) page: i32,
}

impl<'a> ComicsBuilder<'a> {
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }
    pub fn creator_id(mut self, creator_id: impl Into<String>) -> Self {
        self.creator_id = Some(creator_id.into());
        self
    }
    pub fn chinese_team(mut self, ct: impl Into<String>) -> Self {
        self.chinese_team = Some(ct.into());
        self
    }
    pub fn sort(mut self, sort: Sort) -> Self {
        self.sort = sort;
        self
    }
    pub fn page(mut self, page: i32) -> Self {
        self.page = page;
        self
    }
}

impl<'a> IntoFuture for ComicsBuilder<'a> {
    type Output = Result<PageData<ComicSimple>>;
    type IntoFuture = Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'a>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .comics_exec(
                    self.category,
                    self.tag,
                    self.author,
                    self.creator_id,
                    self.chinese_team,
                    self.sort,
                    self.page,
                )
                .await
        })
    }
}
