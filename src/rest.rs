use std::collections::HashMap;
use std::error::Error;
use crate::auth::UpYun;

struct RestReqConfig {
    method: String,
    uri: String,
    query: String,
    headers: HashMap<String, String>,
    close_body: bool,
    // http_body: ,
    use_md5: bool
}

impl UpYun {
    fn do_rest_request(&self, config: &RestReqConfig) -> Result<(), Box<dyn Error>> {

        todo!()
    }

    /// 获取服务使用量
    pub fn usage(&self) -> Result<u64, Box<dyn Error>> {
        todo!()
    }

    /// 创建目录
    pub fn mkdir(&self, path: String) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}