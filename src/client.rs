pub use crate::entities::*;
pub use crate::types::*;
use chrono::prelude::Local;
use futures_util::FutureExt;
use hyper::client::connect::dns::GaiResolver;
use hyper::client::connect::dns::Name;
use hyper::service::Service;
use regex::Regex;
use reqwest::dns::{Addrs, Resolve, Resolving};
use serde::de::StdError;
use serde_json::json;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

const HOST_URL: &str = "https://picaapi.picacomic.com/";
const API_KEY: &str = "C69BAF41DA5ABD1FFEDC6D2FEA56B";
const NONCE: &str = "b1ab87b4800d4d4590a11701b8551afa";
const DIGEST_KEY: &str = "~d}$Q7$eIni=V)9\\RK/P.RM4;9[7|@/CA}b~OW!3?EV`:<>M7pddUBL5n|0/*Cn";

/// 客户端
pub struct Client {
    agent: RwLock<Arc<reqwest::Client>>,
    token: RwLock<String>,
}

/// 接口实现
impl Client {
    /// 构造方法
    pub fn new() -> Self {
        Self {
            agent: RwLock::new(Arc::new(reqwest::ClientBuilder::new().build().unwrap())),
            token: RwLock::new(String::default()),
        }
    }

    async fn agent(&self) -> Arc<reqwest::Client> {
        let agent = self.agent.read().await;
        return agent.clone();
    }

    pub async fn token(&self) -> String {
        let token = self.token.read().await;
        return token.clone();
    }

    pub async fn set_token(&self, token: impl Into<String>) {
        let token_string = token.into();
        let mut token_lock = self.token.write().await;
        *token_lock = token_string;
    }

    /// 请求和签名
    async fn pica_request<T: for<'de> serde::Deserialize<'de>>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<T> {
        let time = Local::now().timestamp().to_string();
        let request = self
            .agent()
            .await
            .request(method.clone(), format!("{}{}", HOST_URL, path).as_str())
            .header("api-key", API_KEY)
            .header("accept", "application/vnd.picacomic.com.v1+json")
            .header("app-channel", "2")
            .header("time", time.as_str())
            .header("nonce", NONCE)
            .header("app-version", "2.2.1.2.3.3")
            .header("app-uuid", "defaultUuid")
            .header("app-platform", "android")
            .header("app-build-version", "44")
            .header("Content-Type", "application/json; charset=UTF-8")
            .header("User-Agent", "okhttp/3.8.1")
            .header("authorization", self.token().await)
            .header("image-quality", "original")
            .header(
                "signature",
                crate::hmac::hmac_hex(
                    DIGEST_KEY,
                    ("".to_string() + path + time.as_str() + NONCE + method.as_str() + API_KEY)
                        .to_lowercase()
                        .as_str(),
                )
                .as_str(),
            );
        let resp = match body {
            None => request.send(),
            Some(body) => request.body(serde_json::to_string(&body)?).send(),
        };
        let resp = resp.await;
        match resp {
            Ok(resp) => {
                let status = resp.status();
                let json: serde_json::Value = serde_json::from_str(resp.text().await?.as_str())?;
                // println!("{}", &json); // when debug
                match status.as_u16() {
                    200 => {
                        let data = json.get("data");
                        if data.is_some() {
                            Ok(serde_json::from_value(data.ok_or("error")?.clone())?)
                        } else {
                            Ok(serde_json::from_str("null")?)
                        }
                    }
                    _ => {
                        let message = json
                            .get("message")
                            .ok_or("message error")?
                            .as_str()
                            .ok_or("message error")?;
                        Err(Box::new(Error::from(message)))
                    }
                }
            }
            Err(err) => Err(Box::new(Error::from(err.to_string()))),
        }
    }

    /// Get
    async fn pica_get<T: for<'de> serde::Deserialize<'de>>(&self, path: &str) -> Result<T> {
        return self.pica_request(reqwest::Method::GET, path, None).await;
    }

    /// Post
    async fn pica_post<T: for<'de> serde::Deserialize<'de>>(
        &self,
        path: &str,
        body: serde_json::Value,
    ) -> Result<T> {
        return self
            .pica_request(reqwest::Method::POST, path, Some(body))
            .await;
    }

    /// 注册 (email为用户名, 不一定是邮箱)
    pub async fn register(&self, register_dto: RegisterDto) -> Result<()> {
        self.pica_post("auth/register", serde_json::json!(register_dto))
            .await
    }

    /// 用户登陆 (email为用户名, 不一定是邮箱)
    pub async fn login(&self, email: &str, password: &str) -> Result<()> {
        let data: LoginResponseData = self
            .pica_post(
                "auth/sign-in",
                serde_json::json!({
                "email": email,
                "password": password,
                }),
            )
            .await?;
        self.set_token(data.token).await;
        Ok(())
    }

    /// 用户信息
    pub async fn user_profile(&self) -> Result<UserProfile> {
        let data: UserProfileResponseData = self.pica_get("users/profile").await?;
        Ok(data.user)
    }

    /// 打卡
    pub async fn punch_in(&self) -> Result<PunchStatus> {
        let data: PunchResponseData = self.pica_post("users/punch-in", json!({})).await?;
        Ok(data.res)
    }

    /// 漫画分页
    pub async fn comics(
        &self,
        category: Option<String>,
        tag: Option<String>,
        creator_id: Option<String>,
        chinese_team: Option<String>,
        sort: Sort,
        page: i32,
    ) -> Result<PageData<ComicSimple>> {
        let mut url: Vec<String> = vec![];
        url.push("comics?".to_string());
        url.push(match category {
            None => "".to_string(),
            Some(category) => {
                format!("c={}&", urlencoding::encode(category.as_str()).as_ref())
            }
        });
        url.push(match tag {
            None => "".to_string(),
            Some(tag) => {
                format!("t={}&", urlencoding::encode(tag.as_str()).as_ref())
            }
        });
        url.push(match creator_id {
            None => "".to_string(),
            Some(creator_id) => {
                format!("&ca={}&", creator_id.as_str())
            }
        });
        url.push(match chinese_team {
            None => "".to_string(),
            Some(ct) => {
                format!("ct={}&", urlencoding::encode(ct.as_str()).as_ref())
            }
        });
        url.push(format!("s={}&page={}", sort.as_str(), page.to_string()));
        let url: String = url.join("");
        let data: ComicPageResponseData = self.pica_get(url.as_str()).await?;
        Ok(data.comics)
    }

    /// 随机漫画
    pub async fn comics_random(&self) -> Result<Vec<ComicSimple>> {
        let data: ComicListResponseData = self.pica_get("comics/random").await?;
        Ok(data.comics)
    }

    /// 漫画信息
    pub async fn comic_info(&self, comic_id: String) -> Result<ComicInfo> {
        let data: ComicInfoResponseData = self
            .pica_request(
                reqwest::Method::GET,
                format!("comics/{}", comic_id).as_str(),
                None,
            )
            .await?;
        Ok(data.comic)
    }

    /// 获取漫画EP(分页)
    pub async fn comic_eps(&self, comic_id: String, page: i32) -> Result<PageData<ComicEp>> {
        let data: ComicEpsResponseData = self
            .pica_get(format!("comics/{}/eps?page={}", comic_id, page).as_str())
            .await?;
        Ok(data.eps)
    }

    /// 获取EP图片(分页)
    pub async fn comic_ep_pictures(
        &self,
        comic_id: String,
        ep_order: i32,
        page: i32,
    ) -> Result<PageData<ComicEpPicture>> {
        let data: ComicEpPicturePageResponseData = self
            .pica_get(
                format!("comics/{}/order/{}/pages?page={}", comic_id, ep_order, page).as_str(),
            )
            .await?;
        return Ok(data.pages);
    }

    /// 收藏的漫画(分页)
    pub async fn favourite_comics(&self, sort: Sort, page: i32) -> Result<PageData<ComicSimple>> {
        let url: String = format!("users/favourite?s={}&page={}", sort.as_str(), page);
        let data: ComicPageResponseData = self.pica_get(url.as_str()).await?;
        Ok(data.comics)
    }

    /// 喜欢/取消喜欢漫画
    pub async fn switch_like(&self, comic_id: String) -> Result<Action> {
        let url: String = format!("comics/{}/like", comic_id);
        Ok(self.pica_post(url.as_str(), json!({})).await?)
    }

    /// 收藏/取消收藏漫画
    pub async fn switch_favourite(&self, comic_id: String) -> Result<Action> {
        let url: String = format!("comics/{}/favourite", comic_id);
        Ok(self.pica_post(url.as_str(), json!({})).await?)
    }

    /// 获取漫画的评论
    pub async fn comic_comments(&self, comic_id: String, page: i32) -> Result<CommentsResponse> {
        let url: String = format!("comics/{}/comments?page={}", comic_id, page);
        Ok(self.pica_get(url.as_str()).await?)
    }

    /// 发表评论
    pub async fn post_comment(&self, comic_id: String, content: String) -> Result<()> {
        let url: String = format!("comics/{}/comments", comic_id);
        self.pica_post(url.as_str(), json!({ "content": content }))
            .await
    }

    /// 发表回复
    pub async fn post_child_comment(&self, comment_id: String, content: String) -> Result<()> {
        let url: String = format!("comments/{}", comment_id);
        self.pica_post(url.as_str(), json!({ "content": content }))
            .await
    }

    /// 搜索
    pub async fn advanced_search(
        &self,
        content: String,
        sort: Sort,
        page: i32,
        categories: Vec<String>,
    ) -> Result<ComicSearchResponseData> {
        let url = format!("comics/advanced-search?page={}", page);
        Ok(self
            .pica_post(
                url.as_str(),
                json!({
                    "keyword": content,
                    "sort": sort.as_str(),
                    "categories": categories,
                }),
            )
            .await?)
    }
}

#[derive(Clone)]
struct RegexpDnsOverridesResolver {
    regexp: Regex,
    addr: SocketAddr,
    hyper_gai_resolver: GaiResolver,
}

impl Resolve for RegexpDnsOverridesResolver {
    fn resolve(&self, domain: Name) -> Resolving {
        let mut this = self.clone();
        if this.regexp.is_match(domain.as_str()) {
            Box::pin(async move { Ok(Addrs::from(Box::new(vec![this.addr].into_iter()))) })
        } else {
            Box::pin(
                Service::<Name>::call(&mut this.hyper_gai_resolver, domain).map(|result| {
                    result
                        .map(|addrs| -> Addrs { Box::new(addrs) })
                        .map_err(|err| -> Box<dyn StdError + Send + Sync> { Box::new(err) })
                }),
            )
        }
    }
}
