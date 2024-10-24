// Роутер для работы с ролями.

use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct RoleCreate {
    name: String,
}

pub fn router() -> Router {
    Router::new().route("/create", post(create_role))
}

async fn create_role(Json(role): Json<RoleCreate>) -> Json<String> {
    // Логика создания роли
    Json(format!("Role created: {}", role.name))
}
