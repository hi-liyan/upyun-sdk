use std::time::Duration;
use reqwest::header::HeaderMap;
use reqwest::{Error, Method, Response};
use crate::upyun::UpYun;

#[derive(Debug, Clone)]
pub struct HttpConfig {
    pub method: Method,
    pub url: String,
    pub headers: Option<HeaderMap>
}

pub async fn http(upyun: &UpYun, config: &HttpConfig) -> Result<Response, Error> {
    let client = &upyun.http_client;

    // 请求超时时间，默认3000ms
    let timeout = if let Some(timeout) = upyun.config.timeout {
        timeout
    } else {
        3000
    };

    let mut req_builder = client.request(config.method.clone(), &config.url)
        .timeout(Duration::from_millis(timeout));

    if let Some(headers) = config.headers.clone() {
        req_builder = req_builder.headers(headers);
    }

    let req = req_builder
        .build()
        .unwrap();

    client.execute(req).await
}