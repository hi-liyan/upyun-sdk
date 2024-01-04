use base64::Engine;
use base64::engine::general_purpose;
use chrono::Utc;
use crypto::digest::Digest;
use crypto::md5::Md5;
use hmac::{Hmac, Mac};
use sha1::Sha1;

/// 获取日期时间，GMT 格式字符串 (RFC 1123)，如 Wed, 29 Oct 2014 02:26:58 GMT
pub fn get_rfc1123_date() -> String {
    let now_time = Utc::now();
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

/// 生成认证签名
pub fn sign(method: &String, path: &String, date: &String, operator: &String, password: &String) -> String {
    let raw = format!("{}&{}&{}", method, path, date);

    // 计算 HMAC-SHA1
    let hmac_data = hmac_sha1(password.as_bytes(), raw.as_bytes());

    // 将结果进行 Base64 编码
    let signature = general_purpose::STANDARD.encode(&hmac_data);

    format!("UPYUN {}:{}", operator, signature)
}

/// 为了测试
pub mod test {
    use std::{fs, io};
    use crate::upyun::UpYun;

    /// 读取 cred.txt 文件，里面包含凭证信息
    ///
    /// 文件内容格式：
    ///
    /// bucket:operator:password
    #[allow(unused)]
    fn read_cred_file() -> io::Result<Vec<String>> {
        // 读取 cred.txt 文件内容到字符串
        let contents = fs::read_to_string("cred.txt")?;

        let cred = contents.split(':').map(|s| { s.to_string() }).collect();
        Ok(cred)
    }

    /// 获取 Upyun 实例
    #[allow(unused)]
    pub fn get_upyun() -> UpYun {
        let cred = read_cred_file().unwrap();

        UpYun::builder()
            .bucket(&cred[0])
            .operator(&cred[1])
            .password(&cred[2])
            .build()
    }
}