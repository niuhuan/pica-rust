use crate::types::Result;
use crate::{Client, RegisterDto, Sort};
use serde::Serialize;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

#[derive(Serialize, serde::Deserialize)]
struct CachedToken {
    token: String,
    expires_at: u64,
}

async fn create_client() -> Client {
    let c = Client::new().await;

    if let Ok(switch_addr) = env::var("PICA_SWITCH_ADDRESS") {
        let addrs: Vec<String> = switch_addr
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let _ = c.set_switch_addresses(addrs).await;
    }
    if let Ok(image_host) = env::var("PICA_IMAGE_SWITCH_HOST") {
        let trimmed = image_host.trim().to_string();
        let _ = c
            .set_image_switch_host(if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            })
            .await;
    }

    if let Some(token) = load_cached_token() {
        c.set_token(token).await;
        return c;
    }

    let username = env::var("PICA_USERNAME").expect("PICA_USERNAME not set");
    let password = env::var("PICA_PASSWORD").expect("PICA_PASSWORD not set");


    c.login(username.as_str(), password.as_str())
        .await
        .expect("login failed");
    cache_token(c.token().await);
    c
}

fn print<T: Serialize>(result: Result<T>) {
    match result {
        Ok(data) => println!("{}", serde_json::to_string(&data).unwrap()),
        Err(err) => panic!("{}", err),
    }
}

fn cache_file() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push("pica_token_cache.json");
    path
}

fn cache_token(token: String) {
    let expires_at = SystemTime::now()
        .checked_add(Duration::from_secs(3600))
        .unwrap()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let cached = CachedToken { token, expires_at };
    let _ = serde_json::to_string(&cached)
        .ok()
        .and_then(|s| fs::write(cache_file(), s).ok());
}

fn load_cached_token() -> Option<String> {
    let path = cache_file();
    let contents = fs::read_to_string(path).ok()?;
    let cached: CachedToken = serde_json::from_str(contents.as_str()).ok()?;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .ok()?
        .as_secs();
    if cached.expires_at > now {
        Some(cached.token)
    } else {
        None
    }
}

/// 测试 注册
#[tokio::test]
async fn register() {
    let c = create_client().await;
    match c
        .register(RegisterDto {
            email: "username".to_string(),
            password: "password".to_string(),
            name: "name".to_string(),
            birthday: "2000-01-01".to_string(),
            gender: "m".to_string(),
            answer1: "回答1".to_string(),
            answer2: "回答2".to_string(),
            answer3: "回答3".to_string(),
            question1: "问题1".to_string(),
            question2: "问题2".to_string(),
            question3: "问题3".to_string(),
        })
        .await
    {
        Ok(_) => {
            println!("register OK");
        }
        Err(error) => {
            println!("register ERROR : {}", error.to_string());
        }
    }
}

/// 测试登录
#[tokio::test]
async fn user_profile() {
    let c = create_client().await;
    print(c.user_profile().await)
}

/// 测试 打卡
#[tokio::test]
async fn punch_in() {
    let c = create_client().await;
    print(c.punch_in().await)
}

/// 测试 本子分页
#[tokio::test]
async fn comics() {
    let c = create_client().await;
    print(
        c.comics(None, None, None, None, None, Sort::SORT_DEFAULT, 1)
            .await,
    )
}

#[tokio::test]
async fn comics_random() {
    let c = create_client().await;
    print(c.comics_random().await)
}

#[tokio::test]
async fn comic_info() {
    let c = create_client().await;
    print(c.comic_info("5b6bdf4558ed442d899486b7".to_string()).await)
}

#[tokio::test]
async fn comic_eps() {
    let c = create_client().await;
    print(c.comic_eps("5b6bdf4558ed442d899486b7".to_string(), 1).await)
}

#[tokio::test]
async fn comic_ep_pictures() {
    let c = create_client().await;
    print(
        c.comic_ep_pictures("5b6bdf4558ed442d899486b7".to_string(), 1, 1)
            .await,
    )
}

#[tokio::test]
async fn favourite_comics() {
    let c = create_client().await;
    print(c.favourite_comics(Sort::SORT_DEFAULT, 1).await)
}

#[tokio::test]
async fn switch_like() {
    let c = create_client().await;
    print(c.switch_like("5b6bdf4558ed442d899486b7".to_string()).await)
}

#[tokio::test]
async fn comic_comments() {
    let c = create_client().await;
    print(
        c.comic_comments("5b6bdf4558ed442d899486b7".to_string(), 1)
            .await,
    )
}

#[tokio::test]
async fn post_comment() {
    let c = create_client().await;
    print(
        c.post_comment("5b6bdf4558ed442d899486b7".to_string(), "".to_string())
            .await,
    )
}

#[tokio::test]
async fn advanced_search() {
    let c = create_client().await;
    print(
        c.advanced_search("abc".to_string(), Sort::SORT_DEFAULT, 1, None)
            .await,
    )
}
