// Модель для хранения сообщений в базе данных.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub content: String,
    pub user_id: i32,
    pub chat_id: i32,
}
