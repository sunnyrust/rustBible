use axum::routing::{get,get_service,};
use crate::controller;
use tower_http::{services::{ServeDir},};
use super::web_handle_error;
pub fn init() -> axum::Router {
    let serve_dir = get_service(ServeDir::new("./static")).handle_error(web_handle_error);
    let css_dir = get_service(ServeDir::new("./static/css")).handle_error(web_handle_error);
    let js_dir = get_service(ServeDir::new("./static/js")).handle_error(web_handle_error);
    let images_dir = get_service(ServeDir::new("./static/images")).handle_error(web_handle_error);
    let models_dir = get_service(ServeDir::new("./models")).handle_error(web_handle_error);
    axum::Router::new()
        .nest_service("/", serve_dir.clone())
        .nest_service("/css", css_dir.clone())
        .nest_service("/js", js_dir.clone())
        .nest_service("/images", images_dir.clone())
        .nest_service("/models", models_dir.clone())
        // .route("/", get(crate::root))
        .route("/do", get(crate::get_fun).post(crate::post_fun))
        .route("/greet", get(|| async { "Hello, axum World!🌱🌎" }))
        .nest("/index", controller::index::index_router())
        .nest("/tag",controller::tag::index_router())
        .nest("/three",controller::three::index_router())
        .nest("/emotion",controller::emotion::index_router())
}