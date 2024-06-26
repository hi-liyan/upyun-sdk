# Upyun sdk for rust

又拍云存储 Rust SDK，基于 [又拍云存储 HTTP REST API 接口](https://help.upyun.com/knowledge-base/rest_api/)  和 [又拍云云处理文档 ](http://docs.upyun.com/cloud/)开发。

## 目录

* [用法](#用法)
  * [添加依赖](#添加依赖)
  * [初始化 Upyun](#初始化-upyun)
  * [又拍云 REST API 接口](#又拍云-rest-api-接口) 
    * [获取空间存储使用量](#获取空间存储使用量)
    * [创建目录](#创建目录)
    * [删除目录或文件](#删除目录或文件)
    * [获取文件信息](#获取文件信息)
    * [获取目录文件列表](#获取目录文件列表)
    * [复制文件](#复制文件)
    * [移动文件](#移动文件)
    * [下载文件](#下载文件)
    * [上传文件](#上传文件)

## 用法

### 添加依赖

```toml
[dependencies]
upyun-sdk = "0.1.3"
```

### 初始化 Upyun

```rust
fn main() {
    let upyun = UpYun::builder()
        .bucket("service_name")             // 必须
        .operator("user")                   // 必须
        .password("123456")                 // 必须
        .timeout(30000)                     // 请求超时时间，可选，默认30s
        .endpoint(Endpoint::Auto)           // 接入点，可选，默认为自动识别接入点
        .danger_accept_invalid_certs(false) // 忽略证书验证，可选，默认为 false
        .build();
}
```

### 又拍云 REST API 接口

#### 获取空间存储使用量

```rust
async fn main() {
    let usage: u64 = upyun.usage().await.unwrap();
}
```

#### 创建目录

```rust
async fn main() { 
    upyun.mkdir("/rust").await.unwrap();
}
```

#### 删除目录或文件

path 可以是目录或文件路径，如果是目录，只允许删除空的目录，否则删除请求会被拒绝。

```rust
async fn main() {
    upyun.rm("/rust").await.unwrap();
}
```

#### 获取文件信息

```rust
async fn main() {
    let info: FileInfo = upyun.file_info("/rust/image.png").await.unwrap();
}
```

#### 获取目录文件列表

```rust
async fn main() {
    // 参数可选
    let params = ListDirParams {
        x_list_iter: None,
        x_list_limit: Some(2),
        x_list_order: Some("desc".to_string())
    };
    let dir_list = upyun.list_dir("/rust", Some(params)).await.unwrap();
}
```

#### 复制文件

同一个 `bucket` 下复制文件。并且它只能操作文件，不能操作文件夹。

```rust
async fn main() {
    let params = CopyParams {
        source_path: "/rust/image.jpg".to_string(),
        x_upyun_metadata_directive: None,
        content_md5: None
    };

    upyun.copy_file("/rust/image_copy.jpg", &params).await.unwrap();
}
```

#### 移动文件

该操作可以实现文件重命名、文件移动。同一个 `bucket` 下移动文件，它只能操作文件，不能操作文件夹。

```rust
async fn main() {
    let params = MoveParams {
        source_path: "/rust/image.jpg".to_string(),
        x_upyun_metadata_directive: None,
        content_md5: None
    };

    upyun.move_file("/rust/1/image.jpg", &params).await.unwrap();
}
```

#### 下载文件

```rust
async fn main() {
    let bytes: Vec<u8> = upyun.download("/rust/image.jpg").await.unwrap();

    let path = Path::new("./image.jpg");
    let mut file = File::create(path).unwrap();
    file.write_all(&bytes).unwrap();
}
```

#### 上传文件

```rust
async fn main() {
    let file: Vec<u8> = read_file_to_vec("./image.jpg").unwrap();
    // 可选参数
    let params = UploadParams {
        content_type: None,
        content_md5: None,
        content_secret: None,
        x_upyun_meta_x: None,
        x_upyun_meta_ttl: None,
        x_gmkerl_thumb: None,
    };

    upyun.upload("/rust/1/image.jpg", file, Some(params)).await.unwrap()
}
```