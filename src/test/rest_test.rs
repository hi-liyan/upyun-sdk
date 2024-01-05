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
    upyun.mkdir("/rust/1").await.unwrap();
}

/// 测试删除目录
#[tokio::test]
async fn test_rmdir() {
    let upyun = get_upyun();
    upyun.rmdir("/rust/1").await.unwrap();
}

/// 测试删除文件
#[tokio::test]
async fn test_rm() {
    let upyun = get_upyun();
    upyun.rmdir("/rust/image.jpg").await.unwrap();
}