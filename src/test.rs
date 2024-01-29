use crate::types::Result;
use crate::{Client, RegisterDto, Sort};
use serde::Serialize;

fn create_client() -> Client {
    Client::new()
}

fn print<T: Serialize>(result: Result<T>) {
    match result {
        Ok(data) => println!("{}", serde_json::to_string(&data).unwrap()),
        Err(err) => panic!("{}", err),
    }
}

/// 测试 注册
#[tokio::test]
async fn register() {
    match create_client()
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
async fn login() {
    let c = create_client();
    c.set_token("token").await;
    match c.login("username", "password").await {
        Ok(_) => {
            println!("{}", c.token().await);
        }
        Err(err) => {
            println!("{}", err);
            return;
        }
    }
}

/// 测试 用户信息
#[tokio::test]
async fn user_profile() {
    print(create_client().user_profile().await)
}

/// 测试 打卡
#[tokio::test]
async fn punch_in() {
    print(create_client().punch_in().await)
}

/// 测试 本子分页
#[tokio::test]
async fn comics() {
    print(
        create_client()
            .comics(None, None, None, None, Sort::SORT_DEFAULT, 1)
            .await,
    )
}

#[tokio::test]
async fn comics_random() {
    print(create_client().comics_random().await)
}

#[tokio::test]
async fn comic_info() {
    print(
        create_client()
            .comic_info("5b6bdf4558ed442d899486b7".to_string())
            .await,
    )
}

#[tokio::test]
async fn comic_eps() {
    print(
        create_client()
            .comic_eps("5b6bdf4558ed442d899486b7".to_string(), 1)
            .await,
    )
}

#[tokio::test]
async fn comic_ep_pictures() {
    print(
        create_client()
            .comic_ep_pictures("5b6bdf4558ed442d899486b7".to_string(), 1, 1)
            .await,
    )
}

#[tokio::test]
async fn favourite_comics() {
    print(
        create_client()
            .favourite_comics(Sort::SORT_DEFAULT, 1)
            .await,
    )
}

#[tokio::test]
async fn switch_like() {
    print(
        create_client()
            .switch_like("5b6bdf4558ed442d899486b7".to_string())
            .await,
    )
}

#[tokio::test]
async fn comic_comments() {
    print(
        create_client()
            .comic_comments("5b6bdf4558ed442d899486b7".to_string(), 1)
            .await,
    )
}

#[tokio::test]
async fn post_comment() {
    print(
        create_client()
            .post_comment("5b6bdf4558ed442d899486b7".to_string(), "".to_string())
            .await,
    )
}

#[tokio::test]
async fn advanced_search() {
    print(
        create_client()
            .advanced_search("abc".to_string(), Sort::SORT_DEFAULT, 1, Vec::new())
            .await,
    )
}


