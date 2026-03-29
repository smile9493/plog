//! Media 单元测试

use plog_media::*;

/// 测试图片类型检测
#[test]
fn test_is_image() {
    use entity::image_types;

    assert!(entity::is_image(image_types::JPEG));
    assert!(entity::is_image(image_types::PNG));
    assert!(entity::is_image(image_types::GIF));
    assert!(entity::is_image(image_types::WEBP));
    assert!(!entity::is_image("application/pdf"));
}

/// 测试文档类型检测
#[test]
fn test_is_document() {
    use entity::document_types;

    assert!(entity::is_document(document_types::PDF));
    assert!(entity::is_document(document_types::DOC));
    assert!(entity::is_document(document_types::DOCX));
    assert!(entity::is_document(document_types::XLS));
    assert!(entity::is_document(document_types::XLSX));
    assert!(!entity::is_document("image/jpeg"));
}

/// 测试获取文件扩展名
#[test]
fn test_get_extension() {
    assert_eq!(entity::get_extension("photo.jpg"), "jpg");
    assert_eq!(entity::get_extension("document.PDF"), "pdf");
    assert_eq!(entity::get_extension("file.name.with.dots.png"), "png");
    assert_eq!(entity::get_extension("noextension"), "");
}

/// 测试生成唯一文件名
#[test]
fn test_generate_unique_filename() {
    let name1 = entity::generate_unique_filename("photo.jpg");
    let name2 = entity::generate_unique_filename("photo.jpg");

    // 应该生成不同的文件名
    assert_ne!(name1, name2);

    // 应该保留扩展名
    assert!(name1.ends_with(".jpg"));
    assert!(name2.ends_with(".jpg"));

    // 无扩展名的情况
    let name3 = entity::generate_unique_filename("noext");
    assert!(!name3.contains('.'));
}

/// 测试默认允许的扩展名
#[test]
fn test_default_allowed_extensions() {
    let exts = entity::default_allowed_extensions();

    assert!(exts.contains(&"jpg"));
    assert!(exts.contains(&"png"));
    assert!(exts.contains(&"pdf"));
    assert!(exts.contains(&"zip"));
    assert!(exts.contains(&"mp4"));
}

/// 测试本地存储 URL 生成
#[test]
fn test_local_storage_url() {
    use plog_media::LocalStorage;

    let storage = LocalStorage::new("/data/uploads", "https://example.com/files");

    assert_eq!(
        storage.url("uploads/2024/photo.jpg"),
        "https://example.com/files/uploads/2024/photo.jpg"
    );

    // 测试尾部斜杠处理
    let storage2 = LocalStorage::new("/data/uploads", "https://example.com/files/");
    assert_eq!(
        storage2.url("uploads/photo.jpg"),
        "https://example.com/files/uploads/photo.jpg"
    );
}
