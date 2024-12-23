use actix_web::{post, web, HttpResponse, Responder};
use crate::model::ChatMessage;

#[post("/api/chat")]
pub async fn chat(
    web::Json(form): web::Json<ChatMessage>,
) -> actix_web::Result<impl Responder> {
    println!("Received chat message>>> id: {} and message : {}", form.character_id, form.message);

    Ok(HttpResponse::Ok().json(format!("data cached until")))
}