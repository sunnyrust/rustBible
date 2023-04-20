use crate::{config::WebInfo,AppError,Result};
pub mod index;
pub mod tag;
pub mod three;
pub mod emotion;
use axum::{
   // http::{header, HeaderMap, StatusCode},
   response::Html,
};
use askama::Template;
#[allow(dead_code)]
fn get_web_info<'a>(state: &'a WebInfo) -> WebInfo{
   state.to_owned()
}
/// 渲染模板
fn render<T: Template>(tpl: T, handler_name: &str) -> Result<super::util::types::HtmlResponse> {
   let out = tpl
       .render()
       .map_err(AppError::from)
       .map_err(log_error(handler_name))?;
   Ok(Html(out))
}
/// 记录错误
fn log_error(handler_name: &str) -> Box<dyn Fn(AppError) -> AppError> {
   let handler_name = handler_name.to_string();
   Box::new(move |err| {
       tracing::error!("{}: {:?}", handler_name, err);
       err
   })
}