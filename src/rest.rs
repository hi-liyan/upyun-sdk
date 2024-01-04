use std::error::Error as StdError;
use std::str::FromStr;

use reqwest::{Error, Method, Response};
use reqwest::header::{HeaderMap, HeaderValue};
use url_escape::encode_path;

use crate::common::error::ApiError;
use crate::common::utils::{get_rfc1123_date, sign};
use crate::upyun::UpYun;

impl UpYun {
    async fn request(&self, method: Method, uri: &str, query: Option<&str>, headers: Option<HeaderMap>) -> Result<Response, Error> {
        // 获取当前时间的 RFC1123 格式
        let date = get_rfc1123_date();

        // 获取 Endpoint 和整理 URI
        let endpoint = self.endpoint.value();
        let uri = format!("/{}", uri).trim_start_matches('/').to_string();
        let path = format!("/{}/{}", self.bucket, uri);


        // 构建 Query 参数
        let query = query.map_or_else(|| "".to_string(), |q| format!("?{}", q));

        // 构建完整的 URL
        let url = format!("{}{}{}", endpoint, path, query);

        // 生成签名
        let sign = sign(
            &method.to_string(),
            &encode_path(&path).to_string(),
            &date,
            &self.operator,
            &self.password,
        );

        // 设置请求头
        let mut headers = headers.unwrap_or_default();
        headers.append("Date", HeaderValue::from_str(&date).unwrap());
        headers.append("Authorization", HeaderValue::from_str(&sign).unwrap());

        // 发起 HTTP 请求
        self.http(method, url, Some(headers)).await
    }
}

impl UpYun {
    /// 获取服务使用量
    pub async fn usage(&self) -> Result<u64, Box<dyn StdError>> {
        let result = self.request(
            Method::GET,
            "/",
            Some("usage"),
            None,
        ).await;

        let resp = match result {
            Ok(r) => r,
            Err(e) => {
                return Err(Box::try_from(e).unwrap());
            }
        };

        if resp.status().as_u16() == 200 {
            let value_str = resp.text().await.unwrap();
            let value = u64::from_str(value_str.as_str()).unwrap();
            return Ok(value);
        } else {
            let error: ApiError = resp.json().await.unwrap();
            return Err(Box::try_from(error).unwrap());
        }
    }

    /// 创建目录
    pub async fn mkdir(&self, path: &str) -> Result<(), Box<dyn StdError>> {
        let mut headers = HeaderMap::new();
        headers.append("folder", HeaderValue::from_static("true"));
        headers.append("x-upyun-folder", HeaderValue::from_static("true"));

        let result = self.request(
            Method::POST,
            path,
            None,
            Some(headers),
        ).await;

        let resp = match result {
            Ok(r) => r,
            Err(e) => {
                return Err(Box::try_from(e).unwrap());
            }
        };

        if resp.status().as_u16() != 200 {
            let error: ApiError = resp.json().await.unwrap();
            return Err(Box::try_from(error).unwrap());
        }

        Ok(())
    }
}