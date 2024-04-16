use reqwest::{Client, ClientBuilder};
use crate::common::utils::md5;

/// Rest Api 接入点
///
/// 可选值：
///
/// `Auto`：根据网络条件自动选择接入点:v0.api.upyun.com（默认）
///
/// `Telecom`：电信接入点:v1.api.upyun.com
///
/// `Cnc`：联通网通接入点:v2.api.upyun.com
///
/// `Ctt`：移动铁通接入点:v3.api.upyun.com
#[derive(Clone)]
pub enum Endpoint {
    Auto,
    Telecom,
    Cnc,
    Ctt,
}

impl Endpoint {
    pub fn value(&self) -> &'static str {
        match self {
            Endpoint::Auto => "https://v0.api.upyun.com",
            Endpoint::Telecom => "https://v1.api.upyun.com",
            Endpoint::Cnc => "https://v2.api.upyun.com",
            Endpoint::Ctt => "https://v3.api.upyun.com"
        }
    }
}

/// Upyun 实例
pub struct UpYun {
    /// 服务名称
    pub bucket: String,
    /// 操作员
    pub operator: String,
    /// 密码
    pub password: String,
    /// 请求超时时间（默认：30s）
    pub timeout: u64,
    /// 接入点（默认为自动识别接入点）
    pub endpoint: Endpoint,
    /// HTTP 客户端
    pub client: Client
}

/// Upyun 实例构造器
pub struct UpyunBuilder {
    bucket: Option<String>,
    operator: Option<String>,
    password: Option<String>,
    timeout: Option<u64>,
    endpoint: Option<Endpoint>,
    danger_accept_invalid_certs: bool
}

impl UpYun {
    /// 构造器
    pub fn builder() -> UpyunBuilder {
        UpyunBuilder {
            bucket: None,
            operator: None,
            password: None,
            timeout: None,
            endpoint: None,
            danger_accept_invalid_certs: false
        }
    }
}

impl UpyunBuilder {
    /// 服务名称
    pub fn bucket(mut self, bucket: &str) -> Self {
        self.bucket = Some(bucket.to_string());
        self
    }

    /// 操作员
    pub fn operator(mut self, operator: &str) -> Self {
        self.operator = Some(operator.to_string());
        self
    }

    /// 密码
    pub fn password(mut self, password: &str) -> Self {
        self.password = Some(password.to_string());
        self
    }

    /// 请求超时时间（默认：30s）
    pub fn timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// 接入点（默认为自动识别接入点）
    pub fn endpoint(mut self, endpoint: Endpoint) -> Self {
        self.endpoint = Some(endpoint);
        self
    }

    /// 忽略证书验证
    pub fn danger_accept_invalid_certs(mut self, danger_accept_invalid_certs: bool) -> Self {
        self.danger_accept_invalid_certs = danger_accept_invalid_certs;
        self
    }

    /// 构造 Upyun 实例
    pub fn build(self) -> UpYun {
        if self.bucket.is_none() {
            panic!("Bucket is required.")
        }
        if self.operator.is_none() {
            panic!("Operator is required.")
        }
        if self.password.is_none() {
            panic!("Password is required.")
        }

        UpYun {
            bucket: self.bucket.unwrap(),
            operator: self.operator.unwrap(),
            // 密码使用 MD5 加密
            password: md5(self.password.unwrap()),
            // 超时时间默认30s
            timeout: self.timeout.unwrap_or(30 * 1000),
            // 默认为自动识别接入点
            endpoint: self.endpoint.unwrap_or(Endpoint::Auto),
            client: ClientBuilder::new()
                .danger_accept_invalid_certs(self.danger_accept_invalid_certs)
                .build()
                .unwrap()
        }
    }
}