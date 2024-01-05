use reqwest::Client;
use crate::common::utils::md5;

/// Rest Api 接入点
///
/// 可选值：
///
/// `EdAuto`：根据网络条件自动选择接入点:v0.api.upyun.com（默认）
///
/// `EdTelecom`：电信接入点:v1.api.upyun.com
///
/// `EdCnc`：联通网通接入点:v2.api.upyun.com
///
/// `EdCtt`：移动铁通接入点:v3.api.upyun.com
#[derive(Clone)]
pub enum Endpoint {
    EdAuto,
    EdTelecom,
    EdCnc,
    EdCtt,
}

impl Endpoint {
    pub fn value(&self) -> &'static str {
        match self {
            Endpoint::EdAuto => "https://v0.api.upyun.com",
            Endpoint::EdTelecom => "https://v1.api.upyun.com",
            Endpoint::EdCnc => "https://v2.api.upyun.com",
            Endpoint::EdCtt => "https://v3.api.upyun.com"
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
    pub bucket: Option<String>,
    pub operator: Option<String>,
    pub password: Option<String>,
    pub timeout: Option<u64>,
    pub endpoint: Option<Endpoint>,
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
            endpoint: self.endpoint.unwrap_or(Endpoint::EdAuto),
            client: Client::new()
        }
    }
}