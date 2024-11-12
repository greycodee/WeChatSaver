use actix_web::{get, App, HttpServer, Responder};
use wechat_saver_lib::wechat::voice_decode;

#[get("/")]
async fn hello() -> impl Responder {
    "hello"
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
    }).bind(("127.0.0.1",9090))?
        .run()
        .await
}
