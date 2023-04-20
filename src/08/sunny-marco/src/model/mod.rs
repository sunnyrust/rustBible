use crate::{dbstate::DbState,};
pub mod tag;
pub mod emotion;
use serde::{Deserialize, Serialize};
#[allow(dead_code)]
/// 取得PostgreSQL的conn
fn get_conn<'a>(state: &'a DbState) -> &'a sqlx::PgPool {
    &state.conn
}


#[allow(dead_code)]
/// 取得redis的conn
fn get_redis_conn<'a>(state: &'a DbState) -> &'a redis::Client {
    &state.redis_conn
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct CountModel {
    pub count:i64,
}