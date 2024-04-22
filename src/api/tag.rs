use futures::{
    future::{self, join_all}, stream, StreamExt
};
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

use crate::{
    schema::{TagCategory, TagPair},
    store::Store,
};

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
    #[oai(path = "/query/id", method = "get")]
    async fn query_id(&self, names: Query<Vec<String>>) -> Json<Vec<TagPair>> {
        let res = stream::iter(names.0)
            .filter_map(|name| self.store.tag_pair_from_name(name))
            .collect::<Vec<_>>()
            .await;

        Json(res.into())
    }

    #[oai(path = "/category", method = "get")]
    async fn get_category(&self, id: Query<i32>) -> Json<Option<TagCategory>> {
        let json = self.store.category_id(id.0).await;
        Json(json.into())
    }
}

impl Store {
    async fn tag_pair_from_name(&self, name: String) -> Option<TagPair> {
        let cache: Option<i32> = self
            .redis
            .clone()
            .get(format!("tag:pair:name:{}", &name))
            .await
            .unwrap();
        if let Some(it) = cache {
            return Some(TagPair { id: it, name });
        };

        let res = sqlx::query_as!(TagPair, "SELECT id, name FROM tag WHERE name = $1", name)
            .fetch_optional(&self.pg)
            .await
            .unwrap();
        res
    }

    async fn category_id(&self, id: i32) -> Option<TagCategory> {
        let res = sqlx::query_as!(TagCategory, "SELECT * FROM tag_category where id = $1", id)
            .fetch_optional(&self.pg)
            .await
            .unwrap();
        res
    }
}
