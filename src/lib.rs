mod entities;
mod error;
mod hmac;
mod test;

pub use crate::entities::*;
pub use crate::error::Error;

extern crate ureq;
extern crate chrono;
extern crate serde_json;

use chrono::prelude::Local;
use ureq::{Agent};

const HOST_URL: &str = "https://picaapi.picacomic.com/";
const API_KEY: &str = "C69BAF41DA5ABD1FFEDC6D2FEA56B";
const NONCE: &str = "b1ab87b4800d4d4590a11701b8551afa";
const DIGEST_KEY: &str = "~d}$Q7$eIni=V)9\\RK/P.RM4;9[7|@/CA}b~OW!3?EV`:<>M7pddUBL5n|0/*Cn";

/// 客户端
pub struct Client {
    agent: Agent,
    token: String,
}

/// 代理
fn request_agent(url: Option<&str>) -> ureq::Agent {
    match url {
        None => {
            ureq::AgentBuilder::new()
                .build()
        }
        Some(url) => {
            ureq::AgentBuilder::new()
                .proxy(ureq::Proxy::new(url).expect("error"))
                .build()
        }
    }
}

/// 接口实现
impl Client {
    /// 构造方法
    pub fn new() -> Self {
        Self {
            agent: request_agent(None),
            token: "".to_string(),
        }
    }

    /// 设置代理
    fn set_proxy(&mut self, url: Option<&str>) {
        self.agent = request_agent(url);
    }

    /// 请求和签名
    fn pica_request<T: for<'de> serde::Deserialize<'de>>(&self, method: &str, path: &str, body: Option<ureq::SerdeValue>) -> Result<T, Error> {
        let time = Local::now().timestamp().to_string();
        let request = self.agent.request(method, format!("{}{}", HOST_URL, path).as_str());
        let request = request.set("api-key", API_KEY)
            .set("accept", "application/vnd.picacomic.com.v1+json")
            .set("app-channel", "2")
            .set("time", time.as_str())
            .set("nonce", NONCE)
            .set("app-version", "2.2.1.2.3.3")
            .set("app-uuid", "defaultUuid")
            .set("app-platform", "android")
            .set("app-build-version", "44")
            .set("Content-Type", "application/json; charset=UTF-8")
            .set("User-Agent", "okhttp/3.8.1")
            .set("authorization", self.token.as_str())
            .set("image-quality", "original")
            .set("signature", hmac::hmac_hex(DIGEST_KEY, ("".to_string() + path + time.as_str() + NONCE + method + API_KEY).to_lowercase().as_str()).as_str());
        let resp = match body {
            None => {
                request.call()
            }
            Some(body) => {
                request.send_json(body)
            }
        };
        match resp {
            Ok(resp) => {
                let json: serde_json::Value = resp.into_json()?;
                let v = json.get("data")
                    .ok_or(Error::from("response data error"))?
                    .clone();
                let r = serde_json::from_value(v)?;
                Ok(r)
            }
            Err(err) => {
                match err {
                    ureq::Error::Status(_, resp) => {
                        let rsp: serde_json::Value = resp.into_json()?;
                        let message =
                            rsp.get("message").ok_or("message error")?
                                .as_str().ok_or("message error")?;
                        println!("M message : {}", message);
                        Err(Error::from(message))
                    }
                    ureq::Error::Transport(t) => {
                        Err(Error::from(t.to_string().as_str()))
                    }
                }
            }
        }
    }

    /// 用户登陆
    /// email 用户名 (不一定是邮箱)
    /// password 密码
    ///  # Examples
    /// ```
    /// client.login("password", "username")?;
    /// ```
    pub fn login(&mut self, email: &str, password: &str) -> Result<(), Error> {
        let data: LoginResponseData = self.pica_request("POST", "auth/sign-in", Some(ureq::json!({
            "email": email,
            "password": password,
            })))?;
        self.token = data.token;
        Ok(())
    }

    /// 随机漫画
    pub fn random_comics(&self) -> Result<Vec<ComicSimple>, Error> {
        let data: ComicListResponseData = self.pica_request("GET", "comics/random", None)?;
        Ok(data.comics)
    }

    /// 漫画信息
    pub fn comic_info(&self, comic_id: String) -> Result<ComicInfo, Error> {
        let data: ComicInfoResponseData = self.pica_request("GET", format!("comics/{}", comic_id).as_str(), None)?;
        Ok(data.comic)
    }
}

