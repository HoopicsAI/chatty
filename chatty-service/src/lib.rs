mod model;
mod routes;

use actix_web::{web, App, HttpServer};
use chatty_config::CHATTY_CONFIG;
use colored::Colorize;

pub async fn main() -> std::io::Result<()> {
    println!("{}", "Chatty running in API mode".yellow());

    let host = CHATTY_CONFIG.chatty_service.host.clone();
    let port = CHATTY_CONFIG.chatty_service.port;

    let addr = format!("{}:{}", host, port);
    println!(
        "{} {}",
        "The Chatty service is running on → ".green().bold(),
        addr.green()
    );

    HttpServer::new(move || {
        let scope = web::scope("/api")
            .route(
                "/chat_with_fictonx",
                web::post().to(routes::chat_with_fictionx),
            )
            .route("/chat_with_text", web::post().to(routes::chat_with_text));

        App::new().service(scope)
    })
    .bind((host, port))?
    .run()
    .await
}
