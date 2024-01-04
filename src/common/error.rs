use std::error::Error;
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

/// API 请求失败时，又拍云返回的错误信息的结构体
///
/// 参考：
/// - https://help.upyun.com/knowledge-base/errno/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub id: String,
    pub code: u32,
    pub msg: String
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error {} (code {}): {}", self.id, self.code, self.msg)
    }
}

impl Error for ApiError {}