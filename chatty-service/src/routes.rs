use crate::model::{ChatMessage, ChatMessageReply};
use actix_web::{web, HttpResponse, Responder};
use chatty_prompt::create_prompt;
use colored::Colorize;

pub async fn chat(body: web::Json<ChatMessage>) -> impl Responder {
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
    let response = chatty_gemini::agent_api::APIAgent::new()
        .chat(&prompt, &body.message)
        .await;
    let body = ChatMessageReply::new(response);
    HttpResponse::Ok().json(body)
}
