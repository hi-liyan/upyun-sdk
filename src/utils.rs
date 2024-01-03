use chrono::Utc;
use chrono_tz::Asia::Shanghai;
use crypto::digest::Digest;
use crypto::md5::Md5;
use hmac::{Hmac, Mac};
use sha1::Sha1;

/// 获取日期时间，GMT 格式字符串 (RFC 1123)，如 Wed, 29 Oct 2014 02:26:58 GMT
pub fn get_date() -> String {
    let now_time = Utc::now().with_timezone(&Shanghai);
    now_time.format("%a, %d %b %Y %H:%M:%S GMT").to_string()
}

/// HMAC SHA1 加密
pub fn hmac_sha1(key: &[u8], message: &[u8]) -> Vec<u8> {
    let mut hmac: Hmac<Sha1> = Hmac::<Sha1>::new_from_slice(key).expect("HMAC can take key of any size");
    hmac.update(message);
    hmac.finalize().into_bytes().to_vec()
}

/// 字符串 MD5 加密
pub fn md5(input: String) -> String {
    let mut md5 = Md5::new();
    md5.input(input.as_bytes());
    md5.result_str()
}