mod ascii;
mod routes;
mod model;

use actix_web::{App, HttpServer};

pub async fn main() -> std::io::Result<()> {
    ascii::print();

    HttpServer::new(|| App::new().service(routes::chat))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

