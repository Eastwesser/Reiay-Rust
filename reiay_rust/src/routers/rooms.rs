// Роутер для работы с комнатами.

use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct RoomCreate {
    name: String,
    is_voice_chat: bool,
}

pub fn router() -> Router {
    Router::new().route("/create", post(create_room))
}

async fn create_room(Json(room): Json<RoomCreate>) -> Json<String> {
    // Логика создания комнаты
    Json(format!("Room created: {}", room.name))
}
