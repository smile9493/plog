//! 媒体实体定义

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 媒体文件模型
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "media")]
pub struct Model {
    /// 主键
    #[sea_orm(primary_key)]
    pub id: i32,

    /// 文件名
    pub filename: String,

    /// 原始文件名
    pub original_name: String,

    /// 文件路径
    pub filepath: String,

    /// 文件 URL
    pub url: String,

    /// 文件大小 (bytes)
    pub filesize: i64,

    /// MIME 类型
    pub mimetype: String,

    /// 文件扩展名
    pub extension: String,

    /// 宽度 (图片)
    #[sea_orm(nullable)]
    pub width: Option<i32>,

    /// 高度 (图片)
    #[sea_orm(nullable)]
    pub height: Option<i32>,

    /// 上传用户 ID
    #[sea_orm(nullable)]
    pub user_id: Option<i32>,

    /// 存储驱动
    pub storage_driver: String,

    /// 创建时间
    pub created_at: i64,

    /// 更新时间
    pub updated_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

/// 支持的图片类型
pub mod image_types {
    pub const JPEG: &str = "image/jpeg";
    pub const PNG: &str = "image/png";
    pub const GIF: &str = "image/gif";
    pub const WEBP: &str = "image/webp";
    pub const SVG: &str = "image/svg+xml";
}

/// 支持的文档类型
pub mod document_types {
    pub const PDF: &str = "application/pdf";
    pub const DOC: &str = "application/msword";
    pub const DOCX: &str =
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document";
    pub const XLS: &str = "application/vnd.ms-excel";
    pub const XLSX: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";
}

/// 默认允许的文件扩展名
pub fn default_allowed_extensions() -> Vec<&'static str> {
    vec![
        "jpg", "jpeg", "png", "gif", "webp", "svg", "pdf", "doc", "docx", "xls", "xlsx", "zip",
        "rar", "7z", "mp3", "mp4", "avi", "mov",
    ]
}

/// 检查是否为图片
pub fn is_image(mimetype: &str) -> bool {
    mimetype.starts_with("image/")
}

/// 检查是否为文档
pub fn is_document(mimetype: &str) -> bool {
    matches!(
        mimetype,
        document_types::PDF
            | document_types::DOC
            | document_types::DOCX
            | document_types::XLS
            | document_types::XLSX
    )
}

/// 获取文件扩展名
pub fn get_extension(filename: &str) -> String {
    if let Some(pos) = filename.rfind('.') {
        filename[pos + 1..].to_lowercase()
    } else {
        String::new()
    }
}

/// 生成唯一文件名
pub fn generate_unique_filename(original: &str) -> String {
    let ext = get_extension(original);
    let uuid = uuid::Uuid::new_v4().to_string();
    if ext.is_empty() {
        uuid
    } else {
        format!("{}.{}", uuid, ext)
    }
}
