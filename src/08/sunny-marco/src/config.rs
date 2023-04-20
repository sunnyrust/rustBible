use serde::Deserialize;
#[derive(Deserialize)]
pub struct WebConfig {
    pub addr: String,
    pub version: String,
}
#[derive(Deserialize)]
pub struct DbConfig {
    pub pg:String,
    pub connections:u32,
}
#[derive(Deserialize)]
pub struct RedisConfig {
    pub url:String,
}
#[derive(Deserialize)]
pub struct Config {
    pub web: WebConfig,
    pub db: DbConfig,
    pub redis:RedisConfig,
}
impl Config {
    /// 从环境读取
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize()
    }
    /// 从文件读取
    pub fn from_file(path: &'static str) -> Result<Self, config::ConfigError> {
        config::Config::builder()
        .add_source(config::File::with_name(path))
        .add_source(config::Environment::default())
        .build()?
        .try_deserialize()
    }
}
#[derive(Deserialize,Clone,Debug)]
pub struct WebInfo{
    pub web_addr:String,
    pub web_version:String,
}