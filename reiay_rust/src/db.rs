// Файл для подключения к базе данных с использованием SQLx.

use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;
use std::env;

pub async fn connect_db() -> sqlx::PgPool {
    // Загрузка .env файла
    dotenv().ok();

    // Получение строки подключения из переменной окружения
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database")
}
