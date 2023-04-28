use super::render;
use crate::{dbstate::*, model::emotion::*, model::*, util::types::*};
use askama::Template;
use axum::{routing::{get,post}, Extension, Form, Json, Router,extract::Path,
        http::{header::HeaderName, HeaderMap, HeaderValue, StatusCode},
};
use serde::Deserialize;
use std::sync::Arc;
use sunny_derive_trait::PgCurdStruct;
use tower_http::trace::TraceLayer;
pub(crate) fn index_router() -> Router {
    Router::new()
        .route("/test", get(test))
        .route("/list", get(list))
        .route("/insert", get(add).post(do_insert))
        .route("/del/:id", get(do_del))
        .route("/edit/:id", get(edit))
        .route("/doedit", post(do_edit))
        .route("/get", get(get_emotion))
        .layer(TraceLayer::new_for_http())
}

async fn test(Extension(state): Extension<Arc<DbState>>) -> String {
    let emo = emotion::EmotionModel {
        id: 0,
        name: "name".to_string(),
        code: "10086".to_string(),
        unicode: "1F001".to_string(),
    };
    // emo.print();
    eprintln!("Get Table Name:{}", emo.get_table_name());
    let emo_model = emotion::get_all(&state, &emo.select()).await.unwrap();
    let result = Json(serde_json::json!({ "result": emo_model }));
    eprintln!("SQL语句：{}", emo.select());

    eprintln!("SQL语句：{}", emo.insert());
    eprintln!("SQL语句：{}", emo.delete(1));
    eprintln!("SQL语句：{}", emo.update(1));
    String::from(result.to_string())
}
#[derive(Template)]
#[template(path = "emotion/list.html", ext = "html", escape = "none")]
pub struct ListForm {
    pub lists: Vec<EmotionModel>,
}
async fn list(Extension(state): Extension<Arc<DbState>>) -> Result<HtmlResponse> {
    let handler_name = "list";
    let emo = emotion::EmotionModel {
        id: 0,
        name: "name".to_string(),
        code: "10086".to_string(),
        unicode: "1F001".to_string(),
    };
    let emo_model = emotion::get_all(&state, &emo.select()).await.unwrap();
    let tpl = ListForm { lists: emo_model };
    render(tpl, handler_name)
}

#[derive(Template)]
#[template(path = "emotion/add.html", ext = "html", escape = "none")]
pub struct NullForm {}
async fn add() -> Result<HtmlResponse> {
    let handler_name = "add";
    let tpl = NullForm {};
    render(tpl, handler_name)
}
#[derive(Deserialize)]
pub struct AddForm {
    pub id: Option<i32>,
    pub name: String,
    pub code: String,
    pub unicode: String,
}
async fn do_insert(
    Extension(state): Extension<Arc<DbState>>,
    Form(frm): Form<AddForm>,
) -> Result<HtmlResponse> {
    let handler_name = "Message";
    let mut tpl = crate::view::MsgTemplate::default();
    tpl.title = "添加".to_string();
    if frm.name.eq("") || frm.code.eq("") || frm.unicode.eq("") {
        tpl.is_success = false;
        tpl.msg = String::from("参数不能为空");
        tpl.target_url = Some("/emotion/insert".to_string());
    } else {
        let emo = emotion::EmotionModel {
            id: 0,
            name: frm.name,
            code: frm.code.clone(),
            unicode: frm.unicode.clone(),
        };
        let res = emotion::insert_one(&state, &emo.insert(),&emo.get_table_name().to_string(),&frm.code,&frm.unicode).await;
        match res {
            Ok(_) => {
                tpl.is_success = true;
                tpl.msg = String::from("Ok");
                tpl.target_url = Some("/emotion/list".to_string());
            }
            Err(err) => {
                tpl.is_success = false;
                let _msg = match err.error {
                    crate::err::AppErrorItem::Cause(err) => err.to_string(),
                    crate::err::AppErrorItem::Message(msg) => msg.unwrap_or("发生错误".to_string()),
                };
                tpl.msg = _msg;
                tpl.target_url = Some("/emotion/insert".to_string());
            }
        }
    }
    render(tpl, handler_name)
}

/// 删除
pub async fn do_del(
    Extension(state): Extension<Arc<DbState>>,
    Path(id): Path<i32>,
) -> Result<HtmlResponse>  {
    let emo = emotion::EmotionModel {
        id: 0,
        name: "".to_string(),
        code: "".to_string(),
        unicode: "".to_string(),
    };
    
    let mut tpl = crate::view::MsgTemplate::default();
    #[allow(unused_assignments)]
    let mut message = String::from("OK");
    let msg = emotion::delete(&state, &emo.delete(id)).await;
    match msg {
        Ok(msg) => {
            tpl.is_success=true;
            message = msg;
        },
        Err(e) => {
            let msg = match e.error {
                crate::err::AppErrorItem::Cause(err) => err.to_string(),
                crate::err::AppErrorItem::Message(msg) => msg.unwrap_or("发生错误".to_string()),
            };
            tpl.is_success=false;
            message = msg;
        }
    }
    let handler_name = "Message";
    
    tpl.title="删除".to_string();
    tpl.msg = message;
    
    tpl.target_url = Some("/emotion/list".to_string());

    render(tpl, handler_name)
}


#[derive(Template)]
#[template(path = "emotion/edit.html", ext = "html", escape = "none")]
#[derive(Deserialize)]
pub struct EditForm {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub unicode: String,
}
pub async fn edit(
    Extension(state): Extension<Arc<DbState>>,
    Path(id): Path<i32>,
) -> Result<HtmlResponse> {
    let handler_name = "edit";
    let emo = emotion::EmotionModel {
        id: 0,
        name: "".to_string(),
        code: "".to_string(),
        unicode: "".to_string(),
    };
    let m = emotion::get_one_by_id(&state, &emo.get_one_by_id(id)).await.unwrap_or(emo);
    let tpl = EditForm {
        id: m.id,
        name: m.name.clone(),
        code: m.code,
        unicode: m.unicode,
    };
    render(tpl, handler_name)
}




#[derive(Deserialize)]
pub struct EditStruct {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub unicode: String,
}
pub async fn do_edit(
    Extension(state): Extension<Arc<DbState>>,
    Form(frm): Form<EditStruct>,
) -> Result<HtmlResponse> {
    let emo = emotion::EmotionModel {
        id: frm.id,
        name: frm.name.clone(),
        code: frm.code.clone(),
        unicode: frm.unicode.clone(),
    };
    let result = emotion::update(
        &state,&emo.update(frm.id))
        .await;
    let handler_name = "Message";
    #[allow(unused_assignments)]
    let mut message = String::from("OK");
    let mut tpl = crate::view::MsgTemplate::default();
    match result {
        Ok(msg) => {
            message = msg;
            tpl.is_success = true;
        }
        Err(e) => {
            let _msg = match e.error {
                crate::err::AppErrorItem::Cause(err) => err.to_string(),
                crate::err::AppErrorItem::Message(msg) => msg.unwrap_or("发生错误".to_string()),
            };
            message = _msg;
            tpl.is_success = false;
        }
    }
    tpl.msg = message;
    tpl.target_url = Some("/emotion/list".to_string());

    render(tpl, handler_name)
}


async fn get_emotion(Extension(state): Extension<Arc<DbState>>) -> HandlerJsonResult {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json;charset=utf-8"),
    );
    let emo = emotion::EmotionModel {
        id: 0,
        name: "name".to_string(),
        code: "10086".to_string(),
        unicode: "1F001".to_string(),
    };
    let emo_model = emotion::get_all(&state, &emo.select()).await.unwrap();
    let result = Json(serde_json::json!({ "data": emo_model }));
    let code = StatusCode::OK;
    (code, headers, result)
}