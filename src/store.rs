use std::{
    env::{var, VarError},
    sync::Arc,
};

use redis::{aio::MultiplexedConnection, AsyncCommands, RedisError};
use sqlx::{types::Uuid, Pool, Postgres};
use thiserror::Error;

pub struct Store {
    pub pg: Pool<Postgres>,
    pub redis: MultiplexedConnection,
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
}

pub struct Environment {
    pub pg_url: Arc<str>,
    pub redis_url: Arc<str>,
    pub ms_id: Arc<str>,
    pub ms_secret: Arc<str>,
    pub ms_tenant: Arc<str>,
}

#[derive(Error, Debug)]
#[error("Cannot load env {name} due to error {inner}")]
pub struct EnvironmentInitError {
    inner: VarError,
    name: &'static str,
}

impl Environment {
    pub fn init() -> Result<Self, EnvironmentInitError> {
        let pg_url = Self::get_var("PG_URL")?;
        let redis_url = Self::get_var("REDIS_URL")?;
        let ms_id = Self::get_var("MICROSOFT_CLIENT_ID")?;
        let ms_secret = Self::get_var("MICROSOFT_CLIENT_SECRET")?;
        let ms_tenant = Self::get_var("MICROSOFT_CLIENT_TENANT")?;
        Ok(Self {
            pg_url,
            redis_url,
            ms_id,
            ms_secret,
            ms_tenant,
        })
    }

    fn get_var(key: &'static str) -> Result<Arc<str>, EnvironmentInitError> {
        match var(key) {
            Ok(it) => Ok(it.into()),
            Err(err) => {
                return Err(EnvironmentInitError {
                    inner: err,
                    name: key,
                })
            }
        }
    }
}
