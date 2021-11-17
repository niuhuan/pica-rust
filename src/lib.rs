mod entities;
mod hmac;
mod test;
mod types;

use std::net::{SocketAddr};
pub use crate::entities::*;
pub use crate::types::*;

use chrono::prelude::Local;
use serde_json::json;

const HOST_URL: &str = "https://picaapi.picacomic.com/";
const API_KEY: &str = "C69BAF41DA5ABD1FFEDC6D2FEA56B";
const NONCE: &str = "b1ab87b4800d4d4590a11701b8551afa";
const DIGEST_KEY: &str = "~d}$Q7$eIni=V)9\\RK/P.RM4;9[7|@/CA}b~OW!3?EV`:<>M7pddUBL5n|0/*Cn";

const SWITCH_ADDRESS_HOSTS: [&str; 1] = [
    "picaapi.picacomic.com",
];

/// 客户端
pub struct Client {
    agent: reqwest::blocking::Client,
    pub token: String,
    pub switch_ip: Option<String>,
}

/// 接口实现
impl Client {
    /// 构造方法
    pub fn new() -> Self {
        Self {
            agent: Client::request_agent(None, None).unwrap(),
            token: "".to_string(),
            switch_ip: Option::None,
        }
    }

    /// 代理和分流
    fn request_agent(url: Option<&str>, switch_address: Option<SwitchAddress>) -> Result<reqwest::blocking::Client> {
        let mut builder = reqwest::blocking::ClientBuilder::new();
        builder = match url {
            None => { builder }
            Some(url) => {
                builder.proxy(reqwest::Proxy::all(url)?)
            }
        };
        builder = match switch_address {
            None => { builder }
            Some(address) => {
                let address: SocketAddr = address.as_str().parse()?;
                let mut tmp = builder;
                for x in SWITCH_ADDRESS_HOSTS {
                    tmp = tmp.resolve(x, address);
                }
                tmp
            }
        };
        Ok(builder.build()?)
    }

    /// 设置代理和分流
    pub fn set_proxy(&mut self, url: Option<&str>, switch_address: Option<SwitchAddress>) -> Result<()> {
        self.agent = Client::request_agent(url, switch_address)?;
        Ok(())
    }

    /// 请求和签名
    fn pica_request<T: for<'de> serde::Deserialize<'de>>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<T> {
        let time = Local::now().timestamp().to_string();
        let request = self
            .agent
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
            .header("authorization", self.token.as_str())
            .header("image-quality", "original")
            .header(
                "signature",
                hmac::hmac_hex(
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
        match resp {
            Ok(resp) => {
                let status = resp.status();
                let json: serde_json::Value = serde_json::from_str(resp.text()?.as_str())?;
                // println!("{}", &json); // when debug
                match status.as_u16() {
                    200 => {
                        match path {
                            "auth/register" => {
                                Ok(serde_json::from_str("null")?)
                            }
                            _ => {
                                let v = json
                                    .get("data")
                                    .ok_or(Error::from("response data error"))?
                                    .clone();
                                let r = serde_json::from_value(v)?;
                                Ok(r)
                            }
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

            Err(err) => {
                Err(Box::new(Error::from(err.to_string())))
            }
        }
    }

    /// Get
    fn pica_get<T: for<'de> serde::Deserialize<'de>>(
        &self,
        path: &str,
    ) -> Result<T> {
        return self.pica_request(reqwest::Method::GET, path, None);
    }

    /// Post
    fn pica_post<T: for<'de> serde::Deserialize<'de>>(
        &self,
        path: &str,
        body: serde_json::Value,
    ) -> Result<T> {
        return self.pica_request(reqwest::Method::POST, path, Some(body));
    }

    /// 注册 (email为用户名, 不一定是邮箱)
    pub fn register(&self, register_dto: RegisterDto) -> Result<()> {
        self.pica_post("auth/register", serde_json::json!(register_dto))
    }

    /// 用户登陆 (email为用户名, 不一定是邮箱)
    pub fn login(&mut self, email: &str, password: &str) -> Result<()> {
        let data: LoginResponseData = self.pica_post(
            "auth/sign-in",
            serde_json::json!({
            "email": email,
            "password": password,
            }),
        )?;
        self.token = data.token;
        Ok(())
    }

    /// 用户信息
    pub fn user_profile(&self) -> Result<UserProfile> {
        let data: UserProfileResponseData = self.pica_get("users/profile")?;
        Ok(data.user)
    }

    /// 打卡
    pub fn punch_in(&self) -> Result<PunchStatus> {
        let data: PunchResponseData = self.pica_post("users/punch-in", json!({}))?;
        Ok(data.res)
    }

    /// 漫画分页
    pub fn comics(
        &self,
        category: Option<&str>,
        tag: Option<&str>,
        creator_id: Option<&str>,
        chinese_team: Option<&str>,
        sort: Sort,
        page: i32,
    ) -> Result<PageData<ComicSimple>> {
        let mut url = vec![];
        url.push("comics?");

        let value_category;
        match category {
            None => {}
            Some(category) => {
                value_category = urlencoding::encode(category);
                url.push("c=");
                url.push(value_category.as_ref());
                url.push("&");
            }
        }

        let value_tag;
        match tag {
            None => {}
            Some(tag) => {
                value_tag = urlencoding::encode(tag);
                url.push("t=");
                url.push(value_tag.as_ref());
                url.push("&");
            }
        }
        match creator_id {
            None => {}
            Some(creator_id) => {
                url.push("&ca=");
                url.push(creator_id);
                url.push("&");
            }
        }

        let value_chinese_team;
        match chinese_team {
            None => {}
            Some(chinese_team) => {
                value_chinese_team = urlencoding::encode(chinese_team);
                url.push("&t=");
                url.push(value_chinese_team.as_ref());
                url.push("&");
            }
        }

        url.push("s=");
        url.push(sort.as_str());
        url.push("&");

        let page_str = page.to_string();
        url.push("page=");
        url.push(page_str.as_str());

        let url: String = url.join("");
        let data: ComicPageResponseData = self.pica_get(url.as_str())?;
        Ok(data.comics)
    }

    /// 随机漫画
    pub fn comics_random(&self) -> Result<Vec<ComicSimple>> {
        let data: ComicListResponseData = self.pica_get("comics/random")?;
        Ok(data.comics)
    }

    /// 漫画信息
    pub fn comic_info(&self, comic_id: String) -> Result<ComicInfo> {
        let data: ComicInfoResponseData =
            self.pica_request(
                reqwest::Method::GET,
                format!("comics/{}", comic_id).as_str(),
                None,
            )?;
        Ok(data.comic)
    }

    /// 获取漫画EP(分页)
    pub fn comic_eps(&self, comic_id: String, page: i32) -> Result<PageData<ComicEp>> {
        let data: ComicEpsResponseData = self.pica_get(
            format!("comics/{}/eps?page={}", comic_id, page).as_str()
        )?;
        Ok(data.eps)
    }
}
