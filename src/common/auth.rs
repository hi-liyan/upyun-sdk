use base64::Engine;
use base64::engine::general_purpose;
use crate::common::utils::hmac_sha1;
use crate::upyun::UpYun;

/// 认证签名配置结构体
pub struct RestAuthConfig {
    pub method: String,
    pub uri: String,
    pub date: String
}

/// 生成认证签名
pub fn gen_auth_signature(upyun: &UpYun, config: &RestAuthConfig) -> String {
    let pass = &upyun.config.password;
    let data = format!("{}&{}&{}", config.method, config.uri, config.date);
    println!("data--> {}", data);
    let hmac_data = hmac_sha1(pass.as_bytes(), data.as_bytes());
    let signature = general_purpose::STANDARD.encode(hmac_data);
    format!("UPYUN {}:{}", &upyun.config.operator, signature)
    // "Basic cnVzdDE6WnpTeFZFU3A0T3ZxdmFMRWtlSXVUT2Y1dHNuMkxQc00=".to_string()
}

