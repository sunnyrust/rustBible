use super::render;
use crate::{dbstate::*, model::tag::*, model::*, util::stack::*, util::types::*, util::*};
use askama::Template;
use axum::{
    extract::Path,
    http::{header::HeaderName, HeaderMap, HeaderValue,  StatusCode},
    routing::{get, post},
    Extension, Form, Json, Router,
};
use serde::Deserialize;
use tower_http::trace::TraceLayer;
pub(crate) fn index_router() -> Router {
    Router::new()
        .route("/demo", get(demo))
        .route("/move", get(walk))
        .layer(TraceLayer::new_for_http())
}

#[derive(Template)]
#[template(path = "three/demo.html")]
pub struct DemoTemplate {}
/// 显示del页面
async fn demo() -> Result<HtmlResponse>{
    let handler_name = "Three";

    let tpl = DemoTemplate {};
    render(tpl, handler_name)
}

#[derive(Template)]
#[template(path = "three/move_female.html")]
pub struct MoveTemplate {}
/// 显示del页面
async fn walk() -> Result<HtmlResponse>{
    let handler_name = "Three";

    let tpl = MoveTemplate {};
    render(tpl, handler_name)
}