use crate::upyun::{Endpoint, UpYun};

#[allow(unused)]
fn get_upyun() -> UpYun {
    UpYun::builder()
        .bucket("surcode")
        .operator("rust1")
        .password("ZzSxVESp4OvqvaLEkeIuTOf5tsn2LPsM")
        .timeout(30000)
        .endpoint(Endpoint::EdAuto)
        .build()
}

#[tokio::test]
async fn test_usage() {
    let upyun = get_upyun();

    let usage = upyun.usage().await.unwrap();
    println!("用量：{}", usage);
}

#[tokio::test]
async fn test_mkdir() {
    let upyun = get_upyun();

    let _ = upyun.mkdir("/for-rust/2").await.unwrap();
}