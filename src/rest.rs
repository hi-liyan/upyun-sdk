use std::error::Error as StdError;
use std::str::FromStr;

use reqwest::{Error, Method, Response};
use reqwest::header::{HeaderMap, HeaderValue};
use url_escape::encode_path_to_string;
use crate::common::auth::{gen_auth_signature, RestAuthConfig};
use crate::common::error::ApiError;
use crate::common::http::{http, HttpConfig};
use crate::common::utils::get_date;
use crate::upyun::UpYun;

#[derive(Debug, Clone)]
struct RestReqConfig {
    method: Method,
    uri: String,
    query: Option<String>,
    headers: Option<HeaderMap>,
}

/// 发起 Rest Api 请求
async fn rest_request(upyun: &UpYun, config: &RestReqConfig) -> Result<Response, Error> {
    let endpoint = "https://v0.api.upyun.com";
    let mut encode_uri = format!("{}/{}", endpoint, &upyun.config.bucket);
    encode_path_to_string(&config.uri, &mut encode_uri);

    if !encode_uri.ends_with("/") {
        encode_uri.push('/');
    }

    if let Some(query) = &config.query {
        encode_uri.push_str(format!("?{}", query).as_str())
    }

    let mut headers = if let Some(headers) = config.headers.clone() {
        headers
    } else {
        HeaderMap::new()
    };

    let date = get_date();
    println!("日期--> {}", date);
    // 请求头添加认证签名
    let signature = gen_auth_signature(upyun, &RestAuthConfig {
        method: config.method.clone().to_string(),
        uri: config.uri.clone(),
        date: date.clone(),
    });
    println!("签名--> {}", signature);
    headers.append("Authorization", HeaderValue::from_str(signature.as_str()).unwrap());
    headers.append("Date", HeaderValue::from_str(date.clone().as_str()).unwrap());
    println!("HeaderValue--> {:?}", HeaderValue::from_str(date.clone().as_str()).unwrap());

    let http_config = HttpConfig {
        method: config.clone().method,
        url: encode_uri,
        headers: Some(headers)
    };
    http(upyun, &http_config).await
}

impl UpYun {
    /// 获取服务使用量
    pub async fn usage(&self) -> Result<u64, Box<dyn StdError>> {
        let result = rest_request(self, &RestReqConfig {
            method: Method::GET,
            uri: "/".to_string(),
            query: Some("usage".to_string()),
            headers: None,
        }).await;

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
    pub async fn mkdir(&self, path: String) -> Result<(), Box<dyn StdError>> {
        let mut headers = HeaderMap::new();
        headers.append("folder", HeaderValue::from_static("true"));
        headers.append("x-upyun-folder", HeaderValue::from_static("true"));

        let result = rest_request(self, &RestReqConfig {
            method: Method::POST,
            uri: path,
            query: None,
            headers: Some(headers),
        }).await;

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