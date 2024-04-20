use poem_openapi::{
    param::Query,
    payload::{Json, PlainText},
    types::ToJSON,
    ApiResponse, Object, OpenApi,
};
use redis::{aio::ConnectionLike, AsyncCommands};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;

use crate::{schema::TagCategory, store::Store};

pub struct TagService {
    store: Store,
}

impl TagService {
    pub fn new(store: Store) -> Self {
        Self { store }
    }
}

#[OpenApi]
impl TagService {
    #[oai(path = "/category", method = "get")]
    async fn get_category(&self, id: Query<i32>) -> Json<TagCategory> {
        let json = self.store.get_category_id(id.0).await;
        Json(json.into())
    }
}

impl Store {
    async fn get_category_id(&self, id: i32) -> TagCategory {
        let res = sqlx::query_as!(TagCategory, "SELECT * FROM tag_category where id = $1", id)
            .fetch_one(&self.pg)
            .await.expect("I dunno what to do with this");

        res
    }

}
