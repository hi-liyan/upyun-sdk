use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

use crate::{common::utils::test::get_upyun, rest_type::ListDirParams};
use crate::rest_type::{CopyParams, MoveParams, UploadParams};

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

/// 测试删除目录或文件
#[tokio::test]
async fn test_rmdir() {
    let upyun = get_upyun();
    upyun.rm("/rust/微信图片_20231231164013.jpg").await.unwrap();
}

/// 测试获取文件信息
#[tokio::test]
async fn test_file_info() {
    let upyun = get_upyun();
    let fileinfo = upyun.file_info("/rust/微信图片_20231231164013.jpg").await.unwrap();
    println!("文件信息：{:#?}", fileinfo);
}

/// 测试获取目录文件列表
#[tokio::test]
async fn test_list_dir() {
    let upyun = get_upyun();
    
    let params = ListDirParams {
        x_list_iter: None,
        x_list_limit: Some(2),
        x_list_order: Some("desc".to_string())
    };
    let dir_list = upyun.list_dir("/rust", Some(params)).await.unwrap();
    println!("目录列表：{:#?}", dir_list);
}

/// 测试复制文件
#[tokio::test]
async fn test_copy() {
    let upyun = get_upyun();
    let params = CopyParams {
        source_path: "/rust/1/image.jpg".to_string(),
        x_upyun_metadata_directive: None,
        content_md5: None
    };

    upyun.copy_file("/rust/微信图片_20231231164013.jpg", params).await.unwrap();
}

/// 测试移动文件
#[tokio::test]
async fn test_move() {
    let upyun = get_upyun();
    let params = MoveParams {
        source_path: "/rust/微信图片_20231231164013.jpg".to_string(),
        x_upyun_metadata_directive: None,
        content_md5: None
    };

    upyun.move_file("/rust/1/image1.jpg", params).await.unwrap();
}

/// 测试下载文件
#[tokio::test]
async fn test_download() {
    let upyun = get_upyun();
    let bytes = upyun.download("/rust/1/image1.jpg").await.unwrap();

    let path = Path::new("./image1.jpg");
    let mut file = File::create(path).unwrap();
    file.write_all(&bytes).unwrap();
}

/// 测试上传文件
#[tokio::test]
async fn test_upload() {
    let upyun = get_upyun();

    let file = read_file_to_vec("./cred.txt").unwrap();

    let params = UploadParams {
        content_type: None,
        content_md5: None,
        content_secret: None,
        x_upyun_meta_x: None,
        x_upyun_meta_ttl: None,
        x_gmkerl_thumb: None,
    };

    upyun.upload("/rust/1/cred.txt", file, Some(params)).await.unwrap()
}

/// 读取文件到字节流
fn read_file_to_vec(file_path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();

    // 读取文件内容到缓冲区
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}