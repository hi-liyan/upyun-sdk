use std::collections::HashMap;
use std::error::Error;
use url_escape::encode_path_to_string;
use crate::auth::UpYun;

struct RestReqConfig {
    method: String,
    uri: String,
    query: String,
    headers: HashMap<String, String>,
}

impl UpYun {
    fn do_rest_request(&self, config: RestReqConfig) -> Result<(), Box<dyn Error>> {
        let mut uri = format!("/{}", self.config.bucket);
        encode_path_to_string(config.uri, &mut uri);

        todo!()
    }

    /// 获取服务使用量
    pub fn usage(&self) -> Result<u64, Box<dyn Error>> {
        let result = self.do_rest_request(RestReqConfig {
            method: "GET".to_string(),
            uri: "/".to_string(),
            query: "usage".to_string(),
            headers: Default::default(),
        });
        todo!()
    }

    /// 创建目录
    pub fn mkdir(&self, path: String) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}