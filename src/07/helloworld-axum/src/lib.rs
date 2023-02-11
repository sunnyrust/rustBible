pub mod config;
pub mod controller;
pub mod model;
pub mod router;

async fn root() -> String {
    String::from("Hello root😀.")
}
async fn get_fun() -> String {
    String::from("get function👋\n")
}
async fn post_fun() -> String {
    String::from("post function🏠\n")
}