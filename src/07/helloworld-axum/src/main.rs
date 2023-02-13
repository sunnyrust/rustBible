use axum::{
    Extension,
};
use tower_http::{trace::TraceLayer};
use std::sync::Arc;
use dotenv::dotenv;
use helloworld_axum::{config,router};

#[tokio::main]
async fn main() {
    eprintln!(
r#"
╔══╗
╚╗╔╝
╔╝(¯`v´¯)
╚══`.¸.[🅰 🆇 🆄 🅼 🌐🌱]
"#);
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "helloworld_axum=debug");
    }
    dotenv().ok();
    tracing_subscriber::fmt::init();
    // let cfg = config::Config::from_env().unwrap();
    let cfg=config::Config::from_file("./app.toml").unwrap();
    let web_info=config::WebInfo{
        web_addr:cfg.web.addr.clone(),
        web_version:cfg.web.version.clone(),
    };
    // 建立一个简单的路由
    let app =  router::init()
            .layer(TraceLayer::new_for_http())
            .layer(Extension(Arc::new(web_info))) ;
    
    tracing::info!("🌱🌎 服务监听于{}🌐🌱", &cfg.web.addr);
    // 起一个http服务，端口依靠读取.env文件获得
    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}