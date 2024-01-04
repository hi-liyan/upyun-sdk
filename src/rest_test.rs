use crate::common::utils::test::get_upyun;

/// 测试获取服务使用量
#[tokio::test]
async fn test_usage() {
    let upyun = get_upyun();

    let usage = upyun.usage().await.unwrap();
    println!("用量：{}", usage);
}

/// 测试创建目录
#[tokio::test]
async fn test_mkdir() {
    let upyun = get_upyun();

    let _ = upyun.mkdir("/demo").await.unwrap();
}