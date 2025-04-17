#[cfg(feature = "advanced_filters")]
use bb8_postgres::tokio_postgres::Row;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[cfg(feature = "redis")]
use redis_macros::{FromRedisValue, ToRedisArgs};
#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "redis", derive(FromRedisValue, ToRedisArgs,))]
pub struct CategoryTree {
    /// Vinted-rs autogenerated id
    pub id: i32,
    /// Father Category id
    pub parent_id: i32,
    /// Child Category id
    pub child_id: i32,
}

#[cfg(feature = "advanced_filters")]
impl From<Row> for CategoryTree {
    fn from(row: Row) -> Self {
        CategoryTree::builder()
            .id(row.get("id"))
            .parent_id(row.get("parent_id"))
            .child_id(row.get("child_id"))
            .build()
    }
}
