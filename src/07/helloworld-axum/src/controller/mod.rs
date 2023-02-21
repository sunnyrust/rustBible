use crate::{config::WebInfo};
pub mod index;
pub mod tag;
#[allow(dead_code)]
fn get_web_info<'a>(state: &'a WebInfo) -> WebInfo{
   state.to_owned()
}
/// 渲染模板
fn render<T: Template>(tpl: T, handler_name: &str) -> Result<super::vulcan_util::types::HtmlResponse> {
   let out = tpl
       .render()
       .map_err(AppError::from)
       .map_err(log_error(handler_name))?;
   Ok(Html(out))
}