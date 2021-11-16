extern crate hmac;
extern crate sha2;

use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn hmac_hex(key: &str, str: &str) -> String {
    let mac = HmacSha256::new_from_slice(key.as_bytes());
    let mut mac = mac.expect("HMAC can take key of any size");
    mac.update(str.as_bytes());
    return hex::encode(mac.finalize().into_bytes().as_slice());
}
