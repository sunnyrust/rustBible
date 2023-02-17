use crate::{dbstate::DbState,};
pub mod tag;
#[allow(dead_code)]
/// 取得PostgreSQL的conn
fn get_conn<'a>(state: &'a DbState) -> &'a sqlx::PgPool {
    &state.conn
}