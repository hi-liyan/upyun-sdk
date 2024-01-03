use std::time::Duration;

use reqwest::{Error, Method, Response};
use reqwest::header::{HeaderMap, HeaderValue};

use crate::auth::{RestAuthConfig, UpYun};

impl UpYun {
    pub async fn do_http_request(&self, method: Method, url: String, headers: Option<HeaderMap>) -> Result<Response, Error> {
        let client = &self.http_client;

        // 请求超时时间，默认3000ms
        let timeout = if let Some(timeout) = self.config.timeout {
            timeout
        } else {
            3000
        };

        let mut req_builder = client.request(method, url)
            .timeout(Duration::from_millis(timeout));

        if let Some(headers) = headers {
            req_builder = req_builder.headers(headers);
        }

        let req = req_builder
            .build()
            .unwrap();

        client.execute(req).await
    }
}