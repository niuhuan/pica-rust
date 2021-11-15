#[cfg(test)]
mod tests {
    use crate::{Client, ComicInfo, Error};

    #[test]
    fn it_works() {
        // init client
        let mut c = Client::new();
        // set proxy
        c.set_proxy(Option::Some("socks5://127.1:1080/"));
        // login
        match c.login("username", "password") {
            Ok(_) => {
                println!("{}", c.token);
            }
            Err(err) => {
                println!("{}", err);
                return;
            }
        }
        // test random comics
        match c.random_comics() {
            Ok(info) => {
                println!("{}", serde_json::to_string(&info).unwrap_or("".to_string()));
            }
            Err(err) => {
                println!("{}", err);
                return;
            }
        }
        // test comic info
        match c.comic_info("6073235eb978f81d68ef48db".to_string()) {
            Ok(info) => {
                println!("{}", serde_json::to_string(&info).unwrap_or("".to_string()));
            }
            Err(err) => {
                println!("{}", err);
                return;
            }
        };
    }
}
