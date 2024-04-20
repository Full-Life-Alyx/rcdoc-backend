use std::env::{var, VarError};

use redis::{aio::MultiplexedConnection, AsyncCommands, RedisError};
use sqlx::{types::Uuid, Pool, Postgres};
use thiserror::Error;

pub struct Store {
    pub(crate) pg: Pool<Postgres>,
    pub(crate) redis: MultiplexedConnection,
}

impl Store {
    pub fn new(pg: Pool<Postgres>, redis: MultiplexedConnection) -> Self {
        Self { pg, redis }
    }

    pub async fn get_doc_content(&self, doc_id: Uuid) -> Result<String, RedisError> {
        let mut redis = self.redis.clone();
        let res: Option<String> = redis.get(format!("build:text:{}", doc_id.simple())).await?;
        Ok(res.unwrap())
    }

    fn build_doc() {}
}

pub struct Environment {
    pub pg_url: String,
    pub redis_url: String,
}

#[derive(Error, Debug)]
#[error("Cannot load env {name} due to error {inner}")]
pub struct EnvironmentInitError {
    inner: VarError,
    name: &'static str
}

impl Environment {
    pub fn init() -> Result<Self, EnvironmentInitError> {
        let pg_url = Self::get_var("PG_URL")?;
        let redis_url = Self::get_var("REDIS_URL")?;
        Ok(Self {
            pg_url,
            redis_url
        })
    }

    fn get_var(key: &'static str) -> Result<String, EnvironmentInitError> {
        match var(key) {
            Ok(it) => Ok(it),
            Err(err) => {
                return Err(EnvironmentInitError {
                    inner: err,
                    name: key
                })
            }
        }
    }

}

