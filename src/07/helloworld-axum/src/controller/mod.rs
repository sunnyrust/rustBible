use crate::{config::WebInfo};
pub mod index;
#[allow(dead_code)]
fn get_web_info<'a>(state: &'a WebInfo) -> WebInfo{
   state.to_owned()
}