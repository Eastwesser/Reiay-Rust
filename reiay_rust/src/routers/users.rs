// Роутер для работы с пользователями.

use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct UserCreate {
    username: String,
    password: String,
}

pub fn router() -> Router {
    Router::new().route("/register", post(register_user))
}

async fn register_user(Json(user): Json<UserCreate>) -> Json<String> {
    // Логика создания пользователя
    Json(format!("User registered: {}", user.username))
}
