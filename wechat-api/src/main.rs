use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use wechat_saver_lib::wechat::voice_decode;
use wechat_saver_lib::wechat::get_all_account;

#[get("/")]
async fn hello() -> impl Responder {
    "hello"
}

#[get("/all_account")]
async fn all_account() -> impl Responder {
    let all_account = get_all_account("/Users/zheng/Downloads/20241024_091952");
    // all_account
    HttpResponse::Ok().json(all_account)
}


#[get("/version")]
async fn version() -> impl Responder {
    let decode_version = voice_decode::get_version();
    decode_version.unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // println!("Hello, world!");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(version)
            .service(all_account)
    }).bind(("127.0.0.1",9090))?
        .run()
        .await
}
