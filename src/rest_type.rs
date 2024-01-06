use serde::{Serialize, Deserialize};

/// 获取文件信息的响应信息
#[derive(Debug, Clone)]
pub struct FileInfo {
    /// 文件为 `file`，文件夹为 `folder`
    pub x_upyun_file_type: String,
    /// 文件大小（字节）
    pub x_upyun_file_size: Option<u64>,
    /// 文件创建时间
    pub x_upyun_file_date: String,
    /// 文件的 MD5 值
    pub content_md5: Option<String>,
}

/// 获取目录文件列表的参数
pub struct ListDirParams {
    /// 分页开始位置，通过 `x-upyun-list-iter` 响应头返回，所以第一次请求不需要填写
    pub x_list_iter: Option<String>,
    /// 获取的文件数量，默认 100，最大 10000
    pub x_list_limit: Option<u64>,
    /// `asc` 或 `desc`，按文件名升序或降序排列。默认 `asc``
    pub x_list_order: Option<String>
}

/// 目录文件列表的元素
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirElem {
    /// 目录/文件的类型。如果是目录，则为 `folder`，如果是文件，则为文件的类型，
    pub r#type: String,
    /// 文件大小（字节）
    pub length: u64,
    /// 文件/目录名
    pub name: String,
    /// 最后修改时间
    pub last_modified: u64
}

/// 获取目录文件列表的响应信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDir {
    /// 各个文件/目录的信息
    pub files: Vec<DirElem>,
    /// 返回下一次分页开始位置。它由一串 Base64 编码的随机数组成，当它是 g2gCZAAEbmV4dGQAA2VvZg 时，表示最后一个分页。
    pub iter: String
}

/// 复制文件的参数
pub struct CopyParams {
    /// 源文件地址（同 bucket），格式 `/<source_to_file>`
    pub source_path: String,
    /// 处理源文件的元信息，默认 `copy`(复制)，详见 [Metadata](https://help.upyun.com/knowledge-base/rest_api/#metadata)
    pub x_upyun_metadata_directive: Option<String>,
    /// 请求的 MD5 值，需要服务端进行 MD5 校验请填写，等效于[签名认证](https://help.upyun.com/knowledge-base/object_storage_authorization/#sign_auth)中的 Content-MD5
    pub content_md5: Option<String>
}

/// 移动文件的参数
pub struct MoveParams {
    /// 需要移动的文件地址（同 bucket），格式 `/<source_to_file>`
    pub source_path: String,
    /// 处理源文件的元信息，默认 `copy`(复制)，详见 [Metadata](https://help.upyun.com/knowledge-base/rest_api/#metadata)
    pub x_upyun_metadata_directive: Option<String>,
    /// 请求的 MD5 值，需要服务端进行 MD5 校验请填写，等效于[签名认证](https://help.upyun.com/knowledge-base/object_storage_authorization/#sign_auth)中的 Content-MD5
    pub content_md5: Option<String>
}

/// 上传文件的参数
pub struct UploadParams {
    /// 文件类型，默认使用文件扩展名作为文件类型
    pub content_type: Option<String>,
    /// 上传文件的 MD5 值，如果请求中文件太大计算 MD5 不方便，可以为空
    pub content_md5: Option<String>,
    /// 文件密钥，用于保护文件，防止文件被直接访问，见 [Content-Secret 参数说明](https://help.upyun.com/knowledge-base/rest_api/#Content-Secret)
    pub content_secret: Option<String>,
    /// 文件元信息，见 [Metadata](https://help.upyun.com/knowledge-base/rest_api/#metadata)
    pub x_upyun_meta_x: Option<String>,
    /// 文件元信息, 指定文件的生存时间，单位天，最大支持180天，见 [Metadata](https://help.upyun.com/knowledge-base/rest_api/#metadata)
    pub x_upyun_meta_ttl: Option<u64>,
    /// 图片预处理参数，见[上传预处理（同步）](https://help.upyun.com/knowledge-base/image/#sync_upload_process)
    pub x_gmkerl_thumb: Option<String>

}