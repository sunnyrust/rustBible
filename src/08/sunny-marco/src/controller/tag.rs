use super::render;
use crate::{dbstate::*, model::tag::*, model::*, util::stack::*, util::types::*, util::*};
use askama::Template;
use axum::{
    extract::Path,
    http::{header::HeaderName, HeaderMap, HeaderValue, StatusCode},
    routing::{get, post},
    Extension, Form, Json, Router,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
pub(crate) fn index_router() -> Router {
    Router::new()
        .route("/list", get(list))
        .route("/tree", get(get_tree))
        .route("/find/:id", get(get_one))
        .route("/edit/:id", get(edit))
        .route("/doedit", post(do_edit))
        .route("/add", get(add).post(do_add))
        .route("/del/:id", get(do_del))
        .layer(TraceLayer::new_for_http())
}

async fn get_one(
    Extension(state): Extension<Arc<DbState>>,
    Path(id): Path<i32>,
) -> types::HandlerJsonResult {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json;charset=utf-8"),
    );

    let tag = tag::get_one_by_id(&state, id).await.unwrap();
    let result = Json(serde_json::json!({ "result": tag }));
    let code = StatusCode::OK;
    (code, headers, result)
}

/// 取得tag树
async fn get_tree(Extension(state): Extension<Arc<DbState>>) -> types::HandlerJsonResult {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json;charset=utf-8"),
    );

    let tag = tag::get_tag_tree(&state).await.unwrap();
    let result = Json(serde_json::json!({ "result": tag }));
    let code = StatusCode::OK;
    (code, headers, result)
}

// #[derive(Template)]
// #[template(path = "tag/delete.html")]
// pub struct TagDeleteTemplate {}
// /// 显示del页面
// async fn del_form() -> crate::Result<HtmlResponse> {
//     let handler_name = "DeleteTagById";

//     let tpl = TagDeleteTemplate {};
//     render(tpl, handler_name)
// }

// #[derive(Debug, Clone, PartialEq, Deserialize, sqlx::FromRow)]
// pub struct DelForm {
//     pub id: i32,
// }
pub async fn do_del(
    Extension(state): Extension<Arc<DbState>>,
    Path(id): Path<i32>,
) -> Result<HtmlResponse> {
    #[allow(unused_assignments)]
    let mut message = String::from("OK");
    let msg = tag::delete(&state, id).await;
    match msg {
        Ok(msg) => message = msg,
        Err(e) => {
            let msg = match e.error {
                crate::err::AppErrorItem::Cause(err) => err.to_string(),
                crate::err::AppErrorItem::Message(msg) => msg.unwrap_or("发生错误".to_string()),
            };
            message = msg;
        }
    }
    let handler_name = "Message";
    let mut tpl = crate::view::MsgTemplate::default();
    tpl.title = "删除".to_string();
    tpl.msg = message;
    tpl.is_success = true;
    tpl.target_url = Some("/tag/list".to_string());

    render(tpl, handler_name)
}

#[derive(Template)]
#[template(path = "tag/list.html", ext = "html", escape = "none")]
pub struct ListForm<'a> {
    pub ul: &'a str,
}

pub async fn list(Extension(state): Extension<Arc<DbState>>) -> Result<HtmlResponse> {
    let tags = tag::get_tag_tree(&state).await.unwrap();
    let handler_name = "list";
    let mut str_html = String::from("");
    let mut stack = Stack::new();
    for tag in tags {
        str_html = set_html(&mut stack, &tag, str_html, true);
    }
    if !stack.is_empty() {
        loop {
            let l_stack = stack.pop().unwrap();
            str_html += r#"</li></ul>"#;
            if l_stack.level == 0 {
                break;
            }
        }
    }
    // tracing::debug!("{str_html}");
    let tpl = ListForm { ul: &str_html };
    render(tpl, handler_name)
}
#[derive(Template)]
#[template(path = "tag/edit.html", ext = "html", escape = "none")]
pub struct EditForm<'a, 'b> {
    pub ul: &'a str,
    pub id: i32,
    pub pid: i32,
    pub name: &'b str,
}
pub async fn edit(
    Extension(state): Extension<Arc<DbState>>,
    Path(id): Path<i32>,
) -> Result<HtmlResponse> {
    let tags = tag::get_tag_tree(&state).await.unwrap();
    //取得pid
    //////////////////////////////////

    let mut node_map: HashMap<i32, TagModel> = HashMap::new();
    for node in tags.clone() {
        node_map.insert(node.id, node);
    }
    let tag_root = node_map.get(&id);
    let tag_root = match tag_root {
        Some(val) => val.to_owned(),
        None => TagModel {
            id: 0,
            name: "Root".to_string(),
            pid: 0,
            level: -9,
            is_parent: true,
        },
    };
    if tag_root.level == -9 {
        let handler_name = "Message";
        let mut tpl = crate::view::MsgTemplate::default();
        tpl.is_success = false;
        tpl.title = "错误".to_string();
        tpl.msg = String::from("数据不存在");
        tpl.target_url = Some("/tag/list".to_string());
        render(tpl, handler_name)
    } else {
        let mut ids = Vec::new();
        tag_root.traverse(&node_map, &mut ids);
        let handler_name = "edit";
        let mut str_html = String::from("");
        let mut stack = Stack::new();
        for tag in tags {
            if !ids.contains(&tag.id) || id == 0 {
                str_html = set_html(&mut stack, &tag, str_html, false);
            }
        }
        if !stack.is_empty() {
            loop {
                let l_stack = stack.pop().unwrap();
                str_html += r#"</li></ul>"#;
                if l_stack.level == 0 {
                    break;
                }
            }
        }
        let m = tag::get_one_by_id(&state, id).await.unwrap_or(Model {
            id: 0,
            name: "".to_string(),
            pid: 0,
        });
        let tpl = EditForm {
            ul: &str_html,
            id: id,
            pid: m.pid,
            name: &m.name,
        };
        render(tpl, handler_name)
    }
}

/// 判断是不是要压栈
fn judge_push(stack: &Stack<TagModel>, tml: &TagModel) -> bool {
    #[allow(unused_assignments)]
    let mut result = false;
    if tml.level == -1 {
        return true;
    }
    let old = stack.get_pop();
    let o = match old {
        Some(val) => val.to_owned(),
        None => TagModel {
            id: 0,
            name: "Root".to_string(),
            pid: 0,
            level: -9,
            is_parent: true,
        },
    };
    if o.level == -9 {
        result = true;
    } else {
        if tml.pid == o.id {
            result = true;
        } else {
            result = false
        }
    }
    result
}

/// 拼接Tree的html字符串
fn set_html(stack: &mut Stack<TagModel>, tml: &TagModel, html: String, have_link: bool) -> String {
    let mut str_html = html;
    #[allow(unused_assignments)]
    let mut s = String::new();
    if judge_push(&stack, &tml) {
        if have_link {
            s = format!(
                "<ul><li id='tree_{}' pid='{}'><a href='./edit/{}'>{}</a>",
                tml.id, tml.pid, tml.id, tml.name
            );
        } else {
            s = format!(
                "<ul><li id='tree_{}' pid='{}'>{}",
                tml.id, tml.pid, tml.name
            );
        }
        str_html += &s;
        stack.push(tml.to_owned());
    } else {
        if !stack.is_empty() {
            loop {
                let l_stack = stack.pop().unwrap();
                str_html += r#"</li>"#;
                if l_stack.pid == tml.pid {
                    break;
                }
                str_html += r#"</ul>"#;
            }
            if have_link {
                s = format!(
                    "<li id='tree_{}' pid='{}'><a href='./edit/{}'>{}</a>",
                    tml.id, tml.pid, tml.id, tml.name
                );
            } else {
                s = format!("<li id='tree_{}' pid='{}'>{}", tml.id, tml.pid, tml.name);
            }
            str_html += &s;
            stack.push(tml.to_owned());
        }
    }

    str_html
}

#[derive(Deserialize)]
pub struct EditStruct {
    pub id: i32,
    pub pid: i32,
    pub name: String,
}

pub async fn do_edit(
    Extension(state): Extension<Arc<DbState>>,
    Form(frm): Form<EditStruct>,
) -> Result<HtmlResponse> {
    let result = tag::update(
        &state,
        Model {
            id: frm.id,
            name: frm.name.clone(),
            pid: frm.pid,
        },
    )
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
    tpl.target_url = Some("/tag/list".to_string());

    render(tpl, handler_name)
}

// method: Method,
// if method.as_str()=="post"{

//     return "Method Not Allowed".to_string();
// }
#[derive(Template)]
#[template(path = "tag/add.html", ext = "html", escape = "none")]
pub struct AddFormTemplate<'a, 'b> {
    pub ul: &'a str,
    pub id: i32,
    pub pid: i32,
    pub name: &'b str,
}
// impl<'a,'b> AddFormTemplate<'a,'b>{
pub async fn add(Extension(state): Extension<Arc<DbState>>) -> Result<HtmlResponse> {
    // if method.as_str() == "post" {
    //     tracing::info!("post…………");
    // }
    let tags = tag::get_tag_tree(&state).await.unwrap();
    let handler_name = "Add";
    let mut str_html = String::from("");
    let mut stack = Stack::new();
    for tag in tags {
        str_html = set_html(&mut stack, &tag, str_html, false);
    }
    if !stack.is_empty() {
        loop {
            let l_stack = stack.pop().unwrap();
            str_html += r#"</li></ul>"#;
            if l_stack.level == 0 {
                break;
            }
        }
    }
    let tpl = AddFormTemplate {
        ul: &str_html,
        id: -1,
        pid: 0,
        name: "",
    };
    render(tpl, handler_name)
}
// }
#[derive(Deserialize)]
pub struct AddStruct {
    pub id: i32,
    pub pid: i32,
    pub name: String,
}
pub async fn do_add(
    Extension(state): Extension<Arc<DbState>>,
    Form(frm): Form<AddStruct>,
) -> Result<HtmlResponse> {
    let result = tag::insert_one(
        &state,
        Model {
            id: frm.id,
            name: frm.name.clone(),
            pid: frm.pid,
        },
    )
    .await;
    let handler_name = "Message";
    #[allow(unused_assignments)]
    let mut message = String::from("OK");
    let mut tpl = crate::view::MsgTemplate::default();
    tpl.title = "添加".to_string();
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
    tpl.target_url = Some("/tag/list".to_string());

    render(tpl, handler_name)
}
