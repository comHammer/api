use std::fmt::format;

use actix_web::{get, post, App, HttpRequest, HttpServer, web, error, HttpResponse, Responder};
use serde::Deserialize;

// 在这里指明路径参数，并使用元组接收，同时确定类型

#[derive(Debug, Deserialize)]
struct User {
    id: i32,
    name: String,
    operate: String,
    object: String,
    number: i128,
}

// 使用json解析
#[post("/t1")]
async fn post1(user: web::Json<User>) -> impl Responder {
    let message = user.into_inner();
    println!("{:?}", message.id);
    "ok".to_string()
}

// 使用json解析
#[post("/t2")]
async fn post2(user: web::Form<User>) -> String {
    let message = user.into_inner();
    println!("{:?}", message.id);
    if message.operate == "register" {
        register(message);
    }




    "ok".to_string()
}

async fn register(message: User){
    let mut str = format!("INSERT INTO users (username,phone_number) VALUES ({},{});",message.name,message.number);
    println!("{:?}", str);
    
}
/// 请求连接：
/// get@/t1/123/aaa
/// get@/t2/123/aaa
/// get@/t3/123/aaa
/// get@/t4?id=123&name=aaa
/// post@/t1 {"id": 123,"name": "aaa"}
/// post@/t2 id: 123, name: "aaa"
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 创建一个json解析配置，并用于处理json解析
    let json_config = web::JsonConfig::default()
        .limit(4096)
        .error_handler(|err, _req| {
            error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                .into()
        });
    HttpServer::new(move || {
        App::new()
            .app_data(json_config.clone())
            .service(post1)
            .service(post2)
    })
        .bind("127.0.0.1:8190")?
        .run()
        .await
}
