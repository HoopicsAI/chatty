mod ascii;
mod model;
mod routes;

use actix_web::{App, HttpServer};
use chatty_config::CHATTY_CONFIG;
use colored::Colorize;

pub async fn main() -> std::io::Result<()> {
    ascii::print();

    let host = CHATTY_CONFIG.chatty_service.host.clone();
    let port = CHATTY_CONFIG.chatty_service.port;

    let addr = format!("{}:{}", host, port);
    println!(
        "{} {}",
        "The Chatty service is running on → ".green(),
        addr.green()
    );

    HttpServer::new(|| App::new().service(routes::chat))
        .bind((host, port))?
        .run()
        .await
}
