use std::error::Error;
use std::str::FromStr;

use reqwest::{Body, Error as ReqwestError, Method, Response};
use reqwest::header::{HeaderMap, HeaderValue};
use url_escape::encode_path;

use crate::common::error::ApiError;
use crate::common::utils::{get_rfc1123_date, http, sign};
use crate::rest_type::{CopyParams, FileInfo, ListDir, ListDirParams, MoveParams, UploadParams};
use crate::upyun::UpYun;

/// 请求配置
struct RequestConfig {
    method: Method,
    uri: String,
    query: Option<String>,
    headers: Option<HeaderMap>,
    body: Option<Body>,
}

impl UpYun {
    async fn request(&self, config: RequestConfig) -> Result<Response, ReqwestError> {
        // 获取当前时间的 RFC1123 格式
        let date = get_rfc1123_date();

        // 获取 Endpoint 和整理 URI
        let endpoint = self.endpoint.value();
        let uri = format!("/{}", config.uri).trim_start_matches('/').to_string();
        let path = format!("/{}/{}", self.bucket, uri);

        // 构建 Query 参数
        let query = config.query.map_or_else(|| "".to_string(), |q| format!("?{}", q));

        // 构建完整的 URL
        let url = format!("{}{}{}", endpoint, path, query);

        // 生成签名
        let sign = sign(
            &config.method.to_string(),
            &encode_path(&path).to_string(),
            &date,
            &self.operator,
            &self.password,
        );

        // 设置请求头
        let mut headers = config.headers.unwrap_or_default();
        headers.append("Date", HeaderValue::from_str(&date).unwrap());
        headers.append("Authorization", HeaderValue::from_str(&sign).unwrap());

        // 发起 HTTP 请求
        http(self, config.method, url, Some(headers), config.body).await
    }
}

impl UpYun {
    /// 获取服务使用量
    pub async fn usage(&self) -> Result<u64, Box<dyn Error>> {
        let resp = self.request(RequestConfig {
            method: Method::GET,
            uri: "/".to_string(),
            query: Some("usage".to_string()),
            headers: None,
            body: None,
        }).await?;

        let resp = handle_response(resp).await?;
        let value_str = resp.text().await.unwrap();
        let value = u64::from_str(value_str.as_str()).unwrap();
        Ok(value)
    }

    /// 创建目录
    pub async fn mkdir(&self, path_to_folder: &str) -> Result<(), Box<dyn Error>> {
        let mut headers = HeaderMap::new();
        headers.append("folder", HeaderValue::from_static("true"));
        headers.append("x-upyun-folder", HeaderValue::from_static("true"));

        let resp = self.request(RequestConfig {
            method: Method::PUT,
            uri: path_to_folder.to_string(),
            query: None,
            headers: Some(headers),
            body: None,
        }).await?;

        handle_response(resp).await?;

        Ok(())
    }

    /// 删除目录或文件
    ///
    /// path 可以是目录或文件路径，如果是目录，只允许删除空的目录，否则删除请求会被拒绝
    pub async fn rm(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let resp = self.request(RequestConfig {
            method: Method::DELETE,
            uri: path.to_string(),
            query: None,
            headers: None,
            body: None,
        }).await?;

        handle_response(resp).await?;

        Ok(())
    }

    /// 获取文件信息
    pub async fn file_info(&self, path_to_file: &str) -> Result<FileInfo, Box<dyn Error>> {
        let resp = self.request(RequestConfig {
            method: Method::HEAD,
            uri: path_to_file.to_string(),
            query: None,
            headers: None,
            body: None,
        }).await?;

        let resp = handle_response(resp).await?;
        let headers = resp.headers();

        Ok(FileInfo {
            x_upyun_file_type: headers.get("x-upyun-file-type").unwrap().to_str().unwrap().to_string(),
            x_upyun_file_size: headers.get("x-upyun-file-size").map(|v| v.to_str().unwrap().parse::<u64>().unwrap()),
            x_upyun_file_date: headers.get("x-upyun-file-date").unwrap().to_str().unwrap().to_string(),
            content_md5: headers.get("Content-Md5").map(|v| v.to_str().unwrap().to_string()),
        })
    }

    /// 获取目录文件列表
    pub async fn list_dir(&self, path_to_folder: &str, params: Option<ListDirParams>) -> Result<ListDir, Box<dyn Error>> {
        // 设置请求头
        let mut headers = HeaderMap::new();
        headers.append("Accept", HeaderValue::from_str("application/json").unwrap());

        if let Some(params) = params {
            if let Some(x_list_iter) = params.x_list_iter {
                headers.append("x-upyun-list-iter", HeaderValue::from_str(&x_list_iter.to_string()).unwrap());
            }
            if let Some(x_list_limit) = params.x_list_limit {
                headers.append("x-upyun-list-limit", HeaderValue::from_str(&x_list_limit.to_string()).unwrap());
            }
            if let Some(x_list_order) = &params.x_list_order {
                headers.append("x-upyun-list-order", HeaderValue::from_str(x_list_order).unwrap());
            }
        }

        let resp = self.request(RequestConfig {
            method: Method::GET,
            uri: path_to_folder.to_string(),
            query: None,
            headers: Some(headers),
            body: None,
        }).await?;

        let resp = handle_response(resp).await?;
        let value: ListDir = resp.json().await.unwrap();

        Ok(value)
    }

    /// 复制文件
    ///
    /// 同一个 `bucket` 下复制文件。并且它只能操作文件，不能操作文件夹。
    pub async fn copy_file(&self, save_as_file: &str, params: CopyParams) -> Result<(), Box<dyn Error>> {
        // 设置请求头
        let mut headers = HeaderMap::new();
        headers.append("X-Upyun-Copy-Source", HeaderValue::from_str(&format!("/{}/{}", self.bucket, params.source_path)).unwrap());
        headers.append("Content-Length", HeaderValue::from_str("0").unwrap());

        if let Some(x_upyun_metadata_directive) = &params.x_upyun_metadata_directive {
            headers.append("X-Upyun-Metadata-Directive", HeaderValue::from_str(x_upyun_metadata_directive).unwrap());
        }
        if let Some(content_md5) = &params.content_md5 {
            headers.append("Content-MD5", HeaderValue::from_str(content_md5).unwrap());
        }

        let resp = self.request(RequestConfig {
            method: Method::PUT,
            uri: save_as_file.to_string(),
            query: None,
            headers: Some(headers),
            body: None,
        }).await?;

        handle_response(resp).await?;

        Ok(())
    }

    /// 移动文件
    ///
    /// 该操作可以进行文件重命名、文件移动。
    ///
    /// 同一个 `bucket` 下移动文件。并且它只能操作文件，不能操作文件夹。
    pub async fn move_file(&self, save_as_file: &str, params: MoveParams) -> Result<(), Box<dyn Error>> {
        // 设置请求头
        let mut headers = HeaderMap::new();
        headers.append("X-Upyun-Move-Source", HeaderValue::from_str(&format!("/{}/{}", self.bucket, params.source_path)).unwrap());
        headers.append("Content-Length", HeaderValue::from_str("0").unwrap());

        if let Some(x_upyun_metadata_directive) = &params.x_upyun_metadata_directive {
            headers.append("X-Upyun-Metadata-Directive", HeaderValue::from_str(x_upyun_metadata_directive).unwrap());
        }
        if let Some(content_md5) = &params.content_md5 {
            headers.append("Content-MD5", HeaderValue::from_str(content_md5).unwrap());
        }

        let resp = self.request(RequestConfig {
            method: Method::PUT,
            uri: save_as_file.to_string(),
            query: None,
            headers: Some(headers),
            body: None,
        }).await?;

        handle_response(resp).await?;

        Ok(())
    }

    /// 下载文件
    pub async fn download(&self, path_to_file: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let resp = self.request(RequestConfig {
            method: Method::GET,
            uri: path_to_file.to_string(),
            query: None,
            headers: None,
            body: None,
        }).await?;

        let resp = handle_response(resp).await?;

        let bytes = resp.bytes().await.unwrap();
        Ok(bytes.to_vec())
    }

    /// 上传文件
    pub async fn upload(&self, path_to_file: &str, file: Vec<u8>, params: Option<UploadParams>) -> Result<(), Box<dyn Error>> {
        // 设置请求头
        let mut headers = HeaderMap::new();
        headers.append("Date", HeaderValue::from_str(&get_rfc1123_date()).unwrap());
        headers.append("Content-Length", HeaderValue::from_str(&file.len().to_string()).unwrap());

        if let Some(params) = params {
            if let Some(content_md5) = &params.content_md5 {
                headers.append("Content-MD5", HeaderValue::from_str(content_md5).unwrap());
            }
            if let Some(content_type) = &params.content_type {
                headers.append("Content-Type", HeaderValue::from_str(content_type).unwrap());
            }
            if let Some(content_secret) = &params.content_secret {
                headers.append("Content-Secret", HeaderValue::from_str(content_secret).unwrap());
            }
            if let Some(x_upyun_meta_x) = &params.x_upyun_meta_x {
                headers.append("x-upyun-meta-x", HeaderValue::from_str(x_upyun_meta_x).unwrap());
            }
            if let Some(x_upyun_meta_ttl) = params.x_upyun_meta_ttl {
                headers.append("x-upyun-meta-ttl", HeaderValue::from(x_upyun_meta_ttl));
            }
            if let Some(x_gmkerl_thumb) = &params.x_gmkerl_thumb {
                headers.append("x-gmkerl-thumb", HeaderValue::from_str(x_gmkerl_thumb).unwrap());
            }
        }

        let resp = self.request(RequestConfig {
            method: Method::PUT,
            uri: path_to_file.to_string(),
            query: None,
            headers: Some(headers),
            body: Some(Body::from(file)),
        }).await?;

        handle_response(resp).await?;

        Ok(())
    }
}

/// 处理响应状态
async fn handle_response(resp: Response) -> Result<Response, Box<dyn Error>> {
    if resp.status().as_u16() != 200 {
        let error: ApiError = resp.json().await.unwrap();
        return Err(Box::try_from(error).unwrap());
    }

    Ok(resp)
}