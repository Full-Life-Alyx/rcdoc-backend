use poem_openapi::{types::ParseFromJSON, Object};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::*, types::uuid::Timestamp};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(FromRow, Deserialize, Serialize, Object)]
pub struct TagCategory {
    pub id: i32,
    pub name: String,
    pub created_by: Uuid,
    pub timestamp: OffsetDateTime
}


