#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use crate::{Client, RegisterDto, Sort, SwitchAddress};

    #[test]
    fn it_works() {
        // init client
        let mut c = Client::new();
        // set proxy
        match c.set_proxy(None, SwitchAddress::ADDRESS1) {
            Ok(_) => {
                println!("PROXY OK");
            }
            Err(err) => {
                println!("PROXY ERROR : {}", err);
                return;
            }
        }

        // 测试 注册
        // match c.register(RegisterDto {
        //     email: "username".to_string(),
        //     password: "password".to_string(),
        //     name: "name".to_string(),
        //     birthday: "2000-01-01".to_string(),
        //     gender: "m".to_string(),
        //     answer1: "回答1".to_string(),
        //     answer2: "回答2".to_string(),
        //     answer3: "回答3".to_string(),
        //     question1: "问题1".to_string(),
        //     question2: "问题2".to_string(),
        //     question3: "问题3".to_string(),
        // }) {
        //     Ok(_) => {
        //         println!("register OK");
        //     }
        //     Err(error) => {
        //         println!("register ERROR : {}", error.to_string());
        //     }
        // }

        // 测试登录
        // match c.login("username", "password") {
        //     Ok(_) => {
        //         println!("{}", c.token);
        //     }
        //     Err(err) => {
        //         println!("{}", err);
        //         return;
        //     }
        // }

        // 测试登录
        // c.token = "".to_string();

        // 测试 用户信息
        // match c.user_profile() {
        //     Ok(profile) => {
        //         println!("{}", serde_json::to_string(&profile).unwrap_or("".to_string()));
        //     }
        //     Err(err) => {
        //         println!("{}", err);
        //         return;
        //     }
        // }

        // 测试 打卡
        // match c.punch_in() {
        //     Ok(status) => {
        //         println!("{}", serde_json::to_string(&status).unwrap_or("".to_string()));
        //     }
        //     Err(err) => {
        //         println!("{}", err);
        //         return;
        //     }
        // }

        // 测试 本子分页
        // match c.comics(None, None, None, None, Sort::SORT_DEFAULT, 1) {
        //     Ok(info) => {
        //         println!("{}", serde_json::to_string(&info).unwrap_or("".to_string()));
        //     }
        //     Err(err) => {
        //         println!("{}", err);
        //         return;
        //     }
        // }

        // 测试 随机本子
        // match c.comics_random() {
        //     Ok(info) => {
        //         println!("{}", serde_json::to_string(&info).unwrap_or("".to_string()));
        //     }
        //     Err(err) => {
        //         println!("{}", err);
        //         return;
        //     }
        // }

        // 测试 漫画信息
        // match c.comic_info("5b6bdf4558ed442d899486b7".to_string()) {
        //     Ok(info) => {
        //         println!("{}", serde_json::to_string(&info).unwrap_or("".to_string()));
        //     }
        //     Err(err) => {
        //         println!("{}", err);
        //         return;
        //     }
        // };

        // 测试 漫画EP
        // match c.comic_eps("5b6bdf4558ed442d899486b7".to_string(), 1) {
        //     Ok(eps) => {
        //         println!("{}", serde_json::to_string(&eps).unwrap_or("".to_string()));
        //     }
        //     Err(err) => {
        //         println!("{}", err);
        //         return;
        //     }
        // };

        // 测试 EP图片
        // match c.comic_ep_pictures("5b6bdf4558ed442d899486b7".to_string(), 1, 1) {
        //     Ok(pictures) => {
        //         println!("{}", serde_json::to_string(&pictures).unwrap_or("".to_string()));
        //     }
        //     Err(err) => {
        //         println!("{}", err);
        //         return;
        //     }
        // };

        // 测试 收藏的漫画
        // match c.favourite_comics(Sort::SORT_DEFAULT, 1) {
        //     Ok(data) => {
        //         println!("{}", serde_json::to_string(&data).unwrap_or("".to_string()));
        //     }
        //     Err(err) => {
        //         println!("{}", err);
        //         return;
        //     }
        // }
    }
}
