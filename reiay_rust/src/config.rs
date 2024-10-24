// Файл для конфигурации приложения (например, загрузка переменных окружения).

use dotenv::dotenv;
use std::env;

pub fn load_config() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database: {}", database_url);
}
