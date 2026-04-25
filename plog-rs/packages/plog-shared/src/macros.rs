//! Plog CMS Macros
//! 
//! 用于压缩样板代码的宏定义

/// 条件更新字段宏
/// 
/// 用于 SeaORM ActiveModel 的条件更新，替代重复的 if-is-set 模式
/// 
/// # Example
/// 
/// ```ignore
/// apply_if_set!(data, active, title, content, excerpt);
/// ```
#[macro_export]
macro_rules! apply_if_set {
    ($data:expr, $active:expr, $( $field:ident ),* $(,)?) => {
        $(
            if $data.$field.is_set() {
                $active.$field = $data.$field.clone();
            }
        )*
    };
}

/// API 结果处理宏
/// 
/// 用于简化路由处理器中的 match 模式
/// 
/// # Example
/// 
/// ```ignore
/// api_result!(repo.find_by_id(id).await, "Post not found")
/// ```
#[macro_export]
macro_rules! api_result {
    ($result:expr, $not_found_msg:expr) => {
        match $result {
            Ok(Some(data)) => $crate::ApiResponse::success(data),
            Ok(None) => $crate::ApiResponse::error("NOT_FOUND", $not_found_msg),
            Err(e) => $crate::ApiResponse::error("DATABASE_ERROR", e.to_string()),
        }
    };
}

/// API 分页结果处理宏
/// 
/// 用于处理分页查询结果
/// 
/// # Example
/// 
/// ```ignore
/// api_paged!(repo.paginate(page, per_page).await, page, per_page)
/// ```
#[macro_export]
macro_rules! api_paged {
    ($result:expr, $page:expr, $per_page:expr) => {
        match $result {
            Ok((items, total)) => {
                let total_pages = (total + $per_page - 1) / $per_page;
                $crate::ApiResponse::success(serde_json::json!({
                    "items": items,
                    "pagination": {
                        "page": $page,
                        "per_page": $per_page,
                        "total": total,
                        "total_pages": total_pages,
                        "has_more": $page < total_pages
                    }
                }))
            }
            Err(e) => $crate::ApiResponse::error("DATABASE_ERROR", e.to_string()),
        }
    };
}

/// API 简单结果处理宏（用于 create/update/delete）
/// 
/// # Example
/// 
/// ```ignore
/// api_simple!(repo.create(data).await, "Failed to create")
/// ```
#[macro_export]
macro_rules! api_simple {
    ($result:expr, $error_msg:expr) => {
        match $result {
            Ok(data) => $crate::ApiResponse::success(data),
            Err(e) => $crate::ApiResponse::error("OPERATION_FAILED", format!("{}: {}", $error_msg, e)),
        }
    };
}

/// API 删除结果处理宏
/// 
/// # Example
/// 
/// ```ignore
/// api_delete!(repo.delete(id).await, "Item not found")
/// ```
#[macro_export]
macro_rules! api_delete {
    ($result:expr, $not_found_msg:expr) => {
        match $result {
            Ok(true) => $crate::ApiResponse::success(()),
            Ok(false) => $crate::ApiResponse::error("NOT_FOUND", $not_found_msg),
            Err(e) => $crate::ApiResponse::error("DATABASE_ERROR", e.to_string()),
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_apply_if_set() {
        use sea_orm::Set;

        struct TestData {
            title: Set<String>,
            content: Set<String>,
        }

        let data = TestData {
            title: Set("test".to_string()),
            content: Set("content".to_string()),
        };

        assert!(data.title.is_set());
        assert!(data.content.is_set());
    }
}
