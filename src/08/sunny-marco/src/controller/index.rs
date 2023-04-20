use axum::{extract::Path,Extension,Router,routing::{get} };
use crate::{config::WebInfo};
use std::sync::Arc;
use tower_http::{trace::TraceLayer};
use super::{get_web_info,};
/// 把当前controller里面需要暴露的请求，写成函数
pub(crate) fn index_router() -> Router {
    Router::new()
    .route("/greet", get(greet))
    .route("/info", get(getwebinfo))
    .route("/ping", get(ping))
    .layer(TraceLayer::new_for_http())

}
pub async fn greet(
    Path(name): Path<String>,
)-> String{
    format!("Hello {name}")
}
pub async fn getwebinfo(
    Extension(info): Extension<Arc<WebInfo>>,
)-> String{
    let info=get_web_info(&info);
    format!("web version {}",info.web_version)
}
pub async fn ping()-> String{
    String::from("Axum ping 🦸.")
}

