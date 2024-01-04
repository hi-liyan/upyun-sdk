# Upyun sdk for rust

又拍云存储 Rust SDK，基于 [又拍云存储 HTTP REST API 接口](http://docs.upyun.com/api/rest_api/)  和 [又拍云云处理文档 ](http://docs.upyun.com/cloud/)开发。

## 目录

* [用法](#用法)
  * [添加依赖](#添加依赖)
  * [初始化 Upyun](#初始化-upyun)
  * [又拍云 REST API 接口](#又拍云-rest-api-接口) 
    * [获取空间存储使用量](#获取空间存储使用量)
    * [创建目录](#创建目录)
    * [删除目录](#删除目录)

## 用法

### 添加依赖

```toml
[dependencies]
upyun-sdk = "0.1.1"
```

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
    let usage: u64 = upyun.usage().await.unwrap();
}
```

#### 创建目录

```rust
async fn main() { 
    upyun.mkdir("/rust").await.unwrap();
}
```

#### 删除目录

```rust
async fn main() {
    upyun.rmdir("/rust").await.unwrap();
}
```