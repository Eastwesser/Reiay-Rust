// Роутер для работы с сообщениями.

use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MessageCreate {
    content: String,
    user_id: i32,
    chat_id: i32,
}

pub fn router() -> Router {
    Router::new().route("/create", post(create_message))
}

async fn create_message(Json(message): Json<MessageCreate>) -> Json<String> {
    // Логика сохранения сообщения
    Json(format!("Message created: {}", message.content))
}
