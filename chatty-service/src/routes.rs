use crate::model::{ChatMessage, ChatMessageReply};
use actix_web::{web, HttpResponse, Responder};
use chatty_gemini::reply;
use colored::Colorize;
use chatty_prompt::create_prompt;

pub async fn chat_with_fictionx(body: web::Json<ChatMessage>) -> impl Responder {
    println!("{}", "Received chat message".green());
    let model_info = match &body.model {
        Some(model) => format!("{}", model.blue()),
        None => "gemini".yellow().to_string(),
    };
    println!(
        "model: {}\nid: {}\nmessage: {}",
        model_info,
        body.character_id.yellow(),
        body.message.cyan()
    );

    let prompt = create_prompt!(body.character_name, body.description, body.message);
    let response = reply(&prompt, &body.message).await;
    let body = ChatMessageReply::new(response);
    println!("Gemini Response: {:#?}", body);

    HttpResponse::Ok().json(body)
}

pub async fn chat_with_text(_body: web::Json<ChatMessage>) -> impl Responder {
    HttpResponse::Ok()
}