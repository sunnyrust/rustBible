pub mod emotion;
pub mod gesture;
pub mod iot;
pub mod iot_behavior;
pub mod tag;

use crate::{dbstate::DbState, AppError, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
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
    pub count: i64,
}

use async_trait::async_trait;
#[async_trait]
pub trait InterfaceDB<T>
where
    T: Serialize + DeserializeOwned + Unpin + Send + Sync,
{
    async fn delete<'a, 'b>(state: &'a DbState, sql: &'b String) -> Result<String> {
        let pool = get_conn(&state);
        //let sql=format!("Delete from {} where id ={}",get_table_name(),id);
        let res = sqlx::query(&sql).execute(pool).await;
        match res {
            Ok(result) => {
                let _rows = result.rows_affected();
                if _rows == 0 {
                    let code = AppError::from_err(
                        format!("库里不存在这个id，无法删除").into(),
                        crate::AppErrorType::Database,
                    );
                    return Err(code);
                }
            }
            Err(err) => {
                let code = AppError::from_err(err.into(), crate::AppErrorType::Database);
                return Err(code);
            }
        }
        // 操作redis 清除缓存
        Self::remove_redis_cache(&state);

        Ok("ok".to_string())
    }
    
    /// 取得cache名字
    fn get_cache_name() -> String {
        // let g = Model::default();
        // g.get_cache_name().to_string()
        "all_iot".to_string()
        
    }
    /// 程序获取一条数据的操作
    // #[allow(dead_code)]
    // async fn get_one_by_id<'a, 'b,'T>(
    //     state: &'a DbState,
    //     sql: &'b String,
    // ) -> std::result::Result<T, String> {
    //     let pool = get_conn(&state);
    //     let result = sqlx::query_as::<_, T>(&sql)
    //         .fetch_one(pool)
    //         .await
    //         .map_err(|e| format!("Error fetching from the database: {}", e))?;
    //     Ok(result)
    // }
    /// 删除redis缓存中的数据
    fn remove_redis_cache(state: &DbState) {
        // 操作redis 清除缓存
        let client = get_redis_conn(&state);
        let mut redis_conn = client.get_connection().expect("redis connect error");
        redis::cmd("DEL")
            .arg(Self::get_cache_name())
            .execute(&mut redis_conn);
    }
    async fn update<'a, 'b>(state: &'a DbState, sql: &'b String) -> Result<String> {
        let pool = get_conn(&state);
        let res = sqlx::query(&sql).execute(pool).await;
        match res {
            Ok(result) => {
                let _rows = result.rows_affected();
                if _rows == 0 {
                    let code = AppError::from_err(
                        format!("库里不存在这个id，无法删除").into(),
                        crate::AppErrorType::Database,
                    );
                    return Err(code);
                }
            }
            Err(err) => {
                let code = AppError::from_err(err.into(), crate::AppErrorType::Database);
                return Err(code);
            }
        }
        Self::remove_redis_cache(&state);
        Ok("ok".to_string())
    }
}
