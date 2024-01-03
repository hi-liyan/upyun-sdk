use std::collections::HashMap;

pub struct UpYunConfig {
    bucket: String,
    operator: String,
    password: String,
    hosts: HashMap<String, String>,
    user_agent: String,
}

pub struct UpYun {
    config: UpYunConfig,
}

impl UpYun {
    /// 创建 Upyun 实例
    pub fn new(config: UpYunConfig) -> Self {

        todo!()
    }

    /// 生成认证签名
    pub fn gen_auth_signature(&self) -> String {

        todo!()
    }
}

pub struct UnifiedAuthConfig {
    pub method: String,
    pub uri: String,
    pub date_str: String,
    pub content_md5: Option<String>
}

