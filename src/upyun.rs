use reqwest::Client;
use crate::common::utils::md5;

/// Upyun 实例配置
pub struct UpYunConfig {
    pub bucket: String,
    pub operator: String,
    pub password: String,
    pub timeout: Option<u64>
}

/// Upyun 实例
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
}