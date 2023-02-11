use axum::routing::{get};
use crate::controller;

pub fn init() -> axum::Router {
    
    axum::Router::new()
        .route("/", get(crate::root))
        .route("/do", get(crate::get_fun).post(crate::post_fun))
        .route("/greet", get(|| async { "Hello, axum World!🌱🌎" }))
        .nest("/index", controller::index::index_router())
}