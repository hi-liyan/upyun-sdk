use std::time::Duration;

use reqwest::{Error, Method, Response};
use reqwest::header::HeaderMap;

use crate::upyun::UpYun;

impl UpYun {
    /// 发起 HTTP 请求
    pub async fn http(&self, method: Method, url: String, headers: Option<HeaderMap>) -> Result<Response, Error> {
        // 创建一个请求构建器，并设置超时时间
        let mut req_builder = self.client
            .request(method, url)
            .timeout(Duration::from_millis(self.timeout));

        // 如果有传入请求头，则添加到请求构建器中
        if let Some(headers) = headers {
            req_builder = req_builder.headers(headers);
        }

        // 构建请求
        let req = req_builder
            .build()
            .unwrap();

        // 使用 reqwest 执行请求，并返回结果
        self.client.execute(req).await
    }
}

