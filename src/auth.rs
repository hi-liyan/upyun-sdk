use base64::{Engine as _, engine::general_purpose};
use reqwest::Client;
use crate::utils::{hmac_sha1, md5};

pub struct UpYunConfig {
    pub bucket: String,
    pub operator: String,
    pub password: String,
    pub timeout: Option<u64>
}

pub struct UpYun {
    pub config: UpYunConfig,
    pub http_client: Client
}

impl UpYun {
    /// 创建 Upyun 实例
    pub fn new(config: UpYunConfig) -> Self {
        UpYun {
            config: UpYunConfig {
                bucket: config.bucket,
                operator: config.operator,
                password: md5(config.password),
                timeout: config.timeout
            },
            http_client: Client::new()
        }
    }

    /// 生成认证签名
    pub fn gen_auth_signature(&self, config: RestAuthConfig) -> String {
        let pass = &self.config.password;
        let data = format!("{}&{}&{}&", config.method, config.uri, config.date);
        println!("data--> {}", data);
        let hmac_data = hmac_sha1(pass.as_bytes(), data.as_bytes());
        let signature = general_purpose::STANDARD.encode(hmac_data);
        format!("UPYUN {}:{}", self.config.operator, signature)
    }
}

pub struct RestAuthConfig {
    pub method: String,
    pub uri: String,
    pub date: String
}

