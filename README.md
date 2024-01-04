# Upyun sdk for rust

又拍云存储 Rust SDK，基于 [又拍云存储 HTTP REST API 接口](http://docs.upyun.com/api/rest_api/)  和 [又拍云云处理文档 ](http://docs.upyun.com/cloud/)开发。

## 引入依赖

```toml
upyun-sdk = { github = "" }
```

## 目录

* [用法](#用法)
  * [初始化 Upyun](#初始化-upyun)
  * [又拍云 REST API 接口](#又拍云-rest-api-接口) 
    * [获取空间存储使用量](#获取空间存储使用量)
    * [创建目录](#创建目录)

## 用法

### 初始化 Upyun

```rust
fn main() {
    let upyun = UpYun::builder()
        .bucket("service_name")     // 必须
        .operator("user")           // 必须
        .password("123456")         // 必须
        .timeout(30000)             // 请求超时时间，可选，默认30s
        .endpoint(Endpoint::EdAuto) // 接入点，可选，默认为自动识别接入点
        .build();
}
```

### 又拍云 REST API 接口

#### 获取空间存储使用量

```rust
async fn main() {
    let result: Result<u64, Box<dyn StdError>> = upyun.useage().await;
}
```

#### 创建目录

```rust
async fn main() {
    let _ = upyun.mkdir("/images").await;
}
```