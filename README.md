PICA-RUST
===========
Rust哔卡漫画API

## 实现功能

- [x] 用户
    - [x] 注册 / 登录 / 获取用户信息 / 打卡
- [x] 漫画
    - [x] 分类 / 随机本子 / 获取章节 / 获取图片
    - [x] 收藏漫画 / 喜欢漫画
    - [x] 获取漫画评论 / 对漫画发表评论及回复
- [x] 网络
    - [x] 代理 / 分流

## 使用方法

在Cargo.toml中增加依赖

```toml
[dependencies]
pica = { git = "https://github.com/niuhuan/pica-rust.git", branch = "master" }
```

调用客户端

```rust
use pica::{Client, Sort, SwitchAddress};

async fn main() {
    // 创建客户端并设置分流
    let mut client = Client::new();
    client.set_proxy(None, SwitchAddress::ADDRESS1);
    // 登录或注入token (选择其一)
    client.login("username", "password").await.unwarp();
    client.token = "".to_string();
    // 获取漫画列表
    client.comics(None, None, None, None, Sort::SORT_DEFAULT, 1);
    // ...
}
```
