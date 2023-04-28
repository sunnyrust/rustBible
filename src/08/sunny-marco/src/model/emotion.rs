use redis::{Commands, };
use serde::{Deserialize, Serialize};
use super::{get_conn, get_redis_conn};
use crate::{AppError,dbstate::DbState,Result};
use sunny_derive_trait::*;
use sunny_derive::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow, PgCurdStruct)]
#[TableName = "storyboard.emotion"]
#[CacheName = "all_emotion"]
pub struct EmotionModel {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub unicode: String,
}
impl Default for EmotionModel {
    fn default() -> Self {
        EmotionModel {
            id: 0,
            name: String::new(),
            code: String::new(),
            unicode: String::new(),
        }
    }
}
fn get_cache_name()->String{
    let g=EmotionModel::default();
    g.get_cache_name().to_string()
}
#[allow(dead_code)]
pub async fn get_all<'a,'b>(state: &'a DbState,sql:&'b String) -> Result<Vec<EmotionModel>> {
    // 操作redis
    let client=get_redis_conn(&state);
    let mut redis_conn = client.get_connection().expect("redis connect error");
    let mut b_have_key=false;  //是否有缓存
    let rv:redis::RedisResult<String> = redis_conn.get(get_cache_name());//读取缓存
    let result =match rv {
        Ok(data) => {
            b_have_key=true;
            data
        },
        Err(_err) => {
            "".to_string()
        }
    };
    #[allow(unused_assignments)]
    let mut vec_emotion:Vec<EmotionModel>=vec![];
    if !b_have_key{
        let pool = get_conn(&state);
        let rows = sqlx::query_as::<_, EmotionModel>(&sql)
            .fetch_all(pool)
            .await
            .unwrap();
        vec_emotion=rows.clone();
        // 插入redis
        let strm:String=serde_json::to_string(&rows).unwrap();
        let _:()=redis_conn.set(get_cache_name(),  strm).unwrap();
    }else{
        vec_emotion=serde_json::from_str(&result).unwrap();
    }
    Ok(vec_emotion)
}


/// 判断名字是不是在数据库中存在
async fn have_code<'a,'b,'c,'d>(state: &'a DbState,table_name:&'b String,code:&'c String,unicode:&'d String) ->Result<bool>{
    let pool = get_conn(&state);
    let sql=format!("SELECT count(1) as count from {} where code ='{}' or unicode='{}'",table_name,code,unicode);
    let mut b=false;  
    let parent_rows = sqlx::query_as::<_, super::CountModel>(&sql)
        .fetch_one(pool)
        .await;
     match parent_rows {
             Ok(result) => {
                 if result.count!=0{
                     b=true;
                 }
             },
             Err(err) => {
                 tracing::error!("----{}---",err);
             }
     }
     Ok(b)
 }
#[allow(dead_code)]
pub async fn insert_one<'a,'b,'c,'d,'e>(state: &'a DbState,sql:&'b String,table_name:&'c String,code:&'d  String,unicode:&'e String) -> Result<String> {
    let pool = get_conn(&state);
    let b=have_code(state,table_name,code,unicode).await.unwrap();
    if b{
        let code = AppError::from_err(format!("code:{} or unicode:{}已经存在，不能添加",code,unicode).into(),crate::AppErrorType::Database);
        return Err(code);
    }
    let res=sqlx::query(&sql)
    .execute(pool)
    .await;
    match res {
        Ok(result) => {
            let _rows=result.rows_affected();
            if _rows==0{
                let code = AppError::from_err(format!("Insert Error").into(),crate::AppErrorType::Database);
                return Err(code);
            }
        },
        Err(err) => {
            let code = AppError::from_err(err.into(),crate::AppErrorType::Database);
            return Err(code);
        }
    }
    // 操作redis 清除缓存
    let client=get_redis_conn(&state);
    let mut redis_conn = client.get_connection().expect("redis connect error");
    redis::cmd("DEL").arg(get_cache_name()).execute(&mut redis_conn);
    Ok("ok".to_string())
}

#[allow(dead_code)]
pub async fn delete<'a,'b>(state: &'a DbState,sql:&'b String) -> Result<String> {
    let pool = get_conn(&state);
    //let sql=format!("Delete from {} where id ={}",get_table_name(),id);
    let res=sqlx::query(&sql)
    .execute(pool)
    .await;
    match res {
        Ok(result) => {
            let _rows=result.rows_affected();
            if _rows==0{
                let code = AppError::from_err(format!("库里不存在这个id，无法删除").into(),crate::AppErrorType::Database);
                return Err(code);
            }
        },
        Err(err) => {
            let code = AppError::from_err(err.into(),crate::AppErrorType::Database);
            return Err(code);
        }
    }
    // 操作redis 清除缓存
    let client=get_redis_conn(&state);
    let mut redis_conn = client.get_connection().expect("redis connect error");
    redis::cmd("DEL").arg(get_cache_name()).execute(&mut redis_conn);
    Ok("ok".to_string())
}


/// 程序获取一条数据的操作
#[allow(dead_code)]
pub async fn get_one_by_id<'a,'b>(state: &'a DbState,sql:&'b String) -> std::result::Result<EmotionModel,String>  {
    let pool = get_conn(&state);
    let result = sqlx::query_as::<_, EmotionModel>(&sql)
                    .fetch_one(pool)
                    .await
                    .map_err(|e| format!("Error fetching from the database: {}", e))?;
    Ok(result)
}

#[allow(dead_code)]
pub async fn update<'a,'b>(state: &'a DbState,sql:&'b String) -> Result<String> {
    let pool = get_conn(&state);
    // let sql=format!("UPDATE {} SET name = '{}',pid = {} WHERE id = {};",get_table_name(),m.name,m.pid,m.id);
    let res=sqlx::query(&sql)
    .execute(pool)
    .await;
    match res {
        Ok(result) => {
            let _rows=result.rows_affected();
            if _rows==0{
                let code = AppError::from_err(format!("库里不存在这个id，无法删除").into(),crate::AppErrorType::Database);
                return Err(code);
            }
        },
        Err(err) => {
            let code = AppError::from_err(err.into(),crate::AppErrorType::Database);
            return Err(code);
        }
    }
    // 操作redis 清除缓存
    let client=get_redis_conn(&state);
    let mut redis_conn = client.get_connection().expect("redis connect error");
    redis::cmd("DEL").arg(get_cache_name()).execute(&mut redis_conn);
    Ok("ok".to_string())
}