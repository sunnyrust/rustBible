#[allow(unused_imports)]
use serde_json::json;
#[derive(Debug)]
struct SunnyMsgStruct {
    status:bool, 
    msg:String,
    data:serde_json::Value
 }
fn main() {
    let data = r#"{
            "name": "Sunny",
            "age": 46,
            "email": [
                "hello@sunny.com",
                "great@sunny.com"
            ]
        }"#;
    let sms=SunnyMsgStruct{status:true,msg:"OK".to_string(),data:serde_json::from_str(data).unwrap()};
    println!("{:#?}",sms);

    let sms=SunnyMsgStruct{status:false,msg:"Error".to_string(),data:serde_json::Value::Null};
    println!("{:#?}",sms);
}
