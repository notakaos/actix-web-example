extern crate actix_web_example;

use actix_web_example::{hello, echo, manual_hello};

use actix_web::{App, web, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:7878")?
    .run()
    .await
}

