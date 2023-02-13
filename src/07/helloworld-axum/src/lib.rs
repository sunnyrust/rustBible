pub mod config;
pub mod controller;
pub mod model;
pub mod router;
use axum::{
    http::StatusCode,
    response::IntoResponse,
};
use std::{io};
pub async  fn web_handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
async fn root() -> String {
    String::from("Hello root😀.")
}
async fn get_fun() -> String {
    String::from("get function👋\n")
}
async fn post_fun() -> String {
    String::from("post function🏠\n")
}

