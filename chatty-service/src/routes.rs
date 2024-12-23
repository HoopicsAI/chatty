use crate::model::{ChatMessage, ChatMessageReply};
use actix_web::{post, web, HttpResponse, Responder};
use chatty_gemini::reply;
use colored::Colorize;

#[post("/api/chat")]
pub async fn chat(web::Json(form): web::Json<ChatMessage>) -> actix_web::Result<impl Responder> {
    println!("{}", "Received chat message".green());
    let model_info = match &form.model {
        Some(model) => format!("{}", model.blue()),
        None => "gemini".yellow().to_string(),
    };
    println!(
        "model: {}\nid: {}\nmessage: {}",
        model_info,
        form.character_id.yellow(),
        form.message.cyan()
    );

    let response = reply(&form.message).await;
    let body = ChatMessageReply::new(response);
    println!("Gemini Response: {:#?}", body);

    Ok(HttpResponse::Ok().json(body))
}
