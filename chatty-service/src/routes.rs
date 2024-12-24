use crate::model::{ChatMessage, ChatMessageReply};
use actix_web::{web, HttpResponse, Responder};
use chatty_gemini::reply;
use colored::Colorize;

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

    let response = reply(&body.message).await;
    let body = ChatMessageReply::new(response);
    println!("Gemini Response: {:#?}", body);

    HttpResponse::Ok().json(body)
}

pub async fn chat_with_text(web::Json(form): web::Json<ChatMessage>) -> impl Responder {
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

    HttpResponse::Ok().json(body)
}