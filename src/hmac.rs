extern crate hmac;
extern crate sha2;

use sha2::Sha256;
use hmac::{Hmac, Mac, NewMac};

type HmacSha256 = Hmac<Sha256>;

pub fn hmac_hex(key: &str, str: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(key.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(str.as_bytes());
    return hex::encode(mac.finalize().into_bytes().as_slice());
}
