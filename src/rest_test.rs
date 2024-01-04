use crate::upyun::{UpYun, UpYunConfig};

#[tokio::test]
async fn test_usage() {
    let config = UpYunConfig {
        bucket: "surcode".to_string(),
        operator: "rust1".to_string(),
        password: "ZzSxVESp4OvqvaLEkeIuTOf5tsn2LPsM".to_string(),
        timeout: None,
    };
    let upyun = UpYun::new(config);

    let result = upyun.usage().await;
    println!("执行了吗？");

    match result {
        Ok(usage) => {
            println!("用量：{}", usage)
        }
        Err(e) => {
            println!("错误：{:?}", e);
        }
    }
}

#[tokio::test]
async fn test_mkdir() {
    let config = UpYunConfig {
        bucket: "surcode".to_string(),
        operator: "rust1".to_string(),
        password: "ZzSxVESp4OvqvaLEkeIuTOf5tsn2LPsM".to_string(),
        timeout: None,
    };
    let upyun = UpYun::new(config);

    let result = upyun.mkdir("for-rust".to_string()).await;
    println!("执行了吗？");

    if let Err(e) = result {
        println!("错误：{:?}", e);
    }
}