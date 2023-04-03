use askama::Template;

#[derive(Template)]
#[template(path = "common/msg.html")]
pub struct MsgTemplate {
    pub is_success: bool,  
    pub msg: String,
    pub target_url: Option<String>,
    pub wait:i32,
    pub title:String,
}

impl MsgTemplate {
    fn new(is_success: bool, msg: String, target_url: Option<String>,wait:i32,title:String) -> Self {
        Self {
            is_success,
            msg,
            target_url,
            wait,  //默认等待5秒
            title
        }
    }
    pub fn ok(msg: &str, target_url: &str,title:&str) -> Self {
        Self::new(true, msg.to_string(), Some(target_url.to_string()),5,title.to_string())
    }
    pub fn err(msg: &str,title:&str) -> Self {
        Self::new(false, msg.to_string(), None,5,title.to_string())
    }
    pub fn target_url(&self) -> String {
        match self.target_url.clone() {
            Some(target_url) => target_url,
            None => format!(""),
        }
    }
}

impl Default for MsgTemplate {
    fn default() -> Self {
        Self::new(false, String::from(""), None,5,String::from("消息"))
    }
}