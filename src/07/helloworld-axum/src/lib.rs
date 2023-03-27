pub mod config;
pub mod controller;
pub mod view;
pub mod model;
pub mod router;
pub mod dbstate;
pub mod err;
pub mod util;
use axum::{
    http::StatusCode,
    response::IntoResponse,
};
use std::{io};
pub async  fn web_handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
#[allow(dead_code)]
async fn root() -> String {
    String::from("Hello root😀.")
}
async fn get_fun() -> String {
    String::from("get function👋\n")
}
async fn post_fun() -> String {
    String::from("post function🏠\n")
}
pub use err::{AppError, AppErrorType};
pub type Result<T> = std::result::Result<T, crate::AppError>;
