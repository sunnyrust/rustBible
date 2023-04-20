use redis::{Commands, };
use serde::{Deserialize, Serialize};
use super::{get_conn, get_redis_conn};
use crate::{AppError,dbstate::DbState,Result};
use sunny_derive_trait::*;
use sunny_derive::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow, PgCurdStruct)]
#[TableName = "storyboard.emotion"]
pub struct EmotionModel {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub unicode: String,
}

#[allow(dead_code)]
pub async fn get_all<'a,'b>(state: &'a DbState,sql:&'b String) -> Result<Vec<EmotionModel>> {
    // 操作redis
    let client=get_redis_conn(&state);
    let mut redis_conn = client.get_connection().expect("redis connect error");
    let mut b_have_key=false;  //是否有缓存
    let rv:redis::RedisResult<String> = redis_conn.get("all_emotion");//读取缓存
    let result =match rv {
        Ok(data) => {
            b_have_key=true;
            data
        },
        Err(_err) => {
            "".to_string()
        }
    };
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
        let _:()=redis_conn.set("all_emotion",  strm).unwrap();
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
    redis::cmd("DEL").arg("all_emotion").execute(&mut redis_conn);
    Ok("ok".to_string())
}