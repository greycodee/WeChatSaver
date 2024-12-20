mod databases;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::path::Path;
// use wechat_saver_lib::wechat::get_all_account;
// use wechat_saver_lib::wechat::voice_decode;

#[get("/")]
async fn hello() -> impl Responder {
    "hello"
}

// #[get("/all_account")]
// async fn all_account() -> impl Responder {
//     let base_path = Path::new("/Users/zheng/Downloads/20241024_091952");
//     let all_account = get_all_account(base_path).unwrap();
//     // all_account
//     HttpResponse::Ok().json(all_account)
// }

// #[get("/version")]
// async fn version() -> impl Responder {
//     let decode_version = voice_decode::get_version();
//     decode_version.unwrap()
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO 初始化数据库连接
    // println!("Hello, world!");
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await
}
