use base64::{engine::general_purpose, Engine as _};
pub use crate::entities::*;
pub use crate::types::*;
use chrono::prelude::Local;
use hyper::client::connect::dns::GaiResolver;
use hyper::client::connect::dns::Name;
use hyper::service::Service;
use reqwest::dns::{Addrs, Resolve, Resolving};
use reqwest::{Proxy, Url};
use serde_json::json;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

const HOST_URL: &str = "https://picaapi.picacomic.com/";
const API_KEY: &str = "C69BAF41DA5ABD1FFEDC6D2FEA56B";
const NONCE: &str = "b1ab87b4800d4d4590a11701b8551afa";
const DIGEST_KEY: &str = "~d}$Q7$eIni=V)9\\RK/P.RM4;9[7|@/CA}b~OW!3?EV`:<>M7pddUBL5n|0/*Cn";
const DEFAULT_SWITCH_ADDRESSES: &[&str] = &[
    "172.67.7.24:443",
    "172.67.194.19:443",
    "172.67.80.1:443",
    "104.21.235.3:443",
    "104.21.235.4:443",
    "[2606:4700:3038::6815:eb03]:443",
    "104.20.180.50:443",
    "104.20.181.50:443",
    "104.22.64.159:443",
    "104.21.91.145:443",
];
const DEFAULT_IMAGE_SWITCH_HOSTS: &[&str] = &[
    "https://storage1.picacomic.com",
    "https://s2.picacomic.com",
    "https://s3.picacomic.com",
    "https://storage.tipatipa.xyz",
    "https://storage-b.picacomic.com",
    "https://storage.wikawika.xyz",
];

/// 客户端
pub struct Client {
    agent: RwLock<Arc<reqwest::Client>>,
    token: RwLock<String>,
    host: Arc<RwLock<String>>,
    dns_overrides: Arc<RwLock<Option<Vec<SocketAddr>>>>,
    image_switch_host: Arc<RwLock<Option<String>>>,
    image_use_dns_overrides: Arc<RwLock<bool>>,
    proxy: Arc<RwLock<Option<Proxy>>>,
}

/// 接口实现
impl Client {
    /// 构造方法
    pub async fn new() -> Self {
        Self::with_host(HOST_URL.to_string()).await
    }

    /// 使用自定义host (便于测试或代理)
    pub async fn with_host(host: impl Into<String>) -> Self {
        let dns_overrides_value = Some(Self::default_switch_addresses());
        let image_switch_host_value = Some(Self::default_image_switch_host());
        let image_use_dns_overrides_value = true;
        let proxy_value = None;
        let host_value = host.into();
        let agent = Self::build_agent(
            dns_overrides_value.clone(),
            host_value.clone(),
            image_switch_host_value.clone(),
            image_use_dns_overrides_value,
            proxy_value.clone(),
        );
        let dns_overrides = Arc::new(RwLock::new(dns_overrides_value));
        let image_switch_host = Arc::new(RwLock::new(image_switch_host_value));
        let host_lock = Arc::new(RwLock::new(host_value));
        let image_use_dns_overrides = Arc::new(RwLock::new(image_use_dns_overrides_value));
        let proxy = Arc::new(RwLock::new(proxy_value));
        Self {
            agent: RwLock::new(agent),
            token: RwLock::new(String::default()),
            host: host_lock,
            dns_overrides,
            image_switch_host,
            image_use_dns_overrides,
            proxy,
        }
    }

    fn default_switch_addresses() -> Vec<SocketAddr> {
        DEFAULT_SWITCH_ADDRESSES
            .iter()
            .filter_map(|addr| addr.parse::<SocketAddr>().ok())
            .collect()
    }

    fn default_image_switch_host() -> String {
        DEFAULT_IMAGE_SWITCH_HOSTS
            .first()
            .unwrap_or(&"https://storage1.picacomic.com")
            .to_string()
    }

    fn build_agent(
        dns_overrides: Option<Vec<SocketAddr>>,
        host: String,
        image_switch_host: Option<String>,
        image_use_dns_overrides: bool,
        proxy: Option<Proxy>,
    ) -> Arc<reqwest::Client> {
        let resolver =
            PicaDnsResolver::new(dns_overrides, host, image_switch_host, image_use_dns_overrides);
        let mut builder = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(60))
            .dns_resolver(Arc::new(resolver));
        if let Some(proxy) = proxy {
            builder = builder.proxy(proxy);
        }
        Arc::new(builder.build().expect("failed to build reqwest client"))
    }

    async fn agent(&self) -> Arc<reqwest::Client> {
        self.agent.read().await.clone()
    }

    async fn host(&self) -> String {
        self.host.read().await.clone()
    }

    pub async fn set_host(&self, host: impl Into<String>) {
        {
            let mut host_lock = self.host.write().await;
            *host_lock = host.into();
        }
        self.rebuild_agent().await;
    }

    /// 设置分流地址（匹配 pica/wika/tipa 域名的 DNS Override）
    pub async fn set_switch_addresses(&self, addresses: Vec<String>) -> Result<()> {
        let parsed: Vec<SocketAddr> = addresses
            .into_iter()
            .filter_map(|addr| addr.parse::<SocketAddr>().ok())
            .collect();
        if parsed.is_empty() {
            return Err(Error::InvalidAddress("no valid addresses provided".into()));
        }
        {
            let mut guard = self.dns_overrides.write().await;
            *guard = Some(parsed);
        }
        self.rebuild_agent().await;
        Ok(())
    }

    /// 禁用自定义分流地址
    pub async fn clear_switch_addresses(&self) {
        {
            let mut guard = self.dns_overrides.write().await;
            *guard = None;
        }
        self.rebuild_agent().await;
    }

    async fn rebuild_agent(&self) {
        let agent = Self::build_agent(
            self.dns_overrides.read().await.clone(),
            self.host.read().await.clone(),
            self.image_switch_host.read().await.clone(),
            *self.image_use_dns_overrides.read().await,
            self.proxy.read().await.clone(),
        );
        let mut agent_lock = self.agent.write().await;
        *agent_lock = agent;
    }

    /// 设置图片分流地址（可选，不设置使用默认常量）
    pub async fn set_image_switch_host(&self, host: Option<String>) -> Result<()> {
        if let Some(host) = host {
            if host.trim().is_empty() {
                return Err(Error::InvalidAddress("no image host provided".into()));
            }
            let mut guard = self.image_switch_host.write().await;
            *guard = Some(host);
        } else {
            let mut guard = self.image_switch_host.write().await;
            *guard = None;
        }
        self.rebuild_agent().await;
        Ok(())
    }

    /// 图片是否使用 DNS 分流（默认开启）
    pub async fn set_image_use_dns_overrides(&self, use_overrides: bool) {
        {
            let mut guard = self.image_use_dns_overrides.write().await;
            *guard = use_overrides;
        }
        self.rebuild_agent().await;
    }

    /// 设置代理（支持 HTTP/SOCKS5），传 None 清除
    pub async fn set_proxy(&self, proxy: Option<String>) -> Result<()> {
        let parsed = if let Some(p) = proxy {
            if p.trim().is_empty() {
                None
            } else {
                Some(Proxy::all(p).map_err(|e| Error::InvalidAddress(e.to_string()))?)
            }
        } else {
            None
        };
        {
            let mut guard = self.proxy.write().await;
            *guard = parsed;
        }
        self.rebuild_agent().await;
        Ok(())
    }

    pub async fn token(&self) -> String {
        self.token.read().await.clone()
    }

    pub async fn set_token(&self, token: impl Into<String>) {
        let token_string = token.into();
        let mut token_lock = self.token.write().await;
        *token_lock = token_string;
    }

    /// 请求和签名
    async fn pica_request_impl<T: for<'de> serde::Deserialize<'de>>(
        &self,
        method: reqwest::Method,
        host: Option<&str>,
        path: &str,
        body: Option<serde_json::Value>,
        image_quality: &str,
        expect_data_wrapper: bool,
    ) -> Result<T> {
        let host_owned;
        let host = match host {
            Some(host) => host,
            None => {
                host_owned = self.host().await;
                host_owned.as_str()
            }
        };
        let time = Local::now().timestamp().to_string();
        let request = self
            .agent()
            .await
            .request(method.clone(), format!("{}{}", host, path).as_str())
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
            .header("image-quality", image_quality)
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
            Some(body) => {
                let body_string = serde_json::to_string(&body)
                    .map_err(|err| Error::Deserialize(err.to_string()))?;
                request.body(body_string).send()
            }
        }
            .await
            .map_err(|err| Error::Network(err.to_string()))?;
        let status = resp.status();
        let text = resp
            .text()
            .await
            .map_err(|err| Error::Network(err.to_string()))?;
        let json: serde_json::Value =
            serde_json::from_str(text.as_str()).map_err(|err| Error::Deserialize(err.to_string()))?;

        if status.is_success() {
            if expect_data_wrapper {
                let data = json.get("data");
                if let Some(data) = data {
                    Ok(serde_json::from_value(data.clone())
                        .map_err(|err| Error::Deserialize(err.to_string()))?)
                } else {
                    Ok(serde_json::from_value(serde_json::Value::Null)
                        .map_err(|err| Error::Deserialize(err.to_string()))?)
                }
            } else {
                Ok(serde_json::from_value(json)
                    .map_err(|err| Error::Deserialize(err.to_string()))?)
            }
        } else {
            let message = json
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("request error")
                .to_string();
            Err(Error::Http {
                status: status.as_u16(),
                message,
            })
        }
    }

    async fn pica_request<T: for<'de> serde::Deserialize<'de>>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<T> {
        self.pica_request_with_quality(method, path, body, "original")
            .await
    }

    async fn pica_request_with_quality<T: for<'de> serde::Deserialize<'de>>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<serde_json::Value>,
        image_quality: &str,
    ) -> Result<T> {
        self.pica_request_impl(
            method,
            None,
            path,
            body,
            image_quality,
            true,
        )
            .await
    }

    async fn pica_request_with_host<T: for<'de> serde::Deserialize<'de>>(
        &self,
        method: reqwest::Method,
        host: &str,
        path: &str,
        body: Option<serde_json::Value>,
        image_quality: &str,
        expect_data_wrapper: bool,
    ) -> Result<T> {
        self.pica_request_impl(method, Some(host), path, body, image_quality, expect_data_wrapper)
            .await
    }

    /// Get
    async fn pica_get<T: for<'de> serde::Deserialize<'de>>(&self, path: &str) -> Result<T> {
        self.pica_request(reqwest::Method::GET, path, None).await
    }

    async fn pica_get_with_quality<T: for<'de> serde::Deserialize<'de>>(
        &self,
        path: &str,
        image_quality: &str,
    ) -> Result<T> {
        self.pica_request_with_quality(reqwest::Method::GET, path, None, image_quality)
            .await
    }

    /// Post
    async fn pica_post<T: for<'de> serde::Deserialize<'de>>(
        &self,
        path: &str,
        body: serde_json::Value,
    ) -> Result<T> {
        self.pica_request(reqwest::Method::POST, path, Some(body)).await
    }

    /// Put
    async fn pica_put<T: for<'de> serde::Deserialize<'de>>(
        &self,
        path: &str,
        body: serde_json::Value,
    ) -> Result<T> {
        self.pica_request(reqwest::Method::PUT, path, Some(body)).await
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

    /// 分类
    pub async fn categories(&self) -> Result<Vec<Category>> {
        let data: CategoriesResponseData = self.pica_get("categories").await?;
        Ok(data.categories)
    }

    /// 漫画分页
    pub async fn comics(
        &self,
        category: Option<String>,
        tag: Option<String>,
        author: Option<String>,
        creator_id: Option<String>,
        chinese_team: Option<String>,
        sort: Sort,
        page: i32,
    ) -> Result<PageData<ComicSimple>> {
        let mut params: Vec<String> = vec![];
        if let Some(category) = category {
            params.push(format!(
                "c={}",
                urlencoding::encode(category.as_str()).as_ref()
            ));
        }
        if let Some(tag) = tag {
            params.push(format!(
                "t={}",
                urlencoding::encode(tag.as_str()).as_ref()
            ));
        }
        if let Some(author) = author {
            params.push(format!(
                "a={}",
                urlencoding::encode(author.as_str()).as_ref()
            ));
        }
        if let Some(creator_id) = creator_id {
            params.push(format!("ca={}", creator_id.as_str()));
        }
        if let Some(ct) = chinese_team {
            params.push(format!(
                "ct={}",
                urlencoding::encode(ct.as_str()).as_ref()
            ));
        }
        params.push(format!("s={}", sort.as_str()));
        params.push(format!("page={}", page));
        let url: String = format!("comics?{}", params.join("&"));
        let data: ComicPageResponseData = self.pica_get(url.as_str()).await?;
        Ok(data.comics)
    }

    /// 随机漫画
    pub async fn comics_random(&self) -> Result<Vec<ComicSimple>> {
        let data: ComicListResponseData = self.pica_get("comics/random").await?;
        Ok(data.comics)
    }

    /// 漫画排行榜
    pub async fn leaderboard(&self, leaderboard_type: &str) -> Result<Vec<ComicSimple>> {
        let url = format!("comics/leaderboard?tt={}&ct=VC", leaderboard_type);
        let data: ComicListResponseData = self.pica_get(url.as_str()).await?;
        Ok(data.comics)
    }

    /// 漫画信息
    pub async fn comic_info(&self, comic_id: String) -> Result<ComicInfo> {
        let url = format!("comics/{}", comic_id);
        let data: ComicInfoResponseData = self.pica_get(url.as_str()).await?;
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

    /// 获取EP图片(分页) 指定质量
    pub async fn comic_ep_pictures_with_quality(
        &self,
        comic_id: String,
        ep_order: i32,
        page: i32,
        image_quality: &str,
    ) -> Result<PageData<ComicEpPicture>> {
        let url = format!(
            "comics/{}/order/{}/pages?page={}",
            comic_id, ep_order, page
        );
        let data: ComicEpPicturePageResponseData = self
            .pica_get_with_quality(url.as_str(), image_quality)
            .await?;
        Ok(data.pages)
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

    /// 看了这个本子的也在看
    pub async fn comic_recommendation(&self, comic_id: String) -> Result<Vec<ComicSimple>> {
        let url = format!("comics/{}/recommendation", comic_id);
        let data: ComicListResponseData = self.pica_get(url.as_str()).await?;
        Ok(data.comics)
    }

    /// 获取漫画的评论
    pub async fn comic_comments(&self, comic_id: String, page: i32) -> Result<CommentsResponse> {
        let url: String = format!("comics/{}/comments?page={}", comic_id, page);
        Ok(self.pica_get(url.as_str()).await?)
    }

    /// 我的评论
    pub async fn my_comments(&self, page: i32) -> Result<PageData<MyComment>> {
        let url = format!("users/my-comments?page={}", page);
        let data: MyCommentsResponseData = self.pica_get(url.as_str()).await?;
        Ok(data.comments)
    }

    /// 获取子评论
    pub async fn comment_children(&self, comment_id: String, page: i32) -> Result<PageData<Comment>> {
        let url = format!("comments/{}/childrens?page={}", comment_id, page);
        let data: CommentChildrenResponseData = self.pica_get(url.as_str()).await?;
        Ok(data.comments)
    }

    /// 喜欢/取消喜欢评论
    pub async fn switch_like_comment(&self, comment_id: String) -> Result<Action> {
        let url = format!("comments/{}/like", comment_id);
        self.pica_post(url.as_str(), json!({})).await
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

    /// 隐藏评论(管理员接口)
    pub async fn hide_comment(&self, comment_id: String) -> Result<()> {
        let url = format!("comments/{}/delete", comment_id);
        self.pica_post(url.as_str(), json!({})).await
    }

    /// 搜索
    pub async fn advanced_search(
        &self,
        content: String,
        sort: Sort,
        page: i32,
        categories: Option<Vec<String>>,
    ) -> Result<ComicSearchResponseData> {
        let url = format!("comics/advanced-search?page={}", page);
        let mut payload = serde_json::Map::new();
        payload.insert("keyword".to_string(), json!(content));
        payload.insert("sort".to_string(), json!(sort.as_str()));
        if let Some(categories) = categories {
            if !categories.is_empty() {
                payload.insert("categories".to_string(), json!(categories));
            }
        }
        Ok(self
            .pica_post(url.as_str(), serde_json::Value::Object(payload))
            .await?)
    }

    /// 大家都在搜
    pub async fn hot_keywords(&self) -> Result<Vec<String>> {
        let data: HotKeywordsResponseData = self.pica_get("keywords").await?;
        Ok(data.keywords)
    }

    /// 骑士榜
    pub async fn leaderboard_of_knight(&self) -> Result<Vec<Knight>> {
        let data: LeaderboardOfKnightResponseData =
            self.pica_get("comics/knight-leaderboard").await?;
        Ok(data.users)
    }

    /// 游戏列表
    pub async fn game_page(&self, page: i32) -> Result<PageData<GameSimple>> {
        let url = format!("games?page={}", page);
        let data: GamePageResponseData = self.pica_get(url.as_str()).await?;
        Ok(data.games)
    }

    /// 游戏详情
    pub async fn game_info(&self, game_id: String) -> Result<GameInfo> {
        let url = format!("games/{}", game_id);
        let data: GameInfoResponseData = self.pica_get(url.as_str()).await?;
        Ok(data.game)
    }

    /// 游戏评论
    pub async fn game_comments(&self, game_id: String, page: i32) -> Result<GameCommentsResponseData> {
        let url = format!("games/{}/comments?page={}", game_id, page);
        self.pica_get(url.as_str()).await
    }

    /// 发表游戏评论
    pub async fn post_game_comment(&self, game_id: String, content: String) -> Result<()> {
        let url = format!("games/{}/comments", game_id);
        self.pica_post(url.as_str(), json!({ "content": content }))
            .await
    }

    /// 游戏子评论
    pub async fn game_comment_children(
        &self,
        comment_id: String,
        page: i32,
    ) -> Result<PageData<Comment>> {
        let url = format!("comments/{}/childrens?page={}", comment_id, page);
        let data: GameCommentChildrenResponseData = self.pica_get(url.as_str()).await?;
        Ok(data.comments)
    }

    /// 修改密码
    pub async fn update_password(&self, old_password: String, new_password: String) -> Result<()> {
        self.pica_put(
            "users/password",
            json!({
                "old_password": old_password,
                "new_password": new_password,
            }),
        )
            .await
    }

    /// 修改签名
    pub async fn update_slogan(&self, slogan: String) -> Result<()> {
        self.pica_put("users/profile", json!({ "slogan": slogan }))
            .await
    }

    /// 修改头像
    pub async fn update_avatar(&self, jpeg_bytes: Vec<u8>) -> Result<()> {
        let encoded = general_purpose::STANDARD.encode(jpeg_bytes);
        self.pica_put(
            "users/avatar",
            json!({ "avatar": format!("data:image/jpeg;base64,{}", encoded) }),
        )
            .await
    }

    /// 下载图片（使用与 API 相同的分流）
    pub async fn download_image(&self, file_server: &str, path: &str) -> Result<Vec<u8>> {
        let target_server = self
            .image_switch_host
            .read()
            .await
            .as_ref()
            .map(|h| h.trim_end_matches('/').to_string())
            .unwrap_or_else(|| file_server.trim_end_matches('/').to_string());
        let url = format!("{}/static/{}", target_server, path.trim_start_matches('/'));
        let resp = self
            .agent()
            .await
            .get(url)
            .send()
            .await
            .map_err(|err| Error::Network(err.to_string()))?;
        let status = resp.status();
        if !status.is_success() {
            return Err(Error::Http {
                status: status.as_u16(),
                message: "failed to fetch image".to_string(),
            });
        }
        resp.bytes()
            .await
            .map(|b| b.to_vec())
            .map_err(|err| Error::Network(err.to_string()))
    }

    /// 收藏夹合集
    pub async fn collections(&self) -> Result<Vec<Collection>> {
        let data: CollectionsResponseData = self.pica_get("collections").await?;
        Ok(data.collections)
    }

    /// 找回密码-获取问题
    pub async fn forgot_password(&self, email: String) -> Result<ForgotPasswordResult> {
        self.pica_post("auth/forgot-password", json!({ "email": email }))
            .await
    }

    /// 找回密码-重置密码
    pub async fn reset_password(
        &self,
        email: String,
        question_no: i32,
        answer: String,
    ) -> Result<ResetPasswordResult> {
        self.pica_post(
            "auth/reset-password",
            json!({
                "email": email,
                "questionNo": question_no,
                "answer": answer,
            }),
        )
            .await
    }

    /// 初始化信息
    pub async fn init_info(&self) -> Result<InitInfo> {
        self.pica_request_with_host(
            reqwest::Method::GET,
            "http://68.183.234.72/",
            "init",
            None,
            "original",
            false,
        )
            .await
    }
}

#[derive(Clone)]
struct PicaDnsResolver {
    addrs: Option<Vec<SocketAddr>>,
    host: Option<String>,
    image_switch_host: Option<String>,
    image_use_dns_overrides: bool,
    hyper_gai_resolver: GaiResolver,
}

impl PicaDnsResolver {
    fn new(
        addrs: Option<Vec<SocketAddr>>,
        host: String,
        image_switch_host: Option<String>,
        image_use_dns_overrides: bool,
    ) -> Self {
        Self {
            addrs,
            host: Url::parse(host.as_str())
                .ok()
                .and_then(|u| u.host_str().map(|h| h.to_lowercase())),
            image_switch_host,
            image_use_dns_overrides,
            hyper_gai_resolver: GaiResolver::new(),
        }
    }
}

impl Resolve for PicaDnsResolver {
    fn resolve(&self, domain: Name) -> Resolving {
        let addrs = self.addrs.clone();
        let mut hyper = self.hyper_gai_resolver.clone();
        let image_use_dns_overrides = self.image_use_dns_overrides;
        let image_switch_host = self.image_switch_host.clone();
        let host = self.host.clone();
        Box::pin(async move {
            if image_use_dns_overrides {
                if let Some(addresses) = addrs {
                    let domain_str = domain.as_str().to_lowercase();
                    let matched = if image_switch_host.is_some() {
                        true
                    } else if let Some(h) = host.as_ref() {
                        domain_str == *h
                    } else {
                        false
                    };
                    if matched {
                        return Ok(Addrs::from(Box::new(addresses.into_iter())));
                    }
                }
            }
            match Service::<Name>::call(&mut hyper, domain).await {
                Ok(addrs) => Ok(Box::new(addrs)),
                Err(err) => Err(Box::new(err) as Box<dyn std::error::Error + Send + Sync>),
            }
        })
    }
}
