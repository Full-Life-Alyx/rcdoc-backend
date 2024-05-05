use sqlx::{query, query_as};
use uuid::Uuid;

use crate::store::Store;

pub mod middleware;
pub mod ms_api;

impl Store {
    pub async fn create_account(&self, nickname: &str, identifier: &str) -> Uuid {
        struct UuidContainer {
            id: Uuid
        }
        let res = query_as!(UuidContainer,
            "INSERT INTO account (nickname, identifier, is_admin)
            VALUES ($1, $2, false)
            RETURNING id;", nickname, identifier
        ).fetch_one(&self.pg).await.unwrap();
        res.id
    }
    pub async fn ms_auth_add(&self, user: Uuid, sub: &str) {
        query!(
            "INSERT INTO ms_auth (account_id, subject) VALUES ($1, $2)",
            user, sub
        ).execute(&self.pg).await.unwrap();
    }
}

