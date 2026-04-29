//! Plog CMS Macros
//! 
//! 用于压缩样板代码的宏定义

/// 条件更新字段宏
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
#[macro_export]
macro_rules! api_result {
    ($result:expr, $not_found_msg:expr) => {
        match $result {
            Ok(Some(data)) => $crate::ApiResponse::ok(data),
            Ok(None) => $crate::ApiResponse::err("NOT_FOUND", $not_found_msg),
            Err(e) => $crate::ApiResponse::err("DATABASE_ERROR", e.to_string()),
        }
    };
}

/// API 分页结果处理宏
#[macro_export]
macro_rules! api_paged {
    ($result:expr, $page:expr, $per_page:expr) => {
        match $result {
            Ok((items, total)) => {
                $crate::ApiResponse::ok($crate::PaginatedData::new(
                    items,
                    $page,
                    $per_page,
                    total,
                ))
            }
            Err(e) => $crate::ApiResponse::err("DATABASE_ERROR", e.to_string()),
        }
    };
}

/// API 删除结果处理宏
#[macro_export]
macro_rules! api_delete {
    ($result:expr, $not_found_msg:expr) => {
        match $result {
            Ok(true) => $crate::ApiResponse::ok(()),
            Ok(false) => $crate::ApiResponse::err("NOT_FOUND", $not_found_msg),
            Err(e) => $crate::ApiResponse::err("DATABASE_ERROR", e.to_string()),
        }
    };
}

/// 实现 CrudRepository Trait 的宏
#[macro_export]
macro_rules! impl_crud_repository {
    (
        $repo:ty,
        $entity:ty,
        $active_model:ty,
        $id_type:ty,
        $id_column:path
    ) => {
        // Sealed trait 实现
        impl $crate::private::Sealed for $repo {}
        
        #[async_trait::async_trait]
        impl $crate::CrudRepository for $repo {
            type Entity = $entity;
            type ActiveModel = $active_model;
            type Id = $id_type;

            fn db(&self) -> &std::sync::Arc<sea_orm::DatabaseConnection> {
                &self.db
            }

            fn id_column() -> <Self::Entity as sea_orm::EntityTrait>::Column {
                $id_column
            }

            async fn find_by_id(&self, id: Self::Id) -> Result<Option<<Self::Entity as sea_orm::EntityTrait>::Model>, sea_orm::DbErr> {
                Self::Entity::find()
                    .filter(Self::id_column().eq(id))
                    .one(&**self.db())
                    .await
            }

            async fn find_all(&self) -> Result<Vec<<Self::Entity as sea_orm::EntityTrait>::Model>, sea_orm::DbErr> {
                Self::Entity::find()
                    .all(&**self.db())
                    .await
            }

            async fn create(&self, data: Self::ActiveModel) -> Result<<Self::Entity as sea_orm::EntityTrait>::Model, sea_orm::DbErr> {
                data.insert(&**self.db()).await
            }

            async fn delete(&self, id: Self::Id) -> Result<bool, sea_orm::DbErr> {
                let model = Self::Entity::find()
                    .filter(Self::id_column().eq(id))
                    .one(&**self.db())
                    .await?;
                match model {
                    Some(m) => {
                        m.delete(&**self.db()).await?;
                        Ok(true)
                    }
                    None => Ok(false),
                }
            }

            async fn count(&self) -> Result<u64, sea_orm::DbErr> {
                Self::Entity::find()
                    .count(&**self.db())
                    .await
            }
        }
    };
}

/// 实现 SortableRepository Trait 的宏
#[macro_export]
macro_rules! impl_sortable_repository {
    ($repo:ty, $sort_column:path) => {
        #[async_trait::async_trait]
        impl $crate::SortableRepository for $repo {
            fn default_sort_column() -> <Self::Entity as sea_orm::EntityTrait>::Column {
                $sort_column
            }

            async fn find_all_sorted(&self, desc: bool) -> Result<Vec<<Self::Entity as sea_orm::EntityTrait>::Model>, sea_orm::DbErr> {
                let query = Self::Entity::find();
                let query = if desc {
                    query.order_by_desc(Self::default_sort_column())
                } else {
                    query.order_by_asc(Self::default_sort_column())
                };
                query.all(&**self.db()).await
            }
        }
    };
}
