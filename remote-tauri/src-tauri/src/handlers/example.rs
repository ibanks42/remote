use actix_web::{get, HttpResponse, Responder};

#[get("/api/test")]
pub async fn handle() -> impl Responder {
    let text = "hello world";
    println!("{}", text);

    HttpResponse::Ok()
}
