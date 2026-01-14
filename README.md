PICA-RUST
===========
Rust哔卡漫画API

## 实现功能

- [x] 用户
    - [x] 注册 / 登录 / 获取用户信息 / 打卡
    - [x] 修改签名 / 修改头像
    - [x] 找回密码
- [x] 漫画
    - [x] 分类 / 随机本子 / 获取章节 / 获取图片
    - [x] 收藏漫画 / 喜欢漫画
    - [x] 获取漫画评论 / 对漫画发表评论及回复
    - [x] 我的评论 / 喜欢/取消喜欢评论 /
    - [x] 搜索漫画 / 大家都在搜
    - [x] 排行榜 / 骑士榜 
    - [x] 看了这个本子的也在看
    - [x] 游戏列表 / 详情 / 评论相关
- [x] 网络
    - [x] 获取官方分流
    - [x] 代理 / 分流

## 使用方法

在Cargo.toml中增加依赖

```toml
[dependencies]
pica = { git = "https://github.com/niuhuan/pica-rust.git", branch = "master" }
```

基本使用

```rust
use pica::{Client, Sort};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建客户端
    let client = Client::new().await;

    // 可选：设置代理或分流
    client.set_proxy(None).await?;
    client
        .set_switch_addresses(vec!["172.67.80.1:443".to_string()])
        .await?;

    // 登录或直接注入 token
    client.login("username", "password").await?;
    // client.set_token("token").await;

    // 获取漫画列表
    let comics = client
        .comics(None, None, None, None, None, Sort::SORT_DEFAULT, 1)
        .await?;

    println!("got {} comics", comics.docs.len());
    Ok(())
}
```

链式 Builder 示例

```rust
// 与 comics 等效，但可链式配置后直接 .await
let page = client
    .comics_builder()
    .category("纯爱")
    .sort(Sort::SORT_TIME_NEWEST)
    .page(1)
    .await?;
println!("docs: {}", page.docs.len());
```
