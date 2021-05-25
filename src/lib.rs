use actix_web::{get, post, HttpResponse, Responder};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!\n")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(format!("{}\n", req_body))
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey, there!\n")
}