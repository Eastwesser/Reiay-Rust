// Файл с основным роутером, который объединяет все остальные роутеры (пользователи, роли, комнаты и т.д.).

use axum::{routing::get, Router};

pub fn api_router() -> Router {
    Router::new()
        .nest("/users", users::router())
        .nest("/roles", roles::router())
        .nest("/rooms", rooms::router())
        .nest("/messages", messages::router())
}
