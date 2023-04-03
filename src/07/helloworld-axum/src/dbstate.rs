use sqlx::PgPool;
use redis::Client;
pub struct DbState {
    pub conn: PgPool,
    pub redis_conn:Client,
}
