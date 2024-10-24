// Основной файл приложения, где инициализируются роутеры, мидлвари и сервер.

use axum::{routing::get, Router};
use std::net::SocketAddr;

mod db;
mod config;
mod i18n;
mod routers;
mod middleware;

use routers::api_router;
use middleware::locale_middleware::locale_middleware;

#[tokio::main]
async fn main() {
    i18n::init_fluent();  // Инициализация переводов

    let app = Router::new()
        .nest("/api/reiay", api_router())
        .layer(axum::middleware::from_fn(locale_middleware));  // Мидлварь для выбора языка

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Запуск приложения на {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
