use crate::{dbstate::DbState,};
pub mod tag;
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